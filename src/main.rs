use colored::*;
use serde_json::Value;
use std::fs;
use std::process::Command;

fn main() {
    let file = fs::read_to_string("./mirrors.json");
    let json: Value = serde_json::from_str(&file.unwrap()).unwrap();

    Command::new("git").arg("remote").arg("rm").arg("origin");
    println!("Removed origin.");

    let primary = json["primary"].as_str().unwrap();
    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(primary);
    println!("Added primary 🔗 {}", primary.cyan());
    Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("--add")
        .arg("--push")
        .arg("origin")
        .arg(primary);

    let mirrors = json["mirrors"].as_array().unwrap();
    for mirror in mirrors.iter() {
        let value = mirror.as_str().unwrap();
        Command::new("git")
            .arg("remote")
            .arg("set-url")
            .arg("--add")
            .arg("--push")
            .arg("origin")
            .arg(value);
        println!("Added mirror  🔗 {}", value.magenta());
    }

    println!("Success! 🎉")
}
