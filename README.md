# 🎯 CRACKLE - Probabilistic Wordle Solver

```
██████╗██████╗  █████╗  ██████╗██╗  ██╗██╗     ███████╗
██╔════╝██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██║     ██╔════╝
██║     ██████╔╝███████║██║     █████╔╝ ██║     █████╗
██║     ██╔══██╗██╔══██║██║     ██╔═██╗ ██║     ██╔══╝
╚██████╗██║  ██║██║  ██║╚██████╗██║  ██╗███████╗███████╗
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝
```

**A sophisticated CLI Wordle solver that uses probabilistic analysis and database-driven word filtering to systematically crack any Wordle puzzle.**

## 🌟 Features

- **🧮 Probabilistic Analysis**: Calculates character frequency by position across a comprehensive word list for optimal starting words
- **🎮 Interactive Game Loop**: Guides you through each guess with simple feedback input using G/Y/N notation
- **🗄️ SQLite Database**: Fast word filtering and storage with probability-based ranking
- **📊 Game Statistics**: Tracks wins, attempts, and performance over time
- **🎯 Smart Constraint Handling**: Advanced filtering logic for green/yellow/gray letter feedback
- **📚 Comprehensive Word List**: Extensive vocabulary for better probability calculations
- **🔄 Dynamic Word Source**: Change word lists on-the-fly through interactive menu

## 🚀 Quick Start

### Prerequisites

- Rust (2024 edition)
- SQLite (bundled with rusqlite)

### Installation

1. **Clone the repository:**
```bash
git clone <your-repo-url>
cd crackle
```

2. **Set up environment variables:**
```bash
# Create .env file
echo "WORD_SOURCE=words.txt" > .env
echo "LIMIT=10" >> .env
```

3. **Build and run:**
```bash
cargo run
```

## 🎯 How to Use

### Main Menu
When you start Crackle, you'll see an interactive menu with options to:
- **Play** - Start a new Wordle solving session
- **Generate Report** - View game statistics (coming soon)
- **Change Word Source** - Switch to a different word list
- **Quit** - Exit the application

### Game Session

1. **Get AI Suggestion** - Crackle suggests a starting word based on probability analysis
2. **Enter in Wordle** - Use the suggested word in your actual Wordle game
3. **Provide Feedback** - Tell Crackle the results using this format:
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
Next possible guesses: ABOUT
Starting game with word: ABOUT

> ggynn
Next possible guesses: ABBEY
Starting game with word: ABBEY

> ggggg
You won! 🎉
Game results stored successfully!
```

## 🔤 Input Format

For each guess, enter exactly 5 characters representing the Wordle feedback:

| Character | Meaning | Example |
|-----------|---------|---------|
| **G** | Green - Correct letter, correct position | A is in position 0 |
| **Y** | Yellow - Correct letter, wrong position | I is in the word but not position 2 |
| **N** | Gray - Letter not in word | R is not in the word at all |

**Example**: If you guessed "ARISE" and got:
- A is green (position 0) ✅
- R is gray ❌
- I is yellow (in word, wrong position) 🟡
- S is gray ❌
- E is green (position 4) ✅

Enter: `gnyng`

## 🏗️ Project Architecture

```
src/
├── main.rs           # Entry point with interactive menu
├── lib.rs            # Module exports
├── setup.rs          # Database initialization and word list processing
├── word_analyzer.rs  # Core probability analysis engine
├── database.rs       # SQLite operations and word filtering
├── game_loop.rs      # Interactive game logic and user interface
├── filter_logic.rs   # Advanced word filtering algorithms
└── arena.rs          # Testing framework (in development)

words.txt             # Default comprehensive word list
.env                  # Environment configuration
```

### Core Components

1. **WordAnalyzer**: Analyzes word lists and calculates character probabilities by position
2. **Database**: Stores words with probabilities and handles constraint-based filtering
3. **GameLoop**: Manages interactive gameplay, user input, and game state
4. **FilterLogic**: Advanced filtering for yellow positions and excluded characters
5. **Setup**: Initializes database and processes word lists on first run

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
    word VARCHAR(5),
    date DATE DEFAULT CURRENT_DATE,
    win BOOLEAN NOT NULL,
    number_of_guesses INTEGER CHECK(number_of_guesses >= 1 AND number_of_guesses <= 6)
);
```

### Algorithm Flow

1. **Initialization**: Parse word list, calculate character frequencies by position
2. **Probability Calculation**: Score each word based on sum of positional character probabilities
3. **Database Storage**: Store all words with calculated probabilities, indexed for fast retrieval
4. **Game Loop**:
   - Retrieve highest probability words from remaining candidates
   - Present random selection from top candidates to user
   - Parse user feedback (G/Y/N format)
   - Apply advanced filtering based on constraints
   - Repeat until solved or maximum attempts reached

## ⚙️ Configuration

Environment variables in `.env`:

- `WORD_SOURCE`: Path to word list file (default: `words.txt`)
- `LIMIT`: Number of top words to consider for random selection (default: `10`)

## 📦 Dependencies

- **rusqlite**: SQLite database operations with bundled SQLite
- **thiserror**: Error handling and custom error types
- **dotenv**: Environment variable management
- **rand**: Random selection from top probability words
- **dialoguer**: Interactive CLI menus and prompts

## 📊 Game Statistics

Crackle automatically tracks:
- Words used in each game
- Number of guesses required
- Win/loss outcomes
- Date of each game

Statistics are stored in the SQLite database for future reporting features.

## 🛠️ Error Handling

Comprehensive error handling for:
- Invalid input formats (wrong length, invalid characters)
- Database connection issues
- Word list parsing problems
- File I/O errors
- Missing word source files

## 🧪 Testing

The project includes comprehensive unit tests for:
- Word filtering logic with various constraint combinations
- Game state management and user input parsing
- Word analysis and probability calculations
- Database operations

Run tests with:
```bash
cargo test
```

## 🚧 Known Issues & TODO

### Issues to Fix
- [ ] Fix filtering logic for duplicate characters where first is green and second is gray
- [ ] Improve handling of repeated letters in complex scenarios

### Features in Development
- [ ] Enhanced terminal UI experience
- [ ] Comprehensive game statistics and reporting
- [ ] Performance analytics and optimization suggestions
- [ ] Testing arena for algorithm validation

### Planned Enhancements
- [ ] Hard mode support (must use revealed letters)
- [ ] Multiple word list support with easy switching
- [ ] Advanced information theory scoring
- [ ] Web interface
- [ ] Custom word list upload functionality

## 📈 Performance Notes

- **First Run**: Processes entire word list and builds database (may take a few seconds)
- **Subsequent Runs**: Fast startup as database is pre-populated
- **Word Filtering**: Uses indexed SQL queries for optimal performance
- **Memory Usage**: Minimal as words are streamed from database rather than held in memory

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎮 Example Usage

```bash
$ cargo run

? What would you like to do?
❯ Play
  Generate Report
  Change Word Source
  Quit

# Select "Play"

Welcome to Crackle!
Starting game with word: CRANE

Please enter which characters were in the right position
Example: gyngy

> nnygg
Next possible guesses: STEEL
Starting game with word: STEEL

> ggggg
You won! 🎉
Game results stored successfully!
See you tomorrow!
```

---

**Happy Wordling!** 🎮✨

*Crackle makes solving Wordle puzzles a breeze with its intelligent probability-based approach and user-friendly interface.*
