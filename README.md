# Rust Discord Bot

A Discord bot written in Rust with auto-embedding and utility features.

## Features

- **Auto Embedder**: Automatically converts social media links to embed-friendly versions
  - Supports: Instagram, Twitter/X, TikTok, Reddit, Facebook, Pixiv, Imgur
  - Creates webhook messages with user's avatar and username
  - Delete button for original poster or admins

- **Roll Command**: Roll random numbers
  - `!roll` - Roll 1-100
  - `!roll <max>` - Roll 1 to max

## Setup

1. Create `.env` file:
   ```
   DISCORD_TOKEN=your_bot_token_here
   ```

2. Enable intents in Discord Developer Portal:
   - Message Content Intent
   - Server Members Intent (optional)

3. Run with Docker:
   ```bash
   docker-compose up -d
   ```

   Or run locally:
   ```bash
   cargo run
   ```