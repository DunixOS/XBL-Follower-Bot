use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("could not read {path}: {source}")]
    Read { path: PathBuf, source: io::Error },
    #[error("could not atomically rewrite {path}: {source}")]
    Write { path: PathBuf, source: io::Error },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XboxToken {
    Xbl(String),
    MicrosoftJwe(String),
}

pub fn locate_token_file(root: &Path) -> PathBuf {
    [
        "tokens.env",
        ".env",
        "tokens",
        "tokens.txt",
        "python/tokens.txt",
    ]
    .into_iter()
    .map(|name| root.join(name))
    .find(|path| path.is_file())
    .unwrap_or_else(|| root.join("tokens.txt"))
}

impl XboxToken {
    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        if let Some(credential) = value.strip_prefix("XBL3.0 x=") {
            let (uhs, token) = credential.split_once(';')?;
            if uhs.is_empty() || token.is_empty() || credential.contains('\n') {
                return None;
            }
            return Some(Self::Xbl(format!("XBL3.0 x={uhs};{token}")));
        }
        if !value.contains('.') {
            let (uhs, token) = value.split_once(';')?;
            if uhs.is_empty() || token.is_empty() {
                return None;
            }
            return Some(Self::Xbl(format!("XBL3.0 x={uhs};{token}")));
        }
        let segments: Vec<&str> = value.split('.').collect();
        if segments.len() != 5 || segments.iter().any(|segment| segment.is_empty()) {
            return None;
        }
        let header = URL_SAFE_NO_PAD
            .decode(segments[0])
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())?;
        (header.get("alg")?.as_str()? == "RSA-OAEP"
            && header.get("enc")?.as_str()? == "A128CBC-HS256")
            .then(|| Self::MicrosoftJwe(value.to_owned()))
    }

    pub fn source(&self) -> &str {
        match self {
            Self::Xbl(value) | Self::MicrosoftJwe(value) => value,
        }
    }

    pub fn xbl_header(&self) -> Option<&str> {
        match self {
            Self::Xbl(value) => Some(value),
            Self::MicrosoftJwe(_) => None,
        }
    }

    pub fn microsoft_token(&self) -> Option<&str> {
        match self {
            Self::Xbl(_) => None,
            Self::MicrosoftJwe(value) => Some(value),
        }
    }
}

pub fn load_tokens(path: &Path) -> Result<Vec<String>, TokenError> {
    let contents = fs::read_to_string(path).map_err(|source| TokenError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(contents
        .lines()
        .filter_map(|line| {
            let value = line.trim();
            (!value.is_empty() && !value.starts_with('#')).then(|| value.to_owned())
        })
        .collect())
}

pub fn remove_tokens_atomically(path: &Path, removed: &[String]) -> Result<(), TokenError> {
    if removed.is_empty() {
        return Ok(());
    }
    let removed: HashSet<&str> = removed.iter().map(String::as_str).collect();
    let current = load_tokens(path)?;
    let kept = current
        .into_iter()
        .filter(|token| !removed.contains(token.as_str()))
        .collect::<Vec<_>>();
    let temporary = path.with_file_name(".tokens.txt.tmp");
    let contents = if kept.is_empty() {
        String::new()
    } else {
        format!("{}\n", kept.join("\n"))
    };
    fs::write(&temporary, contents)
        .and_then(|_| fs::rename(&temporary, path))
        .map_err(|source| TokenError::Write {
            path: path.to_path_buf(),
            source,
        })
}

pub fn token_count(input: &str, total: usize) -> usize {
    if input.trim().is_empty() {
        return total;
    }
    input
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|count| *count > 0)
        .map_or(total, |count| count.min(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_normalizes_xbl3_credentials() {
        assert_eq!(
            XboxToken::parse(" uhs;token ").unwrap().source(),
            "XBL3.0 x=uhs;token"
        );
        assert!(XboxToken::parse("not-a-token").is_none());
    }

    #[test]
    fn accepts_the_microsoft_compact_jwe_shape() {
        let header = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkExMjhDQkMtSFMyNTYifQ";
        let value = format!("{header}.a.b.c.d");
        assert!(matches!(
            XboxToken::parse(&value),
            Some(XboxToken::MicrosoftJwe(_))
        ));
    }

    #[test]
    fn applies_token_limit_without_rejecting_interactive_input() {
        assert_eq!(token_count("2", 5), 2);
        assert_eq!(token_count("", 5), 5);
        assert_eq!(token_count("0", 5), 5);
        assert_eq!(token_count("bad", 5), 5);
        assert_eq!(token_count("99", 5), 5);
    }

    #[test]
    fn locates_supported_token_file_names_in_order() {
        let root =
            std::env::temp_dir().join(format!("xbox-follower-path-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("tokens.txt"), "token").unwrap();
        assert_eq!(locate_token_file(&root), root.join("tokens.txt"));
        fs::write(root.join(".env"), "token").unwrap();
        assert_eq!(locate_token_file(&root), root.join(".env"));
        fs::write(root.join("tokens.env"), "token").unwrap();
        assert_eq!(locate_token_file(&root), root.join("tokens.env"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn removes_only_marked_tokens() {
        let path =
            std::env::temp_dir().join(format!("xbox-follower-test-{}.txt", std::process::id()));
        fs::write(&path, "keep\nremove\n").unwrap();
        remove_tokens_atomically(&path, &["remove".to_owned()]).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "keep\n");
        fs::remove_file(path).unwrap();
    }
}
