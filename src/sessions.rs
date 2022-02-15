mod ssh;
mod netcat;

trait base_session{
 //   fn new(hostname:String, port:i32) -> Self;
    fn close(&self);
    fn send_command(&self, cmd:String) -> String;
}
#[warn(dead_code)]
//TODO: Research way to add impl to seprate files
impl base_session for netcat::Netcat{
    // fn new(hostname: String, port: i32) -> Self {
    //     todo!()
    // }

    fn close(&self) {
        todo!()
    }

    fn send_command(&self, cmd: String) -> String {
        todo!()
    }
}
impl netcat::Netcat { 
    fn new(hostname: String, port: i32) -> Self{
        todo!()
    }
}
impl base_session for ssh::SSH{
    // fn new(hostname: String, port: i32) -> Self {
    //     todo!()
    // }

    fn close(&self) {
        todo!()
    }

    fn send_command(&self, cmd: String) -> String {
        todo!()
    }
}

pub(crate) struct SessionHandler{
    sessionhandler: Box<dyn base_session>
}

//pub struct SessionHandler; 

impl SessionHandler{

   fn new() -> Self{
        // Self{
        // sessionhandler: Box::new()
        // }    
        todo!()
    }
    //TODO: Create method to list all sessions
    pub fn list(&self) -> String{
        todo!()
    }

    //TODO: Create a method to create new session return bool
    pub fn create(&mut self, SessionType:String) -> bool{
        todo!()
    }
}