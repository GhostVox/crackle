use super::word_parser::Word;
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
                frequency INTEGER,
                total_probability REAL,
                Word VARCHAR(5)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS game_results(
        id INTEGER PRIMARY KEY autoincrement,
        word_id INTEGER FOREIGN KEY REFERENCES Words(id),
        date DATE DEFAULT CURRENT_DATE,
        win BOOLEAN NOT NULL,
        number_of_guesses INTEGER NOT NULL CHECK(number_of_guesses >= 1 AND number_of_guesses <= 6)
    )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Characters(
        id SMALLINT PRIMARY KEY autoincrement,
        character VARCHAR(1),
        position SMALLINT check(position >= 0 AND position <= 4),
        probability REAL,
        frequency INTEGER
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
        let mut stmt = self.conn.prepare(
            "INSERT INTO words (word, frequency, total_probability) VALUES (?1, ?2, ?3)",
        )?;
        stmt.execute(params![word_str, word.frequency, word.total_probability])?;
        Ok(())
    }
}
