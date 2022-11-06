use crate::constants;
use colored::*;

pub fn print(msg: String) {
    eprintln!("⚠️  {}", "Error".bold());
    eprintln!("{}", msg.red());
}

pub fn print_with_docs(msg: String) {
    print(msg);
    eprintln!(
        "{}",
        format!("Please check the usage here: {}", constants::DOCS_URL)
    );
}
