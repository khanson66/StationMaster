mod cmd;

use std::io::Write;
use std::process::exit;

fn prompt(name:&str) -> (String, Vec<String>) {
    let mut line = String::new();
    print!("{}", name);

    // Clean console up
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    // Command and arg parsing
    let mut parts = line.trim().split(" ");
    let command = parts.next().unwrap().to_string();
    let args = parts.map(|s| s.to_string()).collect();

    return (command, args )
}

fn main() {
    loop{
        //TODO: Add formatted time to prompt
        let (command,args) = prompt("> ");

        // For debugging commands
        println!("[dbg] cmd: {} args: {}",command,args.join(","));

        match command.as_str() {
            "help" => {
                // TODO: implement help function
                println!("THIS IS THE HELP COMMAND")
            },
            "exit" =>{
                exit(0);
            },
            _ =>{
                println!("There is no such command");
            }
        }
    }
}
