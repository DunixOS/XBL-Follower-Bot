# Xbox Live Follower Bot

A Python-based tool for managing Xbox Live follows using XSTS authentication tokens.

![](https://rv.playfairs.cc/DunixOS/XBL-Follower-Bot)

---

>[!IMPORTANT]
>- **I:** If you need help with this tool and how to get it working, you can DM [@playfairs](<https://discord.com/users/1426711359059394662>) on Discord.
>- **II:** If you specifically need help obtaining **Xbox Tokens**, I can **NOT** help you with this, I was working on a tool for a while, but it because outdated after short time, so if you add me to ask for tokens, I can and will **NOT** help or provide tokens as I both cannot nor have interesting in creating / obtaining them myself.
>- **III:** Assuming you want to DM me on Discord, before you DM me about ANY other tools, please read the **README** displayed at the [Home of this Organiztion](https://github.com/DunixOS).


>[!NOTE]
>- **I:** In order for this tool to work, you must already have valid Xbox Live XSTS Tokens. This tool will not provide you with tokens, it only manages existing tokens, if you need to obtain tokens, you can use the Xbox Live authentication process, or create a horribly complicated script that automatically generates valid tokens.
>- **II:** It is recommended to only do 10,000 bots per hour to prevent triggering a system which essentially flags your account as being botted, staying under the 10,000 per hr keeps you in the clear of this.

## Token Requirements

The script requires valid Xbox Live XSTS tokens. These tokens:
- Must be in XBL3.0 format (starting with "XBL3.0 x=")
- Are typically obtained through Xbox Live authentication
- Have a limited lifespan and may need periodic renewal
- Must have proper permissions for social actions

## Features

- Multi-token support for batch following
- Automatic token validation and expiration handling
- Clean console output with status updates
- Automatic removal of expired tokens

## Requirements

- Python 3.6+
- aiohttp
- colorama

## Installation

1. Clone the repository
```bash
git clone https://github.com/DunixOS/XBL-Follower-Bot.git
cd XBL-Follower-Bot
```

2. Install dependencies
```bash
pip install -r requirements.txt
```

3. Create a tokens.txt file in the project directory and add your XSTS tokens (one per line)

## Token Format

Your tokens.txt file should contain XSTS tokens in the following format:
```
XBL3.0 x=<user_hash>;<token_data>
```

Each token should be on its own line. The user_hash and token_data are specific to your Xbox Live authentication.

## Usage

1. Run the script:
```bash
python main.py
```
or
```bash
python3 main.py
```

2. When prompted, enter the target gamertag to follow

## Configuration

- `tokens.txt`: Place your authentication tokens in this file, one per line
- Tokens are automatically validated and expired tokens are removed

>[!NOTE]
>- Only Xbox Live XSTS tokens are supported
>- JWE tokens or other Microsoft authentication tokens will not work directly
>- Invalid or expired tokens are automatically removed from tokens.txt
