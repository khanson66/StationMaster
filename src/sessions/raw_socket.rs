// #![allow(unused_imports)]
// #![allow(dead_code)]
use std::{fmt, io};
use crate::sessions::SESSION;
use std::fmt::Display;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::Deref;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct RawSocket {
    port:u32,
    name: String,
    description:String,
    stream: Arc<Mutex<Option<TcpStream>>>
}

impl RawSocket {

    pub(crate) fn new(name: String, port: u32) -> Self {
        RawSocket {
            port,
            name,
            description: "Raw Socket connection".to_string(),
            stream: Arc::new(Mutex::new(None)),
        }
    }


    //Starts listening to port selected
    //TODO: move to centralized checking model??
    fn start_listener(&mut self, port:u32) {
        let listener = std::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port)).unwrap();
        println!("Started listener on port {}", port);
        //return listener;

        let curr_stream = Arc::clone(&self.stream);
        let handle = std::thread::spawn(move || {
            //let (mut stream, _) = listener.accept()?;
            for incoming_stream  in listener.incoming(){
                match incoming_stream {
                    Ok(incoming_stream) => {
                        println!("[!] Connection received on {}, via NetCat from {}",
                                 port,
                                 incoming_stream.peer_addr().unwrap());

                        let mut shared_stream = curr_stream.lock().unwrap();
                        match *shared_stream {
                            None => {
                                RawSocket::pipe_thread(incoming_stream.try_clone().unwrap(), std::io::stdout());
                                *shared_stream = Option::from(incoming_stream);
                            }
                            Some(_) => {
                                println!("[!] Dropping Connection: Queue Full")
                            }
                        }

                    }
                    Err(e) => { /* connection failed */ }
                }
            }
            //let _t =
            //self.stream = Option::from(stream);
        });


        /*
        //TODO: Split this off eventually to separate function to handle multiple incoming streams
        let (mut stream, _) = listener.accept()?;
        println!("Connection received on {}, via NetCat from {}",port, stream.peer_addr().unwrap());

        let _t = RawSocket::pipe_thread(stream.try_clone().unwrap(), std::io::stdout());
        self.stream = Option::from(stream);

        return Ok(());
        
         */
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
                    //std::process::exit(0x0100);
                    return;
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
    fn close(&self) {
        let mut stream =  self.stream.lock().unwrap();
        match *stream {
            None => {
                println!("No sessions currently connected to drop");
            }
            Some(ref mut s) => {
                println!("[!] Dropping Current Active Session");
                s.shutdown(Shutdown::Both).expect("shutdown call failed");
                *stream = Option::None;
                println!("[!] Successfully Closed");
            }
        }
    }

    fn send_command(&self,cmd: String){
        let cmd = cmd.clone() + "\n";
        let mut stream_opt =  self.stream.lock().unwrap();
        match *stream_opt {
            None => {
                println!("No sessions currently connected");
            }
            Some(ref mut s) => {
                s.write(cmd.as_bytes()).expect("Failed to send TCP.");
            }
        }

    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}