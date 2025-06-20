use crackle::setup;
use dotenv::dotenv;
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_loaded = dotenv().ok();
    if env_loaded == None {
        println!("Failed to load environment variables");
        std::process::exit(1);
    }

    let db_exists = fs::metadata("crackle.db").is_ok();
    if !db_exists {
        setup::setup()?;
    }

    Ok(())
}
