// #![allow(unused_imports)]
// #![allow(dead_code)]
use crate::SessionHandler;
mod utility;


pub fn help() {
    println!("{}
    StationMaster -- Simple program to deploy and manage remote shells
    
    USAGE:
        [Command] <Arguments>
    
    OPTIONS:
        help    --- Obtain Help message
        exit    --- exit application safely 
        ls      --- list all active sessions
        create  --- create an new session
        drop    --- drop an active session
        cmd     --- send command to an active session
        
    ",r"
    _____ _        _   _             __  __           _            
   / ____| |      | | (_)           |  \/  |         | |           
  | (___ | |_ __ _| |_ _  ___  _ __ | \  / | __ _ ___| |_ ___ _ __ 
   \___ \| __/ _` | __| |/ _ \| '_ \| |\/| |/ _` / __| __/ _ \ '__|
   ____) | || (_| | |_| | (_) | | | | |  | | (_| \__ \ ||  __/ |   
  |_____/ \__\__,_|\__|_|\___/|_| |_|_|  |_|\__,_|___/\__\___|_|                                                                    
 ")
}
pub fn help_extension(args: Vec<String>){
    let create_usage = "USAGE:
    create <OPTIONS> <name> <port>

    OPTIONS:
    nc, netcat   Utilize a netcat shell";

    match args[0].as_str() {
        "nc" | "netcat" => {println!("{}",create_usage)}, 
        "create" => {println!("{}",create_usage)}, 
        _ => {println!(" please Specific what you need help with.")}
    } 
}
pub(crate) fn create(session_handler: &mut SessionHandler, args:Vec<String>) {
    // let sessionType = args.pop();
    let create_help  = "USAGE:
        create <OPTIONS> <name> <port>
    
    OPTIONS:
        nc, netcat   Utilize a netcat shell";

    //println!("{}",args[args.len()-1]);
    if args.is_empty(){
        println!("{}",create_help);
    } else {
    match args[0].to_lowercase().as_str(){
      
        "nc" | "netcat" => {
            println!("you are chosing netcat");
            let netcat_usage = "USAGE:
            create -netcat <name> <port>";
            let mut name : String = "Netcat_default".to_string(); 
            let mut port: u32 = 4444;
            
            // if args.len()< 3 {println!("{}", netcat_usage)}
            // else if args[2].is_empty() | args[4].is_empty(){
            //     println!("{}", netcat_usage)
            // }else{
            //     if args[1]=="-h"{
            //         name = args[2].to_lowercase().to_string();
            //     }else {println!("{}", netcat_usage)}
            //     if args[3]=="-p"{
            //         port = args[4].parse::<u32>().unwrap();
            //     }else{println!("{}", netcat_usage)}
            // }
            if args.len() < 3{ 
                println!("{}",netcat_usage);
            }else{ 
                name = args[1].to_string();
                port = args[2].parse::<u32>().unwrap();
            }
            //println!("name: {} & port: {}", name, port);
            let used_port = utility::get_used_port();
            if used_port.contains(&port){println!("please use a different port, port is being utilized")}
            else {
            let _out = session_handler.create("netcat".to_string(),name, port);
            }
        },
        _ => {println!("{}",create_help)}
        }
    }
}

//TODO: Add the ability to select specific session when multiple are coming in
pub(crate) fn select(session_handler: &mut SessionHandler, args:Vec<String>){
    todo!();
}

pub(crate) fn drop(Session_handler: &mut SessionHandler, args:Vec<String>){
    todo!();
}