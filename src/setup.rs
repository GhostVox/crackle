use crate::word_analyzer::WordAnalyzer;
use crate::{database::DB, word_analyzer};
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Rusqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),

    #[error("Word source does not exist")]
    WordSourceDoesNotExist,
}

// We need to set up the database, instantiate the WordParser, and then start parsing the words.txt in the root directory and adding the finished words to the database.

/// The setup function gets the path to the initial word source file, opens the file and reads each word from the file calculates the probability and then adds it to the database.
pub fn setup() -> Result<DB, SetupError> {
    let word_source = std::env::var("WORD_SOURCE").unwrap_or_else(|_| "words.txt".to_string());
    if !std::fs::metadata(&word_source).is_ok() {
        return Err(SetupError::WordSourceDoesNotExist);
    }
    // setup file and reader
    let _file = std::fs::File::open(&word_source)?;
    let reader = BufReader::new(_file);

    // set up word analyzer and database
    let mut word_analyzer = WordAnalyzer::new();
    let db = DB::new()?;
    db.setup()?;

    // walk through the file line by line and analyze each word
    for line in reader.lines() {
        let line = line?;
        let result = word_analyzer.analyze_word(&line);
        if let Err(err) = result {
            eprintln!("Error analyzing word: {}", err);
        }
    }

    word_analyzer.finalize_probabilities();

    // Pop words from the analyzer and add them to the database until there are no more
    loop {
        match word_analyzer.pop() {
            Ok(Some(word)) => {
                db.add_word(word)?;
            }
            Ok(None) => break,
            Err(word_analyzer::WordAnalyzerError::ProbabilitiesNotFinalized) => {
                word_analyzer.finalize_probabilities();
            }
        }
    }

    Ok(db)
}
