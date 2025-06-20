
```

██████╗██████╗  █████╗  ██████╗██╗  ██╗██╗     ███████╗
██╔════╝██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██║     ██╔════╝
██║     ██████╔╝███████║██║     █████╔╝ ██║     █████╗
██║     ██╔══██╗██╔══██║██║     ██╔═██╗ ██║     ██╔══╝
╚██████╗██║  ██║██║  ██║╚██████╗██║  ██╗███████╗███████╗
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝

 ` ` `


    🎯 Probabilistic Wordle Solver 🎯
A sophisticated CLI Wordle solver that uses probabilistic analysis and database-driven word filtering to systematically crack any Wordle puzzle.

## Overview

Crackle analyzes character frequency by position across a comprehensive word list, calculates probability scores for optimal guessing, and uses your feedback to narrow down possibilities until it finds the answer. Built in Rust with SQLite for efficient word storage and filtering.

## Features

- **Probabilistic Word Analysis**: Calculates character frequency by position to suggest optimal starting words
- **Interactive Game Loop**: Guides you through each guess with simple feedback input
- **Database-Driven Filtering**: Uses SQLite for fast word filtering based on game constraints
- **Game Statistics**: Tracks wins, number of guesses, and performance over time
- **Smart Constraint Handling**: Properly handles green/yellow/gray letter feedback
- **Comprehensive Word List**: Includes extensive vocabulary for better probability calculations

## Quick Start

### Prerequisites

- Rust (2024 edition)
- SQLite (bundled with rusqlite)

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd crackle
```

2. Set up environment variables:
```bash
# Create .env file
echo "WORD_SOURCE=words.txt" > .env
echo "LIMIT=10" >> .env
```

3. Build and run:
```bash
cargo run
```

## How to Use

1. **Start the game** - Crackle will suggest a starting word based on probability analysis
2. **Enter the word in Wordle** - Use the suggested word in your Wordle game
3. **Provide feedback** - Tell Crackle the results using this format:
   - `G` = Green (correct letter in correct position)
   - `Y` = Yellow (correct letter in wrong position)
   - `N` = Gray (letter not in word)

### Example Game Session

```
Welcome to Crackle!
Starting game with word: ARISE

Please enter which characters were in the right position
Example: gyngy

> gnnyn
Next possible guesses: ["ABOUT", "ALIEN", "ALONE"]
Starting game with word: ABOUT

> ggynn
Next possible guesses: ["ABBEY"]
Starting game with word: ABBEY

> ggggg
You won! 🎉
```

## Input Format

For each guess, enter exactly 5 characters representing the Wordle feedback:

- **G** (Green): Letter is correct and in the right position
- **Y** (Yellow): Letter is in the word but in the wrong position
- **N** (Gray): Letter is not in the word at all

**Example**: If you guessed "ARISE" and got:
- A is green (position 0)
- R is gray
- I is yellow
- S is gray
- E is green (position 4)

Enter: `gnygn`

## Project Structure

```
src/
├── main.rs           # Entry point and initialization
├── lib.rs            # Module exports
├── setup.rs          # Database setup and word list processing
├── word_analyzer.rs  # Core probability analysis engine
├── database.rs       # SQLite operations and word filtering
└── game_loop.rs      # Interactive game logic and user interface

words.txt             # Comprehensive word list for analysis
.env                  # Environment configuration
```

## Architecture

### Core Components

1. **WordAnalyzer**: Analyzes word lists and calculates character probabilities by position
2. **Database**: Stores processed words with probabilities and handles constraint-based filtering
3. **GameLoop**: Manages interactive gameplay, user input, and game state
4. **Setup**: Initializes database and processes word list on first run

### Database Schema

```sql
-- Stores words with calculated probabilities
CREATE TABLE words (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    total_probability REAL,
    word VARCHAR(5)
);

-- Tracks game performance
CREATE TABLE game_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER REFERENCES words(id),
    date DATE DEFAULT CURRENT_DATE,
    win BOOLEAN NOT NULL,
    number_of_guesses INTEGER CHECK(number_of_guesses >= 1 AND number_of_guesses <= 6)
);
```

### Algorithm Flow

1. **Initialization**: Parse word list, calculate character frequencies by position
2. **Probability Calculation**: Score each word based on sum of character probabilities
3. **Database Storage**: Store all words with calculated probabilities
4. **Game Loop**:
   - Retrieve highest probability word from remaining candidates
   - Present to user for Wordle input
   - Parse user feedback (G/Y/N format)
   - Filter database based on constraints
   - Repeat until solved

## Configuration

Environment variables in `.env`:

- `WORD_SOURCE`: Path to word list file (default: `words.txt`)
- `LIMIT`: Number of top words to consider for random selection (default: `10`)

## Dependencies

- **rusqlite**: SQLite database operations with bundled SQLite
- **thiserror**: Error handling and custom error types
- **dotenv**: Environment variable management
- **rand**: Random selection from top probability words

## Game Statistics

Crackle automatically tracks:
- Words used in each game
- Number of guesses required
- Win/loss outcomes
- Date of each game

View statistics by querying the database directly or implementing additional reporting features.

## Error Handling

Crackle includes comprehensive error handling for:
- Invalid input formats
- Database connection issues
- Word list parsing problems
- File I/O errors

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Performance Notes

- First run processes the entire word list and may take a few seconds
- Subsequent runs are fast as the database is pre-populated
- Word filtering uses indexed SQL queries for optimal performance
- Memory usage is minimal as words are streamed from database

## Future Enhancements

- [ ] Hard mode support (must use revealed letters)
- [ ] Multiple word list support
- [ ] Advanced information theory scoring
- [ ] Web interface
- [ ] Performance analytics and suggestions
- [ ] Custom word list upload

---

**Happy Wordling!** 🎮
