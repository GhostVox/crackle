use dirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub word_list_path: String,
    pub starting_word_limit: u32,
    pub app_db: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            word_list_path: "words.txt".to_string(),
            starting_word_limit: 10,
            app_db: dirs::config_dir()
                .unwrap()
                .join("crackle")
                .join("crackle.db"),
        }
    }
}

impl Config {
    pub fn new(word_list_path: String, starting_word_limit: u32) -> Self {
        Config {
            word_list_path,
            starting_word_limit,
            app_db: dirs::config_dir()
                .unwrap()
                .join("crackle")
                .join("crackle.db"),
        }
    }

    fn update_app_db(&mut self, app_db: PathBuf) {
        self.app_db = app_db;
    }
}

pub fn get_config() -> Config {
    let mut config_path = dirs::config_dir()
        .ok_or("Could not find config directory")
        .unwrap();

    config_path.push("crackle");
    config_path.push("config.toml");

    let mut config = match std::fs::read_to_string(config_path.clone()) {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|_| create_config(&config_path)),
        Err(_) => create_config(&config_path),
    };
    let expected_db_path = config_path.parent().unwrap().join("crackle.db");
    if config.app_db != expected_db_path {
        config.update_app_db(expected_db_path);
    }
    config
}

fn create_config(config_path: &PathBuf) -> Config {
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    println!("Created config file at: {}", config_path.display());
    println!("You can edit to customize your settings.");

    let mut config = Config::default();

    config.update_app_db(
        config_path
            .parent()
            .unwrap()
            .to_path_buf()
            .join("crackle.db"),
    );
    let config_str = toml::to_string(&config).unwrap();

    std::fs::write(config_path, config_str).unwrap();
    config
}
