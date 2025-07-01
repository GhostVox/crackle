use crackle::{database, game_loop, setup};
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
    let err = game.start();
    if let Err(err) = err {
        println!("Error starting game: {err}");
    }

    Ok(())
}
