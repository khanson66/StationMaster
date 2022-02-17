use std::fmt;
use crate::sessions::SESSION;

#[derive(Debug)]
pub struct Netcat{
    pub(crate) hostname: String,
    pub(crate) port:i32,
    pid:i32,
    pub(crate) name: String,
    pub(crate)description:String
}

impl fmt::Display for Netcat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, host/port: {}:{}", self.name, self.hostname,self.port)
    }
}

impl SESSION for Netcat{
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