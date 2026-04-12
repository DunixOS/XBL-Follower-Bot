# Xbox Follower Bot

>[!IMPORTANT]
This tool is abandoned, check https://github.com/playfairs/XBL-Follower-Bot for a working version of this.

A Python-based tool that allows you to follow a user on Xbox Live using multiple accounts. This tool automates the process of following a specific gamertag from multiple Xbox Live accounts.

## Important Notice

This tool and any tools which automate actions on the Xbox Live platform are against Microsoft's Terms of Service. Please ensure you have the necessary permissions to automate actions on the Xbox Live platform. Microsoft's Terms of Service should be reviewed before using this tool. (You have a high risk of getting banned if you use this tool carelessly. Use with caution.)

## Features

- Generate Xbox Live authentication tokens
- Follow a specific gamertag from multiple accounts
- Simple command-line interface
- Automatic token management
- Cross-platform support (Windows, macOS, Linux)

## Prerequisites

- Python 3.7 or higher
- pip or pip3 (Python package manager)
- Git (recommended for cloning the repository)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/playfairs/xbox-follower-bot.git
   cd xbox-follower-bot
   ```

2. Make the start script executable (Linux/macOS):
   ```bash
   chmod +x start.sh
   ```

## Quick Start

1. Run the start script:
   ```bash
   ./start.sh
   ```
   On Windows, use Git Bash or run:
   ```cmd
   .\start.sh
   ```

2. The script will:
   - Set up a Python virtual environment
   - Install required dependencies
   - Check for existing tokens
   - Guide you through token generation if needed
   - Start the follower bot

## Token Generation

If you don't have any tokens:

1. The script will prompt you to generate tokens
2. A browser window will open for Microsoft authentication
3. Sign in with your Microsoft/Xbox account
4. Grant the requested permissions
5. The token will be saved automatically

## Using the Bot

1. Run the start script:
   ```bash
   ./start.sh
   ```

2. When prompted, enter the gamertag you want to follow
3. The bot will process each token and attempt to follow the specified user
4. Progress and results will be displayed in the console

## File Structure

```
xbox-follower-bot/
├── config.py          # Shared configuration
├── requirements.txt   # Python dependencies
├── start.sh           # Launch script (Unix/Linux/macOS)
├── tokens.txt         # Stores authentication tokens (auto-created)
├── src/
│   └── main.py       # Main bot script
└── token-gen/
    └── src/
        └── main.py   # Token generator script
```

## Configuration

- `tokens.txt`: Stores your authentication tokens (one per line)
- `config.py`: Contains shared configuration settings

## Updating

To update to the latest version:

```bash
git pull origin main
```

## Troubleshooting

- **Browser doesn't open for authentication**: Copy the URL from the console and open it manually
- **Token generation fails**: Ensure you're using a valid Microsoft/Xbox account
- **Follow requests failing**: Check your internet connection and ensure the gamertag is correct


## Disclaimer

This tool is not affiliated with, maintained, authorized, endorsed, or sponsored by Microsoft or Xbox. Use at your own risk.
