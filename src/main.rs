mod sessions;
mod cmd;
mod session_handler;
use crate::sessions::SESSION;

use std::io::Write;
use std::process::exit;
use crate::cmd::help;
use crate::session_handler::SessionHandler;
use crate::sessions::raw_socket::RawSocket;

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

fn test_command(args:Vec<String>){
    if args.is_empty() {
        println!("NO ARGS PROVIDED")
    }else{
        println!("{}",args.join(","));
    }

}

fn main() {
    //TODO: Implement Session Handler Object This object that will get passed to functions

    println!("[!] Creating Session Handler ");
    let mut handler: SessionHandler = session_handler::SessionHandler::new();

    loop{
        //TODO: Add formatted time to prompt
        let (command,args) = prompt("> ");

        // For debugging commands
        println!("[dbg] cmd: {} args: {}",command,args.join(","));

        match command.as_str() {
            "help" => {
                cmd::help();
            },
            "exit" =>{
                exit(0);
            },
            "ls" => {
                handler.list_sessions();
            },
            "select" =>{

            }
            "create" =>{
                cmd::create(&mut handler, args);
            },
            "test" =>{
                test_command(args);
            },
            _ =>{
                println!("There is no such command");
            }
        }
    }
}
