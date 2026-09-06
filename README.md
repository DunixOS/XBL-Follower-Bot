# Xbox Live Follower Bot

A tool for managing Xbox Live follows using XSTS authentication tokens.

![](https://rv.playfairs.cc/DunixOS/XBL-Follower-Bot)

---

>[!IMPORTANT]
> **READ EVERYTHING BELOW IF YOU PLAN ON ADDING ME**
>
>- **I:**   If you need help with this tool and how to get it working,
>           you can DM [@playfairs](<https://discord.com/users/1426711359059394662>) or join the [Support Server](https://discord.gg/FuvkW9Hg7y) on Discord.
>- **II:**  If you specifically need help obtaining **Xbox Tokens**, I can **NOT** help you with this,
>           I was working on a tool for a while, but it became outdated after short time,
>           so if you add me to ask for tokens, I can and will **NOT** help or provide tokens
>           as I both cannot nor have interest in creating / obtaining them myself.
>- **III:** Assuming you want to DM me on Discord, before you DM me about ANY other tools, please read the **README** displayed at the [Home of this Organization](https://github.com/DunixOS).

>[!NOTE]
>- **I:**   In order for this tool to work, you must already have valid Xbox Live XSTS Tokens.
>           This tool will not provide you with tokens, it only manages existing tokens, if you
>           need to obtain tokens, you can use the Xbox Live authentication process, or create a
>           horribly complicated script that automatically generates valid tokens.
>- **II:**  It is recommended to only do 10,000 bots per hour to prevent triggering a system which
>           essentially flags your account as being botted, staying under the 10,000 per hr keeps
>           you in the clear of this.

## Features

- Rust-only implementation using Tokio and Reqwest.
- Multiple-token processing with configurable code-level concurrency, defaulting to four simultaneous requests.
- Support for Microsoft compact JWE/RPS tickets and ready-made `XBL3.0` credentials.
- Microsoft User Authentication -> Xbox User Token -> XSTS authentication flow for JWE input.
- Correct URL encoding for gamertags.
- Accurate success reporting based on actual API responses.
- Bounded retries for transient failures and numeric `Retry-After` handling.
- Atomic token-file cleanup for confirmed permanent authentication failures.
- No full token values in application output.

## Token Files

The CLI searches for the first existing file in this order:

1. `tokens.env`
2. `.env`
3. `tokens`
4. `tokens.txt`
5. `python/tokens.txt` as a legacy fallback

Files are line-oriented: one credential per line. Empty lines and lines beginning with `#` are ignored. These names are supported as token-file names; the contents are not parsed as `KEY=value` dotenv assignments.

### Microsoft compact JWE

The supplied Microsoft token format is a five-segment compact JWE:

```text
<header>.<encrypted-key>.<iv>.<ciphertext>.<authentication-tag>
```

The header must declare `RSA-OAEP` and `A128CBC-HS256`. The application validates the structure but does not decrypt the JWE. It performs the following exchange:

1. Send `d=<JWE>` to `https://user.auth.xboxlive.com/user/authenticate`.
2. Send the returned Xbox User Token to `https://xsts.auth.xboxlive.com/xsts/authorize`.
3. Build the final `XBL3.0 x=<uhs>;<xsts_token>` authorization value.

### XBL3 credentials

The CLI also accepts either of these forms:

```text
XBL3.0 x=<user_hash>;<xsts_token>
<user_hash>;<xsts_token>
```

Ready-made XBL3 credentials skip the Microsoft and XSTS exchange.

## API Behavior

The follow request is:

```text
PUT https://social.xboxlive.com/users/me/people/gt(<percent-encoded-gamertag>)
X-XBL-Contract-Version: 2
Authorization: XBL3.0 x=<uhs>;<xsts_token>
```

HTTP `200`, `201`, `202`, and `204` count as success. A sent request, network completion, or unexpected status is never counted as success.

HTTP `408`, `429`, `5xx`, and transient transport failures are retried at most twice with exponential backoff. Numeric `Retry-After` values are honored up to 60 seconds. Permanent authentication failures are not retried indefinitely.

## Architecture

- `src/main.rs`: CLI prompts, token selection, bounded task orchestration, counters, and final summary.
- `src/token.rs`: token-file discovery, parsing, token limits, and atomic cleanup.
- `src/xbox.rs`: Microsoft authentication, XSTS exchange, follow requests, URL encoding, response classification, and retry policy.

## Usage

### Requirements

- Rust via [rustup](https://rustup.rs/)

### Clone and run

```sh
git clone https://github.com/DunixOS/XBL-Follower-Bot.git
cd XBL-Follower-Bot
cp tokens.txt.example tokens.txt
# Replace the example token with an authorized credential.
cargo run --release
```

The CLI asks how many tokens to use and then asks for the target gamertag. Press Enter to use all loaded tokens. Invalid token-count input also falls back to all loaded tokens, matching the original workflow.

## Verification

```sh
cargo fmt --check
cargo check
cargo build
cargo test
cargo clippy -- -D warnings
```

The test suite covers token parsing, JWE structure, token-file discovery, token limits, XSTS response parsing, authentication-error classification, URL encoding, success classification, retry classification, and conservative token removal. These checks do not perform a live Xbox follow.

## XTM

[Xbox Token Manager (XTM)](https://github.com/DunixOS/XTM) is a separate project for managing sessions and Xbox actions. Its planned areas include:

- following users in batches;
- direct messaging in batches;
- reporting;
- token expiry detection;
- batch execution;
- session management.

This repository is the Rust CLI rewrite of the follower tool; XTM is maintained separately.