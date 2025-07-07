use crackle::{
    database, game_loop,
    setup::{self, change_word_src},
};
use dialoguer::{Select, theme::ColorfulTheme};
use dotenv::dotenv;
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_loaded = dotenv().ok();
    if Option::is_none(&env_loaded) {
        println!("Failed to load environment variables");
        std::process::exit(1);
    }

    let db_exists = fs::metadata("crackle.db").is_ok();

    let db = if db_exists {
        database::DB::new()?
    } else {
        setup::setup()?
    };
    let mut game = game_loop::GameLoop::new(db);
    loop {
        let err = menu(&mut game);
        if let Err(err) = err {
            println!("Error in menu: {err}");
            break;
        }
    }

    Ok(())
}

fn menu(game: &mut game_loop::GameLoop) -> Result<(), Box<dyn std::error::Error>> {
    let selections = &["Play", "Generate Report", "Quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(selections)
        .interact()
        .unwrap();
    match selection {
        0 => game.start()?,
        1 => todo!(),
        2 => change_word_src(game)?,
        3 => return Err("exit".into()),
        _ => unreachable!(),
    }
    Ok(())
}
