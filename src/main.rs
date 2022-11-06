mod config;
mod constants;
mod error;

use colored::*;
use std::process;
use std::process::Command;

fn main() {
    let config = config::read_config_or_fail();

    match Command::new("git")
        .args(["remote", "rm", "origin"])
        .output()
    {
        Ok(_) => {}
        Err(e) => {
            error::print(format!("Failed to remove origin."));
            error::print(format!("{}", e));
            process::exit(1);
        }
    }

    match Command::new("git")
        .args(["remote", "add", "origin", &config.primary])
        .output()
    {
        Ok(_) => {}
        Err(e) => {
            error::print(format!("Failed to add primary {}.", &config.primary));
            error::print(format!("{}", e));
            process::exit(1);
        }
    }
    match Command::new("git")
        .args([
            "remote",
            "set-url",
            "--add",
            "--push",
            "origin",
            &config.primary,
        ])
        .output()
    {
        Ok(_) => {}
        Err(e) => {
            error::print(format!("Failed to add primary {}.", &config.primary));
            error::print(format!("{}", e));
            process::exit(1);
        }
    }
    println!("🔗 Added primary {}", &config.primary.cyan());

    for mirror in config.mirrors.iter() {
        match Command::new("git")
            .args(["remote", "set-url", "--add", "--push", "origin", mirror])
            .output()
        {
            Ok(_) => println!("🔗 Added mirror  {}", mirror.magenta()),
            Err(e) => {
                error::print(format!("Failed to add mirror {}.", mirror));
                error::print(format!("{}", e));
            }
        }
    }

    match Command::new("git").args(["remote", "-v"]).output() {
        Ok(output) => {
            println!("\n📄 Summary ");
            println!("{}", String::from_utf8(output.stdout).unwrap());
        }
        Err(e) => {
            error::print(format!("Failed to print summary."));
            error::print(format!("{}", e));
            process::exit(1);
        }
    }
    println!("🎉 {}", "Success!".bold());
}
