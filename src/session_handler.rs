use crate::sessions::raw_socket::RawSocket;
use crate::SESSION;
use std::fmt;
use std::fs::OpenOptions;
use std::ops::Deref;
use std::thread::current;

pub(crate) struct SessionHandler {
    sessions: Vec<SessionTypes>,
    //NOTE: needed to simplify the duplicate check and positioning.
    names: Vec<String>, //TODO: Find a way to remove this
    active_session: Option<SessionTypes>
}

#[derive(Debug)]
pub enum SessionTypes {
    RawSocket(RawSocket),
}

impl SessionHandler {
    pub fn new() -> SessionHandler {
        SessionHandler {
            sessions: vec![],
            names: vec![],
            active_session: None,
        }
    }

    pub fn list_session_types() {
        todo!()
    }

    //TODO: Create method to list all sessions
    pub fn list_sessions(&self) {
        if self.sessions.len() <= 0 && self.active_session.is_none() {
            println!("There are no active sessions");
        } else {
            if self.active_session.is_some(){
                match self.active_session.as_ref().unwrap(){
                    SessionTypes::RawSocket(s) => println!("{:?}", s.to_string()),
                };
            }
            for i in &self.sessions {
                match i {
                    SessionTypes::RawSocket(s) => println!("{:?}", s.to_string()),
                    _ => println!("WTF happened"),
                }
            }
        }
    }

    //TODO: Create a method to create new session return bool
    pub fn create(&mut self, session: String, hostname: String, port: u32) -> bool {
        return match session.to_lowercase().as_str() {
            "MEME" => {
                println!("OMEGALUL BRUV WTF YOU DOING?");
                true
            }
            "netcat" => {

                self.sessions
                    .push(SessionTypes::RawSocket(RawSocket::new(hostname, port)));
                true
            }
            _ => {
                println!("{} DOES NOT EXIST AS A SESSION TYPE", session);
                false
            }
        };
    }

    pub fn set_active_session(&mut self, name: String) {
        if self.sessions.len() <= 0 && self.active_session.is_none() {
            println!("There are no active sessions");
        } else {
            for i in 0..self.sessions.len() {
                let si = self.sessions.get(i).unwrap();
                match si {
                    SessionTypes::RawSocket(s) => {
                        if name == s.get_name(){

                            let temp =
                                Option::from(self.sessions.remove(i));

                            match &self.active_session{
                                Some(T) => {
                                    let old_active_session =
                                        std::mem::replace(&mut self.active_session, temp);

                                    self.sessions.push(old_active_session.unwrap());
                                }
                                None => {
                                    //Place holder incase logic is needed
                                    todo!()
                                }

                            }
                            return;
                        }
                    }
                }
            }
            println!("[!] Cant set active session because one does not exist");
        }
    }

    pub(crate) fn send_command(&self, cmd: String){
        match &self.active_session.as_ref().unwrap() {
            SessionTypes::RawSocket(s) => {
                s.send_command(cmd);
            }
        }
    }
}
