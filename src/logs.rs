use std::io::Write;

pub fn log_session_state(mut state: String) -> Result<(), std::io::Error> {
    let pwd = std::env::current_dir().unwrap();
    let file_path = pwd.join("logs.txt");
    state.push('\n');
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    file.write_all(state.as_bytes())?;
    Ok(())
}
