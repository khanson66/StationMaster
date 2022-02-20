use crate::prompt;
use crate::SessionHandler;



pub fn help() {
    println!("
    StationMaster -- Simple program to deploy and manage remote shells
    
    USAGE:
        StationMaster[EXE] [OPTIONS]
    
    OPTIONS:
        
    ")
}

pub(crate) fn create(session_handler:&SessionHandler, mut args:Vec<String>) {
    // let sessionType = args.pop();
    let create_help  = "USAGE:
        create <OPTIONS>
    
    OPTIONS:
        -s, --ssh       Utilize a ssh reverse shell
        -nc, --netcat   Utilize a ssh netcat shell";

    //println!("{}",args[args.len()-1]);
    if args.is_empty(){
        println!("{}",create_help);
    } else {
    match args[0].to_lowercase().as_str(){
        "-s" | "-ssh" => {
            //println!("you are chosing ssh");
            let ssh_usage = "USAGE:
            create -ssh -h <hostname> -p <port>";
            let mut hostname : String = "default".to_string(); 
            let mut port: i32 = 9090;
            //println!("{}",args.len());
            if args.len()< 3 {println!("{}", ssh_usage)}
            else if args[2].is_empty() | args[4].is_empty(){
                println!("{}", ssh_usage)
            }else{
                if args[1]=="-h"{
                    hostname = args[2].to_lowercase();
                }else {println!("{}", ssh_usage)}
                if args[3]=="-p"{
                    port = args[4].parse::<i32>().unwrap();
                }else{println!("{}", ssh_usage)}
            }
            println!("hostname: {} & port: {}", hostname, port);
            // TODO more input validation for insufficient inputs. 
        },
        "-nc" | "-netcat" => {
            println!("you are chosing netcat");
            let netcat_usage = "USAGE:
            create -netcat -n <name> -p <port>";
            let mut name : String = "default".to_string(); 
            let mut port: i32 = 9090;
            if args.len()< 3 {println!("{}", netcat_usage)}
            else if args[2].is_empty() | args[4].is_empty(){
                println!("{}", netcat_usage)
            }else{
                if args[1]=="-h"{
                    name = args[2].to_lowercase();
                }else {println!("{}", netcat_usage)}
                if args[3]=="-p"{
                    port = args[4].parse::<i32>().unwrap();
                }else{println!("{}", netcat_usage)}
            }
            println!("name: {} & port: {}", name, port);
           
        },
        _ => {println!("{}",create_help)}
        }
    }
}