use crate::game_loop::GameResults;

use super::word_analyzer::Word;
use rusqlite::{Connection, Result, params};
/// This is a wrapper for the database to easily interact with it.
pub struct DB {
    conn: Connection,
}

impl DB {
    /// Creates a new instance of the database.
    /// This function is only called if the database does not already exist.
    pub fn new() -> Result<Self, rusqlite::Error> {
        Ok(Self {
            conn: Connection::open("crackle.db")?,
        })
    }
}

impl DB {
    /// Sets up the database by creating the necessary tables and indexes.
    /// This function is only called if the database does not already exist.
    pub fn setup(&self) -> Result<(), rusqlite::Error> {
        self.create_words_table()?;

        self.create_game_results_table()?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS word_prob_idx ON words(total_probability)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS number_of_guesses_idx ON game_results(number_of_guesses)",
            [],
        )?;
        Ok(())
    }

    /// Adds a new word to the database.
    /// expects a word struct to be passed in
    /// ```
    /// use crackle::word_analyzer::Character;
    ///
    /// struct Word {
    ///      pub frequency: u32,
    ///      pub total_probability: f64,
    ///      pub word: [Character; 5],
    /// }
    /// ```
    pub fn add_word(&self, word: Word) -> Result<(), rusqlite::Error> {
        let word_str = word.as_str();
        let mut stmt = self
            .conn
            .prepare("INSERT INTO words (word, total_probability) VALUES (?1, ?2)")?;
        stmt.execute(params![word_str, word.total_probability])?;
        Ok(())
    }

    pub fn batch_insert(&self, words: &[Word]) -> Result<(), rusqlite::Error> {
        let tx = self.conn.unchecked_transaction()?;
        {
            let mut stmt =
                tx.prepare("INSERT INTO words (word, total_probability) VALUES (?1, ?2)")?;

            for word in words {
                let word_str = word.as_str();
                stmt.execute(params![word_str, word.total_probability])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Gets the words with the highest probability from the database. Requires a limit to be specified.
    pub fn get_top_words(&self, limit: usize) -> Result<Vec<Word>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT word, total_probability FROM words ORDER BY total_probability DESC LIMIT ?1",
        )?;

        let word_iter = stmt.query_map(params![limit], |row| {
            let word_as_str: String = row.get(0)?;
            let total_probability: f64 = row.get(1)?;
            Ok((word_as_str, total_probability))
        })?;

        let mut words = Vec::new();
        for row_result in word_iter {
            let (word_str, prob) = row_result?;
            if let Ok(word) = Word::new(0, prob, &word_str) {
                words.push(word);
            }
            // Skip invalid words silently
        }
        Ok(words)
    }

    /// Stores the results of a game in the database.
    ///
    /// # Arguments
    ///
    /// * `game_results` - The results of the game to store.
    /// ```
    /// struct GameResults {
    ///     word: String,
    ///     number_of_guesses: usize,
    ///     win: bool,
    /// }
    /// ```
    /// # Returns
    ///
    /// * `Ok(())` - The results were successfully stored.
    /// * `Err(rusqlite::Error)` - An error occurred while storing the results.
    pub fn store_game_results(&self, game_results: GameResults) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO game_results (word, number_of_guesses, win) VALUES (?1, ?2, ?3)",
        )?;
        stmt.execute(params![
            game_results.word,
            game_results.number_of_guesses,
            game_results.win
        ])?;
        Ok(())
    }

    /// Retrieves the ID of a word from the database.
    ///
    /// # Arguments
    ///
    /// * `w` - The word to retrieve the ID for.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The ID of the word.
    /// * `Err(rusqlite::Error)` - An error occurred while retrieving the word ID.
    pub fn get_word(&self, w: &str) -> Result<usize, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id FROM words WHERE word = ?1")?;
        let word_id = stmt.query_row(params![w], |row| row.get(0))?;
        Ok(word_id)
    }

    /// Filters words in the database based on a LIKE pattern built by the gameloop struct in game_loop.rs.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to filter words by.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)` - A vector of words that match the pattern.
    /// * `Err(rusqlite::Error)` - An error occurred while filtering words.
    pub fn filter_words(&self, pattern: &str) -> Result<Vec<String>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT word FROM words WHERE word LIKE ?1 ")?;

        let word_iter = stmt.query_map(params![pattern], |row| row.get(0))?;

        word_iter.collect()
    }

    pub fn delete_words(&self) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare("DROP TABLE words;")?;

        stmt.execute(params![])?;
        Ok(())
    }
    pub fn create_words_table(&self) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "CREATE TABLE IF NOT EXISTS words (
                id INTEGER PRIMARY KEY autoincrement,
                total_probability REAL,
                word VARCHAR(5)
            )",
        )?;

        stmt.execute(params![])?;
        Ok(())
    }
    pub fn create_game_results_table(&self) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "CREATE TABLE IF NOT EXISTS game_results(
                id INTEGER PRIMARY KEY autoincrement,
                word VARCHAR(5),
                date DATE DEFAULT CURRENT_DATE,
                win BOOLEAN NOT NULL,
                number_of_guesses INTEGER NOT NULL CHECK(number_of_guesses >= 1 AND number_of_guesses <= 6)
            )",
        )?;

        stmt.execute(params![])?;
        Ok(())
    }
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.create_words_table()?;
        db.create_game_results_table()?;
        Ok(db)
    }
}
