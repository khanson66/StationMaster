use std::fmt;
use std::fmt::write;
use crate::sessions::SESSION;

#[derive(Debug)]
pub struct SSH{
    hostname:String,
    port:i32,
    pid:i32,
    name:String,
    description:String,
    username:String,
    password:String,
}

impl fmt::Display for SSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, host/port: {}:{}", self.name, self.hostname,self.port)
    }
}

impl SESSION for SSH{
    fn new(hostname: String, port: i32) -> Self {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn send_command(cmd: String) -> String {
        todo!()
    }

    fn get_info() {
        todo!()
    }
}