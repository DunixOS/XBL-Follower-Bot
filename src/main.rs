use std::{collections::HashMap, io, time::Duration};

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

extern crate reqwest;

pub fn load_tokens() -> Vec<String> {
    let file_loc = std::env::var("PWD").expect("somehow, PWD isn't set.");

    match std::fs::read_to_string((file_loc.clone() + "/tokens.txt").as_str()) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("[ERROR]: {} was not found!", file_loc.clone());
            vec![]
        }
        Err(e) => panic!("{}", e),
        Ok(s) => return s.lines().map(String::from).collect(),
    }
}

pub async fn get_xuid(gamertag: String, token: String) -> Option<String> {
    let mut headers_input: HashMap<String, String> = HashMap::new();
    let token = token.replace("XBL3.0 x=", "");
    headers_input.insert("Authorization".into(), format!("XBL3.0 x={token}"));
    headers_input.insert("x-xbl-contract-version".into(), "2".into());
    headers_input.insert("Accept-Language".into(), "en-US".into());

    let mut headers = HeaderMap::new();
    for (k, v) in headers_input {
        let name = HeaderName::from_bytes(k.as_bytes()).ok()?;
        let value = HeaderValue::from_str(&v).ok()?;
        headers.insert(name, value);
    }

    let client = Client::new();

    let res = client
        .get(format!(
            "https://profile.xboxlive.com/users/gt({gamertag})/profile/settings"
        ))
        .headers(headers)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .ok();

    println!("Status={}", &res?.status());
    Some("".into())
}

#[tokio::main]
async fn main() {
    let tokens = load_tokens();
    for token in &tokens {
        println!("{}", token)
    }

    get_xuid("goll".into(), tokens[0].clone()).await;
}
