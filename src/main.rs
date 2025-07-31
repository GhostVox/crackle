use crackle::{
    config::{Config, get_config},
    database,
    input::InteractiveInput,
    output::InteractiveOutput,
    session::Session,
    session::SessionType,
    setup::{self},
};
use dialoguer::{Select, theme::ColorfulTheme};
use std::fs;
use std::io::BufReader;

// we need to make sure the crackle db exists in the app config directory and then create it if it doesn't, also we need to make a in memory word db to query.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Embed the word list in the binary
    const WORD_LIST: &str = include_str!("words.txt");
    let config = get_config();

    let in_memory_word_db = setup::setup_word_db(WORD_LIST)?;
    let db_exists = fs::metadata(&config.app_db).is_ok();

    let result_db = if db_exists {
        database::DB::new(&config)?
    } else {
        let db = database::DB::new(&config)?;
        db.create_session_table()?;
        db
    };

    loop {
        let err = menu(&in_memory_word_db, &result_db, &config);
        if let Err(err) = err {
            println!("Error in menu: {err}");
            break;
        }
    }

    Ok(())
}

fn menu(
    in_memory_db: &database::DB,
    result_db: &database::DB,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let selections = &[
        "Interactive Session",
        "Generate Report",
        "Change Word Source",
        "Quit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(selections)
        .interact()
        .unwrap();
    match selection {
        0 => interactive_session(config, result_db, in_memory_db)?,
        1 => todo!(),
        // 2 => change_word_src(game)?,
        3 => return Err("exit".into()),
        _ => unreachable!(),
    }
    Ok(())
}

fn interactive_session(
    config: &Config,
    result_db: &database::DB,
    in_memory_db: &database::DB,
) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = BufReader::new(std::io::stdin());
    let input = InteractiveInput::new(buffer);
    let output = InteractiveOutput::new(std::io::stdout());
    let mut session = Session::new(
        SessionType::Interactive,
        input,
        output,
        config,
        result_db,
        in_memory_db,
    );
    session.initialize()?;
    session.start_interactive()?;
    Ok(())
}
