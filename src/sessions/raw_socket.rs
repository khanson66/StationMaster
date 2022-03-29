// #![allow(unused_imports)]
// #![allow(dead_code)]
use std::{fmt, io};
use crate::sessions::SESSION;
use std::fmt::Display;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

// different types of rotations
pub enum Rotation{
    FIFO,
    LIFO,
}
/*
RawSocket:
    port: the port to open on the host machine to accept connections
    name: the name of the connection, Unique identifier
    description: optional quick description of function
    max_session_count: How many concurrent session are allowed to be connected
    connected_streams: vector of connected streams and relevant metadata
*/
#[derive(Debug)]
pub struct RawSocket {
    port:u32,
    name: String,
    description:String,
    max_session_count: usize,
    connected_streams: Arc<Mutex<Vec<RawStream>>>
}
/*
RawStream:
    stream: The TCPStream object
    target_addr: IP:PORT of connected session
    recv_time: epoch time that the session was retrieved
    output_handle: Handle to thread that is reading the stream for incoming data
*/
struct RawStream{
    stream: TcpStream,
    target_addr: String,
    recv_time: SystemTime,
    output_handle: JoinHandle<()>,
} 

impl RawSocket {

    pub(crate) fn new(name: String, port: u32, count: usize) -> Self {
        RawSocket {
            port,
            name,
            max_session_count:count,
            description: "Raw Socket connection".to_string(),
            connected_streams: Arc::new(Mutex::new(Vec::new())),
        }
    }


    //Starts listening to port selected
    //TODO: move to centralized checking model??
    fn start_listener(&mut self, port:u32) {
        let listener = std::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port)).unwrap();
        println!("Started listener on port {}", port);

        let max = self.max_session_count;
        let curr_stream = Arc::clone(&self.connected_streams);
        let _handle = std::thread::spawn(move || {
            for incoming_stream  in listener.incoming(){
                match incoming_stream {
                    Ok(incoming_stream) => {
                        println!("[!] Connection received on {}, via NetCat from {}",
                                 port,
                                 incoming_stream.peer_addr().unwrap());

                        let mut shared_stream = curr_stream.lock().unwrap();
                        if *shared_stream.len() < max{
                            let target_output = RawSocket::pipe_thread(
                                incoming_stream.try_clone().unwrap(),
                                std::io::stdout());
                            *shared_stream.push(RawStream{
                                stream: incoming_stream,
                                target_addr: incoming_stream.peer_addr().unwrap().to_string(),
                                recv_time: SystemTime::now(),
                                output_handle: target_output
                            });
                        }else{
                            println!("[!] Dropping Connection: Queue Full")
                        }

                    }
                    Err(e) => { /* connection failed */ }
                }
            }
        });
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
        let mut stream =  self.connected_streams.lock().unwrap();
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
        let mut stream_opt =  self.connected_streams.lock().unwrap();
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