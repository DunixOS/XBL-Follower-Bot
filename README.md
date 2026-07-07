# Xbox Live Follower Bot

A tool for managing Xbox Live follows using XSTS authentication tokens.

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

## Token Requirements

The script requires valid Xbox Live XSTS tokens, these tokens:
- Must be in the XBL3.0 format (preferably without "XBL3.0 x=" - we do sanitize it out.)
- Are typically obtained through Xbox Live authentication
- Have a limited lifespan and will require renewal
- Must have proper permissions for social actions

## Features

- Multi-token support for batch following
- Automatic token validation and expiration handling
- Clean console output with status updates
- Automatic removal of expired tokens

## Using XFB

If you aren't going to use the releases version -
you will need to clone the repo using:

```sh
  git clone https://github.com/DunixOS/XBL-Follower-Bot.git
  cd XBL-Follower-Bot
```

Then follow below with what version you will want.

### A. Manual - Rust

This way is recommended, as it will be the **most maintained**,
and will run more stable than the **Python version**.

<details>
  <summary>Dependencies</summary>

  - [Rust (rustup)](https://rustup.rs/)
</details>

Compiling the Rust codebase

```sh
  cargo build --release
```

Running the Rust project

```sh
  cargo run --release
```

### B. Manual - Python

<details>
  <summary>Runtime Dependencies</summary>

  - [Python 3.14](https://www.python.org/downloads/release/python-3144/)
  - Pip (should come with above Python)

  #### Collecting other dependencies from PIP

  Please note if you are running Python on Windows, and installed
  via python.org, you need to prioritize that version in PATH,
  this is due to stub binaries being used to redirect you to the
  WS (Windows Store) link.

  ```sh
    python3 -m ensurepip
    pip install -r python/requirements.txt
  ```
</details>


Running the Python

```sh
  python3 python/main.py
```

## Configuration

- `tokens.txt`: Place your authentication tokens in this file, one per line
- Tokens are automatically validated and expired tokens are removed

>[!NOTE]
>- Only Xbox Live XSTS tokens are supported
>- JWE tokens or other Microsoft authentication tokens will not work directly
>- Invalid or expired tokens are automatically removed from tokens.txt

## Token Format

Your tokens.txt file should contain XSTS tokens in the following format:

```
  XBL3.0 x=<user_hash>;<token_data>
```