use crate::game_loop::GameResults;

use super::word_analyzer::{Character, Word};
use rusqlite::{Connection, Result, params};

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Result<Self, rusqlite::Error> {
        Ok(Self {
            conn: Connection::open("crackle.db")?,
        })
    }
}

impl DB {
    pub fn setup(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "CREATE TABLE  IF NOT EXISTS words (
                id INTEGER PRIMARY KEY autoincrement,
                total_probability REAL,
                word VARCHAR(5)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS game_results(
        id INTEGER PRIMARY KEY autoincrement,
        word_id INTEGER REFERENCES Words(id),
        date DATE DEFAULT CURRENT_DATE,
        win BOOLEAN NOT NULL,
        number_of_guesses INTEGER NOT NULL CHECK(number_of_guesses >= 1 AND number_of_guesses <= 6)
    )",
            [],
        )?;

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
    pub fn add_word(&self, word: Word) -> Result<(), rusqlite::Error> {
        let word_str = word.as_str();
        let mut stmt = self
            .conn
            .prepare("INSERT INTO words (word, total_probability) VALUES (?1, ?2)")?;
        stmt.execute(params![word_str, word.total_probability])?;
        Ok(())
    }
    pub fn add_character(&self, character: &Character) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO Characters (character, position, probability, frequency) VALUES (?1, ?2, ?3, ?4)",
        )?;
        stmt.execute(params![
            character.character,
            character.position,
            character.probability,
            character.frequency
        ])?;
        Ok(())
    }

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

    pub fn store_game_results(&self, game_results: GameResults) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO game_results (word, number_of_guesses, win) VALUES (?1, ?2, ?3)",
        )?;
        stmt.execute(params![
            game_results.word.as_str(),
            game_results.number_of_guesses,
            game_results.win
        ])?;
        Ok(())
    }

    pub fn filter_words(&self, pattern: &str) -> Result<Vec<String>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT word FROM words WHERE word LIKE ?1 ")?;

        let word_iter = stmt.query_map(params![pattern], |row| row.get(0))?;

        word_iter.collect()
    }
}
