# Wordle Solver Architecture & Flow

## Project Overview
Building a CLI tool to correctly guess the Wordle word of the day using probabilistic analysis and database-driven word filtering.

## Architecture Components

### 1. WordParser Module (`word_parser.rs`)
**Responsibilities:**
- Parse word lists from files
- Calculate character frequency by position
- Generate probability scores for words
- Provide processed words with calculated probabilities

**Key Structures:**
```rust
struct Character {
    character: u8,
    position: u8,
    probability: Option<u32>,  // 0-100 percentage
    frequency: u32,
}

struct Word {
    frequency: u32,
    total_probability: f64,    // 0-1.0 decimal
    word: [Character; 5],
}

struct WordParser {
    total_words: u32,
    word_stack: Vec<Word>,
    character_hash_map: HashMap<String, Character>, // "a0", "e1", etc.
}
```

**Key Methods:**
- `parse_word(word: &str)` - Add word to analysis
- `finalize_probabilities()` - Calculate all character probabilities
- `pop_n_parse() -> Option<Word>` - Get word with calculated probability

### 2. Database Module (`database.rs`)
**Responsibilities:**
- Store words with their probabilities
- Retrieve best words based on probability
- Filter words based on game constraints
- Persist game results and statistics

**Database Schema:**
```sql
-- Words table
CREATE TABLE words (
    id INTEGER PRIMARY KEY autoincrement,
    frequency INTEGER,
    total_probability REAL,
    word VARCHAR(5)
);

-- Characters table
CREATE TABLE characters (
    id SMALLINT PRIMARY KEY autoincrement,
    character VARCHAR(1),
    position SMALLINT CHECK(position >= 0 AND position <= 4),
    probability REAL,
    frequency INTEGER
);

-- Game results
CREATE TABLE game_results (
    id INTEGER PRIMARY KEY autoincrement,
    word_id INTEGER REFERENCES words(id),
    date DATE DEFAULT CURRENT_DATE,
    win BOOLEAN NOT NULL,
    number_of_guesses INTEGER CHECK(number_of_guesses >= 1 AND number_of_guesses <= 6)
);
```

**Key Methods to Implement:**
- `add_word(word: Word)` - Store processed word
- `get_best_words(limit: usize) -> Vec<Word>` - Get highest probability words
- `filter_words_by_constraints(game_state: &GameState) -> Vec<Word>`
- `get_words_matching_pattern(pattern: &str) -> Vec<Word>`
- `get_words_containing_letters(letters: &[char]) -> Vec<Word>`
- `get_words_excluding_letters(letters: &[char]) -> Vec<Word>`

### 3. Game Logic Module
**Responsibilities:**
- Manage game state and constraints
- Handle user input for guess results
- Coordinate between parser and database
- Drive the guessing loop

**Game State:**
```rust
struct GameState {
    known_positions: Vec<Option<char>>,  // [Some('a'), None, Some('e'), None, None]
    present_letters: Vec<char>,          // Letters in word but wrong position
    absent_letters: Vec<char>,           // Letters definitely not in word
    guesses_made: u32,
}

enum GuessResult {
    Correct(usize),    // Green - correct letter in correct position
    Present(usize),    // Yellow - correct letter in wrong position
    Absent(usize),     // Gray - letter not in word
}
```

## Complete Workflow

### Phase 1: Setup & Initialization
1. **Read word list file** - Load all possible Wordle words
2. **Parse through WordParser:**
   ```rust
   let mut parser = WordParser::new();
   for word in word_list {
       parser.parse_word(word)?;
   }
   parser.finalize_probabilities();
   ```
3. **Store to database:**
   ```rust
   while let Some(word) = parser.pop_n_parse() {
       db.add_word(word)?;
   }
   ```

### Phase 2: Game Loop
1. **Get best word from database:**
   ```rust
   let best_words = db.get_best_words(1);
   let guess = &best_words[0];
   ```

2. **Present word to user:**
   ```
   Guess: AROSE
   Enter results (G=Green, Y=Yellow, B=Black):
   Position 0 (A): G
   Position 1 (R): B
   Position 2 (O): Y
   Position 3 (S): B
   Position 4 (E): G
   ```

3. **Update game state:**
   ```rust
   game_state.update_from_guess("arose", &results);
   ```

4. **Filter remaining words:**
   ```rust
   let candidates = db.filter_words_by_constraints(&game_state);
   ```

5. **Repeat until solved or 6 guesses**

### Phase 3: Database Filtering Logic

**SQL Query Building Examples:**
```sql
-- Words with known positions (Green letters)
SELECT * FROM words WHERE
    SUBSTR(word, 1, 1) = 'A' AND    -- Position 0 = A
    SUBSTR(word, 5, 1) = 'E'        -- Position 4 = E

-- Words containing letters but not in specific positions (Yellow letters)
SELECT * FROM words WHERE
    word LIKE '%O%' AND             -- Contains O
    SUBSTR(word, 3, 1) != 'O'       -- But not in position 2

-- Words excluding letters (Black letters)
SELECT * FROM words WHERE
    word NOT LIKE '%R%' AND
    word NOT LIKE '%S%'

-- Combined with probability ordering
ORDER BY total_probability DESC
LIMIT 10;
```

## Implementation Priority

### Phase 1: Core Setup
- [ ] Fix database filename consistency (`crackle.db` vs `database.db`)
- [ ] Consolidate duplicate Word structs
- [ ] Implement file reader for word lists
- [ ] Test parser → database workflow

### Phase 2: Database Queries
- [ ] Implement filtering methods in database module
- [ ] Add SQL query builders for constraints
- [ ] Test filtering logic with sample data

### Phase 3: Game Interface
- [ ] Build user input parser for guess results
- [ ] Implement game state management
- [ ] Create main game loop
- [ ] Add CLI interface

### Phase 4: Optimization
- [ ] Add word frequency tracking
- [ ] Implement information theory scoring
- [ ] Store game results for learning
- [ ] Performance optimization

## Key Design Decisions

1. **Separation of Concerns:** Parser handles analysis, Database handles persistence, Game logic coordinates
2. **Probability Storage:** Characters store 0-100 percentages, Words store 0-1.0 decimals
3. **Position-Based Analysis:** Character probabilities calculated per position (crucial for Wordle)
4. **Database-Driven Filtering:** Use SQL for efficient word filtering based on constraints
5. **Incremental Refinement:** Re-analyze remaining words after each guess for better suggestions
