mod ssh;
mod netcat;

trait SESSION {
    fn new(hostname:String, port:i32) -> Self;
    fn close(&self);
    fn send_command(cmd:String) -> String;
}

//TODO: Research way to add impl to seprate files
impl SESSION for netcat::Netcat{
    fn new(hostname: String, port: i32) -> Self {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn send_command(cmd: String) -> String {
        todo!()
    }
}

impl SESSION for ssh::SSH{
    fn new(hostname: String, port: i32) -> Self {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn send_command(cmd: String) -> String {
        todo!()
    }
}

pub(crate) struct SessionHandler{
    session_store: Box<dyn SESSION>,
}

impl SessionHandler{

    pub fn new() -> Self{
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