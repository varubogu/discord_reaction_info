# discord_reaction_info
A Discord bot that displays the results of reaction tallies on messages.

## Overview
This bot allows users to get information about reactions on a Discord message. It provides a `/rinfo` command that takes a message URL or ID and returns a formatted list of reactions and the users who reacted.

## Features
- Parse Discord message URLs to extract channel and message IDs
- Fetch reactions from messages
- Filter reactions and users based on various criteria
- Format reaction information in a readable way

## Project Structure
- `src/main.rs`: Entry point for the bot, handles Discord events and interactions
- `src/commands/`: Contains command implementations
  - `src/commands/mod.rs`: Registers commands with Discord
  - `src/commands/rinfo.rs`: Implements the `/rinfo` command
- `src/utils.rs`: Utility functions for parsing message URLs and creating responses

## Testing
The project includes unit tests for the utility functions. Run the tests with:

```bash
cargo test
```

## Implementation Details
The code has been structured to be modular and testable:
- Functions are broken down into small, focused units
- Each function has a single responsibility
- Error handling is consistent throughout the codebase
- Documentation is provided for all public functions

## Setup and Running
1. Clone the repository
2. Create a `.env` file with your Discord bot token:
   ```
   DISCORD_TOKEN=your_token_here
   ```
3. Run the bot with:
   ```bash
   cargo run
   ```

## Commands
### `/rinfo`
Get reaction information for a message.

Parameters:
- `message`: Message URL or ID (required)
- `exclude_user`: Users to exclude from the results (optional)
- `exclude_reaction`: Reactions to exclude from the results (optional)
- `include_message_user`: Include the message author in the results (optional)
- `user_only`: Only show users, not grouped by reaction (optional)
