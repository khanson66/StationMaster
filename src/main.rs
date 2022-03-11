mod sessions;
mod cmd;
mod session_handler;
use crate::sessions::SESSION;

use std::io::Write;
use std::process::exit;
use crate::session_handler::SessionHandler;
use clearscreen;

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
    //TODO: Implement Session Handler Object This object that will get passed to functions

    println!("[!] Creating Session Handler ");
    cmd::help();
    let mut handler: SessionHandler = session_handler::SessionHandler::new();

    loop{
        //TODO: Add formatted time to prompt
        let (command,args) = prompt("> ");

        // For debugging commands
        println!("[dbg] cmd: {} args: {}",command,args.join(","));

        match command.as_str() {
            "help" => {
                if args.is_empty(){cmd::help();}
                else {cmd::help_extension(args)}
            },
            "exit" =>{
                println!("Good Bye");
                exit(0);
            },
            "ls" => {
              handler.list_sessions();
            },
            "create" =>{
                cmd::create(&mut handler, args);
            },
            "drop" =>{
                println!("Dropping a active session");
                cmd::drop(&mut handler, args); 
            },
            "clear" =>{
              clearscreen::clear().unwrap();
            },
            "cmd" =>{
                cmd::cmd(&mut handler, args);
            },
            "activate" =>{
                cmd::activate(&mut handler, args)
            },
             _ =>{
                println!("There is no such command");
            }
        }
    }
}
