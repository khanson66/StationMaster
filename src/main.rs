mod cmd;

use std::time::{SystemTime};
use std::io::Write;

fn prompt(name:&str) -> (String, Vec<String>) {
    let mut line = String::new();
    print!("{}", name);

    // Clean console up
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    let mut parts = line.trim().split_whitespace();
    let command = parts.next().unwrap().to_string();
    let args = parts;

    return (command, args )
}

fn main() {
    loop{
        //TODO: Add formatted time to prompt
        let input = prompt("> ");
        println!("you put in {}",input);

    }
}
