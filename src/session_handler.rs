use crate::sessions::raw_socket::RawSocket;
use crate::sessions::{Rotation, SESSION};

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
                    _ => println!("WTF happened")
                }
            }
        }
    }
    
    pub fn get_used_names(&self) ->Vec<String> {
        let mut names : Vec<String> = vec![];
        for i in &self.sessions {
            match i {
                SessionTypes::RawSocket(s) => {
                    names.push(s.get_name());
                }
            }
        }
        return names
    }

    //TODO: Create a method to create new session return bool
    pub fn create(&mut self, session: String, hostname: String, port: u32) -> bool {
        return match session.to_lowercase().as_str() {
            "MEME" => {
                println!("OMEGALUL BRUV WTF YOU DOING?");
                true
            }
            "netcat" => {
                let mut socket = RawSocket::new(hostname, port, 3);
                // place holder until we have the logic set up
                socket.start(Rotation::HOLD);
                self.sessions
                    .push(SessionTypes::RawSocket(socket));
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
            println!("There are no active connection");
        } else {
            for i in 0..self.sessions.len() {
                let si = self.sessions.get(i).unwrap();
                match si {
                    SessionTypes::RawSocket(s) => {
                        if name == s.get_name(){

                            let temp =
                                Option::from(self.sessions.remove(i));

                            match &self.active_session{
                                Some(_t) => {
                                    let old_active_session =
                                        std::mem::replace(&mut self.active_session, temp);

                                    self.sessions.push(old_active_session.unwrap());
                                }
                                None => {
                                    self.active_session = temp;
                                }

                            }
                            return;
                        }
                    }
                }
            }
            println!("[!] Cant set active connection because one does not exist");
        }
    }

    pub(crate) fn send_command(&self, cmd: String){

        match &self.active_session {
            Some(session) => {
                match session {
                    SessionTypes::RawSocket(s) => {
                        s.send_command(cmd);
                    }
                }
            }

            None => {
                println!("[!] There is no active connection. Please set one as active to use this command")
            }
        }
    }

    pub(crate) fn drop(&self, name: String){
        match &self.active_session {
            Some(session) => {
                match session {
                    SessionTypes::RawSocket(s) => {
                        s.drop()
                    }
                }
            }

            None => {
                println!("[!] There is no active session. Please set one as active to use this command")
            }
        }
    }

    pub(crate) fn close(&self){
        let used_name = self.get_used_names();
        for names in used_name{
            self.drop(names)
        } 
    } 
}
