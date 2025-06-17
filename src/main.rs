use crackle::database;
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_exists = fs::metadata("crackle.db").is_ok();

    let db = database::DB::new()?;
    if !db_exists {
        db.setup()?;
        println!("Database created and set up successfully!");
    } else {
        println!("Connected to existing database!");
    }
    Ok(())
}
