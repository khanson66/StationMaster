use std::fmt;
use std::thread::current;
use crate::SESSION;
use crate::sessions::netcat::Netcat;
use crate::sessions::ssh::SSH;

pub(crate) struct SessionHandler{
        curr_sessions: Vec<SessionTypes>
}

#[derive(Debug)]
pub enum SessionTypes{
    Netcat(Netcat),
    SSH(SSH),
}

impl SessionHandler{
    
    pub fn new() -> SessionHandler {
        SessionHandler{
            curr_sessions: vec![]
        }
    }

    pub fn list_session_types() {
        todo!()
    }

    //TODO: Create method to list all sessions
    pub fn list_sessions(&self){
        if self.curr_sessions.len() < 0 {
            println!("There are no active sessions");
        }else{
            for i in  &self.curr_sessions{
                println!("{:?}",i);
            }
        }
    }

    //TODO: Create a method to create new session return bool
    pub fn create(&mut self, Session:String, hostname:String, port:i32) -> bool{
        match Session.to_lowercase().as_str(){
            "ssh" => {
                todo!()
            }
            "netcat" => {
                self.curr_sessions.push(SessionTypes::Netcat(Netcat::new(hostname,port)));
                return true;
            }
            _ =>{
                println!("{} DOES NOT EXIST AS A SESSION TYPE", SessionType);
                return false;
            }
        }
    }
}