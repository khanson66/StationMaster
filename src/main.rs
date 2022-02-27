// #![allow(unused_imports)]
// #![allow(dead_code)]
mod sessions;
mod cmd;
mod session_handler;
use crate::sessions::SESSION;

use std::io::Write;
use std::process::exit;
use crate::cmd::help;
use crate::session_handler::SessionHandler;
use crate::sessions::netcat::Netcat;
use std::collections::HashMap; 

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

fn port_existance(map: &HashMap<String, u32>, value: u32) -> bool {
    for (name, port) in map.iter(){
        if port == &value {return true}
    }
    return false;
}

fn remove_port(map: &mut HashMap<String, u32>, port: u32){
    let tmp = map.clone(); 
    let deletion = tmp
        .iter()
        .filter(|&(_,&v)| v == port)
        .map(|(k, _)|k);
    for k in deletion{map.remove(k);}
}

// fn test_command(args:Vec<String>){
//     if args.is_empty() {
//         println!("NO ARGS PROVIDED")
//     }else{
//         println!("{}",args.join(","));
//     }

// }

fn main() {
    //TODO: Implement Session Handler Object This object that will get passed to functions

    println!("[!] Creating Session Handler ");
    let mut used_value = HashMap::new();
    cmd::help();
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
            "create" =>{
       
            if args.len()==3{ 
                let name= args[1].to_string().clone();
                let port= args[2].parse::<u32>().unwrap().clone();
            if used_value.contains_key(&name){
                println!("name unavailable")}
            else if port_existance(&used_value, port){
                println!("port is unavailable")}
            else {
                used_value.insert(name,port);
                }
            }
                //maybe put this elsewhere 
                //it separated with the database system currently, maybe we can incorporate them.
                cmd::create(&mut handler, args);   
            },
            // "test" =>{
            //   test_command(args);
            // },
            _ =>{
                println!("There is no such command");
            }
        }
    }
}
