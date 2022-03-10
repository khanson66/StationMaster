// #![allow(unused_imports)]
// #![allow(dead_code)]
use std::{fmt, io};
use crate::sessions::SESSION;
use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;


#[derive(Debug)]
pub struct RawSocket {
    port:u32,
    name: String,
    description:String,
    stream: Option<TcpStream>
}

impl RawSocket {

    pub(crate) fn new(name: String, port: u32) -> Self {
        RawSocket {
            port,
            name,
            description: "Raw Socket connection".to_string(),
            stream: None,
        }
    }


    //Starts listening to port selected
    fn start_listener(&mut self, port:u32) -> io::Result<()>{
        let listener = std::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port))?;
        println!("Started listener on port {}", port);

        //TODO: Split this off eventually to separate function to handle multiple incoming streams
        let (mut stream, _) = listener.accept()?;
        println!("Connection received on {}, via NetCat from {}",port, stream.peer_addr().unwrap());

        let _t = RawSocket::pipe_thread(stream.try_clone().unwrap(), std::io::stdout());
        self.stream = Option::from(stream);

        return Ok(());
    }

    //handle the web connection??
    fn pipe_thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
        where
            R: std::io::Read + Send + 'static,
            W: std::io::Write + Send + 'static,
    {
        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                let len = r.read(&mut buffer).unwrap();
                if len == 0 {
                    println!("Connection lost");
                    std::process::exit(0x0100);
                }
                w.write(&buffer[..len]).unwrap();
                w.flush().unwrap();
            }
        })
    }
}

impl Display for RawSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, host/port: {}:{}", self.name, "localhost",self.port)
    }
    
}

impl SESSION for RawSocket {
    fn start(&mut self) {
        self.start_listener(self.port);
    }

    //TODO: CLEANER CLOSE
    fn close(&mut self) {

    }

    fn send_command(&self,cmd: String){
        let cmd = cmd.clone() + "\n";
        self.stream.as_ref().unwrap()
            .write(cmd.as_bytes())
            .expect("Failed to send TCP.");
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}