use crackle::{
    config::{Config, get_config},
    database, game_loop,
    setup::{self},
};
use dialoguer::{Select, theme::ColorfulTheme};
use std::fs;
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
        db.create_game_results_table()?;
        db
    };

    let mut game = game_loop::GameLoop::new(in_memory_word_db, result_db);
    loop {
        let err = menu(&mut game, &config);
        if let Err(err) = err {
            println!("Error in menu: {err}");
            break;
        }
    }

    Ok(())
}

fn menu(game: &mut game_loop::GameLoop, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let selections = &["Play", "Generate Report", "Change Word Source", "Quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(selections)
        .interact()
        .unwrap();
    match selection {
        0 => game.start(config)?,
        1 => todo!(),
        // 2 => change_word_src(game)?,
        3 => return Err("exit".into()),
        _ => unreachable!(),
    }
    Ok(())
}
