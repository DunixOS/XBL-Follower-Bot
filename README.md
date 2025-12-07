# Xbox Live Follower Bot

A Python-based tool for managing Xbox Live follows using XSTS authentication tokens.

>[!IMPORTANT]
>- 1. In order for this tool to work, you must already have valid Xbox Live XSTS Tokens. This tool will not provide you with tokens, it only manages existing tokens, if you need to obtain tokens, you can use the Xbox Live authentication process, or create a horribly complicated script that automatically generates valid tokens.
>- 2. If you need help with this tool and how to get it working, you can DM [@playfairs](<https://discord.com/users/1426711359059394662>) on Discord.
>- 3. Assuming you want to DM me on Discord, do you DM me if you have questions about tools such as Autoclaimers, or other Xbox tools, I will work on those tools when I have the time, I am tired of people asking me to make tools that I don't provide, if I do not provide something, Do not ask me to make it. There are reasons why I don't have some tools.

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
git clone https://github.com/playfairs/XBL-Follower-Bot.git
cd xbox-follower-bot
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

2. When prompted, enter the target gamertag to follow

## Configuration

- `tokens.txt`: Place your authentication tokens in this file, one per line
- Tokens are automatically validated and expired tokens are removed

>[!NOTE]
>- Only Xbox Live XSTS tokens are supported
>- JWE tokens or other Microsoft authentication tokens will not work directly
>- Invalid or expired tokens are automatically removed from tokens.txt
