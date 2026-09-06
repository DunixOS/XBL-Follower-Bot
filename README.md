# Xbox Follower CLI

This repository is a Rust-only rewrite of the former Python prototype. It accepts the Microsoft compact-JWE credentials used by the original `python/tokens.txt` and sends a follow request for each account, with bounded concurrency and conservative retries.

### Important authentication limitation

The token file may contain either already-issued Xbox Live credentials:

```text
XBL3.0 x=<user_hash>;<xsts_token>
```

or Microsoft compact JWE/RPS tickets with five dot-separated segments, such as the format in `python/tokens.txt`. The JWE header is validated as `RSA-OAEP` plus `A128CBC-HS256`; the encrypted contents are never decoded by this application.

For a JWE, the application performs the real two-step exchange: it sends `d=<JWE>` to `user.auth.xboxlive.com/user/authenticate`, sends the returned Xbox User Token to `xsts.auth.xboxlive.com/xsts/authorize`, then constructs `XBL3.0 x=<uhs>;<xsts_token>`. Ready-made XBL3.0 credentials skip these steps.

### Architecture

- `src/main.rs`: interactive CLI, bounded task orchestration, counters, and final summary.
- `src/token.rs`: one-pass token loading, JWE/XBL3 validation, token-limit behavior, and atomic removal.
- `src/xbox.rs`: Microsoft authentication, XSTS exchange, URL construction, follow request, response classification, and retry policy.

The follow request is `PUT /users/me/people/gt(<percent-encoded-gamertag>)` on `https://social.xboxlive.com`, with `Authorization: XBL3.0 x=...` and `X-XBL-Contract-Version: 2`. HTTP 200, 201, 202, and 204 are counted as success. No request, network error, or unexpected response is counted as success.

Concurrency defaults to four requests. HTTP 408, 429, 5xx, and transport failures are retried at most twice with exponential backoff. Numeric `Retry-After` values are honored up to 60 seconds. A token is removed only for HTTP 401 with a recognized permanent XErr; 403, malformed responses, rate limits, and server errors remain in the file.

### Usage

```sh
cp tokens.txt.example tokens.txt
# Replace the example with real credentials, without printing them in logs.
cargo run
```

The program checks `tokens.txt` first and falls back to `python/tokens.txt`. It asks for a token count and target gamertag. Empty input uses all non-empty lines. Invalid count input also uses all tokens, matching the original workflow while avoiding silent token-file corruption.

### Verification

```sh
cargo fmt --check
cargo check
cargo build
cargo test
cargo clippy -- -D warnings
```

These checks exercise compilation, nine unit tests, formatting, and strict linting. They do not perform a live Xbox follow and no real credential was used during development.
