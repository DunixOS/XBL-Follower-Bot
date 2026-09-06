use std::{sync::Arc, time::Duration};

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::{
    Client, StatusCode,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::token::XboxToken;

#[derive(Clone, Debug)]
pub struct XboxApiConfig {
    pub social_base: String,
    pub user_auth_url: String,
    pub xsts_url: String,
    pub concurrency: usize,
    pub max_retries: usize,
}

impl Default for XboxApiConfig {
    fn default() -> Self {
        Self {
            social_base: "https://social.xboxlive.com".into(),
            user_auth_url: "https://user.auth.xboxlive.com/user/authenticate".into(),
            xsts_url: "https://xsts.auth.xboxlive.com/xsts/authorize".into(),
            concurrency: 4,
            max_retries: 2,
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Xbox returned HTTP {status}: {body}")]
    Http { status: StatusCode, body: String },
    #[error("token is permanently invalid (XErr {0})")]
    PermanentAuth(i64),
    #[error("XSTS response is missing DisplayClaims.xui[0].uhs or Token")]
    MalformedXsts,
    #[error("Microsoft token exchange failed: HTTP {status}: {body}")]
    Authentication { status: StatusCode, body: String },
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct XstsResponse {
    #[serde(rename = "DisplayClaims")]
    display_claims: Option<DisplayClaims>,
    #[serde(rename = "Token")]
    token: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DisplayClaims {
    xui: Option<Vec<Xui>>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Xui {
    uhs: Option<String>,
}
#[derive(Debug, Deserialize)]
struct XstsError {
    #[serde(rename = "XErr")]
    xerr: Option<i64>,
}

#[derive(Debug, Serialize)]
struct UserAuthRequest<'a> {
    #[serde(rename = "Properties")]
    properties: UserAuthProperties<'a>,
    #[serde(rename = "RelyingParty")]
    relying_party: &'static str,
    #[serde(rename = "TokenType")]
    token_type: &'static str,
}

#[derive(Debug, Serialize)]
struct UserAuthProperties<'a> {
    #[serde(rename = "AuthMethod")]
    auth_method: &'static str,
    #[serde(rename = "SiteName")]
    site_name: &'static str,
    #[serde(rename = "RpsTicket")]
    rps_ticket: String,
    #[serde(skip)]
    marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Debug, Serialize)]
struct XstsRequest<'a> {
    #[serde(rename = "Properties")]
    properties: XstsProperties<'a>,
    #[serde(rename = "RelyingParty")]
    relying_party: &'static str,
    #[serde(rename = "TokenType")]
    token_type: &'static str,
}

#[derive(Debug, Serialize)]
struct XstsProperties<'a> {
    #[serde(rename = "SandboxId")]
    sandbox_id: &'static str,
    #[serde(rename = "UserTokens")]
    user_tokens: Vec<String>,
    #[serde(skip)]
    marker: std::marker::PhantomData<&'a ()>,
}

#[allow(dead_code)]
pub fn parse_xsts_response(body: &str) -> Result<String, ApiError> {
    let (uhs, token) = parse_token_response(body)?;
    Ok(format!("XBL3.0 x={uhs};{token}"))
}

fn parse_token_response(body: &str) -> Result<(String, String), ApiError> {
    let response: XstsResponse = serde_json::from_str(body).map_err(|_| ApiError::MalformedXsts)?;
    let uhs = response
        .display_claims
        .and_then(|claims| claims.xui)
        .and_then(|xui| xui.into_iter().next())
        .and_then(|entry| entry.uhs);
    match (uhs, response.token) {
        (Some(uhs), Some(token)) if !uhs.is_empty() && !token.is_empty() => Ok((uhs, token)),
        _ => Err(ApiError::MalformedXsts),
    }
}

pub fn is_permanent_auth_failure(status: StatusCode, body: &str) -> bool {
    if status != StatusCode::UNAUTHORIZED {
        return false;
    }
    serde_json::from_str::<XstsError>(body)
        .ok()
        .and_then(|error| error.xerr)
        .is_some_and(|code| (2_148_916_233..=2_148_916_235).contains(&code))
}

pub fn follow_url(base: &str, gamertag: &str) -> String {
    format!(
        "{}/users/me/people/gt({})",
        base.trim_end_matches('/'),
        utf8_percent_encode(gamertag.trim(), NON_ALPHANUMERIC)
    )
}

pub fn response_counts_as_success(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED | StatusCode::NO_CONTENT
    )
}

pub fn is_retryable(status: Option<StatusCode>) -> bool {
    status.is_none_or(|status| {
        status == StatusCode::REQUEST_TIMEOUT
            || status == StatusCode::TOO_MANY_REQUESTS
            || status.is_server_error()
    })
}

#[derive(Clone)]
pub struct XboxClient {
    client: Client,
    config: Arc<XboxApiConfig>,
}

impl XboxClient {
    pub fn new(client: Client, config: XboxApiConfig) -> Self {
        Self {
            client,
            config: Arc::new(config),
        }
    }

    async fn authenticate(&self, token: &XboxToken) -> Result<String, ApiError> {
        if let Some(header) = token.xbl_header() {
            return Ok(header.to_owned());
        }
        let microsoft_token = token.microsoft_token().ok_or(ApiError::MalformedXsts)?;
        let user_request = UserAuthRequest {
            properties: UserAuthProperties {
                auth_method: "RPS",
                site_name: "user.auth.xboxlive.com",
                rps_ticket: format!("d={microsoft_token}"),
                marker: std::marker::PhantomData,
            },
            relying_party: "http://auth.xboxlive.com",
            token_type: "JWT",
        };
        let response = self
            .client
            .post(&self.config.user_auth_url)
            .json(&user_request)
            .send()
            .await?;
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable response>".into());
        if !status.is_success() {
            if is_permanent_auth_failure(status, &body) {
                let code = serde_json::from_str::<XstsError>(&body)
                    .ok()
                    .and_then(|error| error.xerr)
                    .unwrap_or_default();
                return Err(ApiError::PermanentAuth(code));
            }
            return Err(ApiError::Authentication { status, body });
        }
        let (_, user_token) = parse_token_response(&body)?;
        let xsts_request = XstsRequest {
            properties: XstsProperties {
                sandbox_id: "RETAIL",
                user_tokens: vec![user_token],
                marker: std::marker::PhantomData,
            },
            relying_party: "http://xboxlive.com",
            token_type: "JWT",
        };
        let response = self
            .client
            .post(&self.config.xsts_url)
            .json(&xsts_request)
            .send()
            .await?;
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable response>".into());
        if !status.is_success() {
            if is_permanent_auth_failure(status, &body) {
                let code = serde_json::from_str::<XstsError>(&body)
                    .ok()
                    .and_then(|error| error.xerr)
                    .unwrap_or_default();
                return Err(ApiError::PermanentAuth(code));
            }
            return Err(ApiError::Authentication { status, body });
        }
        parse_xsts_response(&body)
    }

    pub async fn follow(&self, token: &XboxToken, gamertag: &str) -> Result<(), ApiError> {
        let authorization = self.authenticate(token).await?;
        let mut delay = Duration::from_millis(500);
        for attempt in 0..=self.config.max_retries {
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&authorization).map_err(|_| ApiError::MalformedXsts)?,
            );
            headers.insert("X-XBL-Contract-Version", HeaderValue::from_static("2"));
            let response = self
                .client
                .put(follow_url(&self.config.social_base, gamertag))
                .headers(headers)
                .send()
                .await;
            match response {
                Ok(response) if response_counts_as_success(response.status()) => return Ok(()),
                Ok(response) => {
                    let status = response.status();
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|value| value.to_str().ok())
                        .and_then(|value| value.parse::<u64>().ok());
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "<unreadable response>".into());
                    if is_permanent_auth_failure(status, &body) {
                        return Err(ApiError::PermanentAuth(
                            serde_json::from_str::<XstsError>(&body)
                                .ok()
                                .and_then(|e| e.xerr)
                                .unwrap_or_default(),
                        ));
                    }
                    if !is_retryable(Some(status)) || attempt == self.config.max_retries {
                        return Err(ApiError::Http { status, body });
                    }
                    if let Some(retry_after) = retry_after {
                        tokio::time::sleep(Duration::from_secs(retry_after.min(60))).await;
                        continue;
                    }
                }
                Err(error) => {
                    if attempt == self.config.max_retries {
                        return Err(ApiError::Request(error));
                    }
                }
            }
            tokio::time::sleep(delay).await;
            delay = (delay * 2).min(Duration::from_secs(8));
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_xsts_success() {
        assert_eq!(
            parse_xsts_response(r#"{"DisplayClaims":{"xui":[{"uhs":"123"}]},"Token":"abc"}"#)
                .unwrap(),
            "XBL3.0 x=123;abc"
        );
    }
    #[test]
    fn rejects_malformed_xsts() {
        assert!(parse_xsts_response(r#"{"Token":"abc"}"#).is_err());
    }
    #[test]
    fn encodes_target() {
        assert_eq!(
            follow_url("https://example.test", "A B/т"),
            "https://example.test/users/me/people/gt(A%20B%2F%D1%82)"
        );
    }
    #[test]
    fn classifies_responses() {
        assert!(response_counts_as_success(StatusCode::NO_CONTENT));
        assert!(!response_counts_as_success(StatusCode::BAD_REQUEST));
        assert!(is_retryable(Some(StatusCode::TOO_MANY_REQUESTS)));
    }
    #[test]
    fn recognizes_only_known_permanent_auth_errors() {
        assert!(is_permanent_auth_failure(
            StatusCode::UNAUTHORIZED,
            r#"{"XErr":2148916233}"#
        ));
        assert!(!is_permanent_auth_failure(
            StatusCode::UNAUTHORIZED,
            r#"{"XErr":123}"#
        ));
        assert!(!is_permanent_auth_failure(
            StatusCode::FORBIDDEN,
            r#"{"XErr":2148916233}"#
        ));
    }
}
