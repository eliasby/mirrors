use crate::constants;
use crate::error;
use serde_json::Value;
use std::path::Path;
use std::{fs, process};

pub struct Config {
    pub primary: String,
    pub mirrors: Vec<String>,
}

pub fn read_config_or_fail() -> Config {
    let mut config = Config {
        primary: String::from(""),
        mirrors: Vec::new(),
    };

    let path = Path::new(".").join(constants::CONFIG_FILENAME);
    if !path.exists() {
        error::print_with_docs(format!(
            "The file {} does not exist in the current directory.",
            constants::CONFIG_FILENAME
        ));
        process::exit(1);
    }
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => {
            error::print(format!(
                "Failed to read the content of {}.",
                constants::CONFIG_FILENAME
            ));
            error::print(format!("{}", e));
            process::exit(1);
        }
    };
    let json: Value = match serde_json::from_str::<Value>(&file) {
        Ok(json) => json,
        Err(e) => {
            error::print(format!(
                "Failed to read the content of {}.",
                constants::CONFIG_FILENAME
            ));
            error::print(format!("{}", e));
            process::exit(1);
        }
    };
    match json["primary"].as_str() {
        Some(primary) => {
            config.primary = String::from(primary);
        }
        None => {
            error::print_with_docs(format!(
                "{} does not contain a \"primary\" field.",
                constants::CONFIG_FILENAME
            ));
            process::exit(1);
        }
    };
    match json["mirrors"].as_array() {
        Some(mirrors) => {
            for mirror in mirrors.iter() {
                match mirror.as_str() {
                    Some(mirror) => {
                        config.mirrors.push(String::from(mirror));
                    }
                    None => {
                        error::print(format!("Failed to read mirror."));
                        process::exit(1);
                    }
                }
            }
        }
        None => {
            error::print_with_docs(format!(
                "{} does not contain a \"mirrors\" field.",
                constants::CONFIG_FILENAME
            ));
            process::exit(1);
        }
    };

    config
}
