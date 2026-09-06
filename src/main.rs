mod token;
mod xbox;

use std::{
    io::{self, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use reqwest::Client;
use token::{XboxToken, load_tokens, locate_token_file, remove_tokens_atomically, token_count};
use tokio::sync::Semaphore;
use xbox::{ApiError, XboxApiConfig, XboxClient};

fn prompt(message: &str) -> Result<String, io::Error> {
    print!("{message}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token_path = locate_token_file(&PathBuf::from("."));
    let raw_tokens = load_tokens(&token_path)?;
    if raw_tokens.is_empty() {
        return Err("tokens.txt contains no non-empty lines".into());
    }
    println!("Loaded {} token(s).", raw_tokens.len());
    let requested = prompt(&format!(
        "How many tokens to use? (1-{}, Enter for all): ",
        raw_tokens.len()
    ))?;
    let count = token_count(&requested, raw_tokens.len());
    let target = prompt("Target gamertag: ")?;
    if target.is_empty() {
        return Err("target gamertag cannot be empty".into());
    }
    let tokens: Vec<(String, XboxToken)> = raw_tokens
        .into_iter()
        .take(count)
        .filter_map(|raw| XboxToken::parse(&raw).map(|token| (token.source().to_owned(), token)))
        .collect();
    if tokens.is_empty() {
        return Err("no valid XBL3.0 or Microsoft JWE tokens found".into());
    }

    let config = XboxApiConfig::default();
    let client = XboxClient::new(
        Client::builder()
            .user_agent("xbox-follower-bot/0.1")
            .build()?,
        config.clone(),
    );
    let semaphore = Arc::new(Semaphore::new(config.concurrency));
    let successful = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicUsize::new(0));
    let mut tasks = Vec::with_capacity(tokens.len());
    for (raw, token) in tokens {
        let permit = Arc::clone(&semaphore).acquire_owned().await?;
        let client = client.clone();
        let target = target.clone();
        let successful = Arc::clone(&successful);
        let failed = Arc::clone(&failed);
        tasks.push(tokio::spawn(async move {
            let result = client.follow(&token, &target).await;
            drop(permit);
            match &result {
                Ok(()) => {
                    successful.fetch_add(1, Ordering::Relaxed);
                    println!("success: one follow confirmed");
                }
                Err(error) => {
                    failed.fetch_add(1, Ordering::Relaxed);
                    eprintln!("failed: {error}");
                }
            }
            (raw, result)
        }));
    }
    let mut permanently_invalid = Vec::new();
    for task in tasks {
        let (raw, result) = task.await?;
        if matches!(result, Err(ApiError::PermanentAuth(_))) {
            permanently_invalid.push(raw);
        }
    }
    if !permanently_invalid.is_empty() {
        remove_tokens_atomically(&token_path, &permanently_invalid)?;
        println!(
            "Removed {} permanently invalid token(s).",
            permanently_invalid.len()
        );
    }
    let ok = successful.load(Ordering::Relaxed);
    let errors = failed.load(Ordering::Relaxed);
    println!(
        "Finished: {ok} successful, {errors} failed, {} processed.",
        ok + errors
    );
    Ok(())
}
