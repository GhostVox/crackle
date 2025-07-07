use crate::game_loop::GameLoop;
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
    #[error("Can not get working directory")]
    WorkingDirectoryError,
}

// We need to set up the database, instantiate the WordParser, and then start parsing the words.txt in the root directory and adding the finished words to the database.

/// The setup function gets the path to the initial word source file, opens the file and reads each word from the file calculates the probability and then adds it to the database.
pub fn setup() -> Result<DB, SetupError> {
    let word_source = std::env::var("WORD_SOURCE").unwrap_or_else(|_| "words.txt".to_string());
    let wd = get_working_directory()?;
    let full_path = wd.join(word_source);
    check_full_path(&full_path)?;
    // set up word analyzer and database
    let mut word_analyzer = WordAnalyzer::new();
    let db = DB::new()?;
    db.setup()?;
    read_words_from_file(&full_path, &mut word_analyzer)?;

    word_analyzer.finalize_probabilities();

    db.batch_insert(word_analyzer.words())?;

    Ok(db)
}

fn read_words_from_file(
    word_source: &std::path::Path,
    word_analyzer: &mut WordAnalyzer,
) -> Result<(), SetupError> {
    if std::fs::metadata(word_source).is_err() {
        return Err(SetupError::WordSourceDoesNotExist);
    }
    // setup file and reader
    let _file = std::fs::File::open(word_source)?;
    let reader = BufReader::new(_file);

    // walk through the file line by line and analyze each word

    for line in reader.lines() {
        let line = line?;
        if let Err(err) = word_analyzer.analyze_word(&line) {
            eprintln!("Error analyzing word: {err}");
        }
    }
    Ok(())
}

// fn add_words_to_db(word_analyzer: &mut WordAnalyzer, db: &DB) -> Result<(), SetupError> {
//     // Pop words from the analyzer and add them to the database until there are no more
//     loop {
//         match word_analyzer.pop() {
//             Ok(Some(word)) => {
//                 db.add_word(word)?;
//             }
//             Ok(None) => break,
//             Err(word_analyzer::WordAnalyzerError::ProbabilitiesNotFinalized) => {
//                 word_analyzer.finalize_probabilities();
//             }
//         }
//     }
//     Ok(())
// }

pub fn change_word_src(game: &GameLoop) -> Result<(), SetupError> {
    let wd = get_working_directory()?;
    let word_source = get_new_word_source_path()?;
    let full_path = wd.join(&word_source);
    print!("Changing word source to {}", full_path.display());
    check_full_path(&full_path)?;
    let mut word_analyzer = WordAnalyzer::new();
    read_words_from_file(&full_path, &mut word_analyzer)?;
    // Pop words from the analyzer and add them to the database until there are no more
    word_analyzer.finalize_probabilities();
    game.db.delete_words()?;
    game.db.create_words_table()?;
    game.db.batch_insert(word_analyzer.words())?;
    Ok(())
}

pub fn get_new_word_source_path() -> Result<String, SetupError> {
    let mut word_source = String::new();
    println!("Enter the path to the new word source file:");
    std::io::stdin().read_line(&mut word_source)?;
    Ok(word_source.trim().to_string())
}

pub fn get_working_directory() -> Result<std::path::PathBuf, SetupError> {
    let wd = std::env::current_dir().map_err(|_| SetupError::WorkingDirectoryError)?;
    Ok(wd)
}

pub fn check_full_path(full_path: &std::path::Path) -> Result<(), SetupError> {
    if !full_path.exists() {
        return Err(SetupError::WordSourceDoesNotExist);
    }
    Ok(())
}
