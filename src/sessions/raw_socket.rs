// #![allow(unused_imports)]
// #![allow(dead_code)]
use std::{fmt, io};
use crate::sessions::{Rotation, SESSION};
use std::fmt::Display;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

/*
RAW_SOCKET

TODO:
    * Add Validation for incoming sessions IP addresses
    * Find a way to drop individual sessions vs all session vs close out listener

Internal logic sanity out line.
setting up socket:
    create listener on port that writes incoming streams to vector until it is filled then based on
    the rotation rules drop the appropriate session.
Sending commands:
    use the first session in the vector to send commands. print output to stdout. Future idea, write
    it to log file.
Close:
    drop all sessions and handles and release the listeners.

*/

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
#[derive(Debug)]
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
    fn start_listener(&mut self, port:u32, rotation: Rotation) {
        let listener = std::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port)).unwrap();
        println!("Started listener on port {}", port);

        let max = self.max_session_count;
        let curr_stream = Arc::clone(&self.connected_streams);
        let _handle = std::thread::spawn(move || {
            for incoming_stream in listener.incoming(){
                match incoming_stream {
                    Ok(incoming_stream) => {
                        let address_info = incoming_stream.peer_addr().unwrap().to_string();
                        println!("[!] Connection received on {}, via NetCat from {}",
                                 port,
                                 address_info);

                        let mut shared_stream = curr_stream.lock().unwrap();
                        let vec_len = shared_stream.deref().len();
                        if vec_len <= max {
                            //TODO: Clean this up
                            println!("[!] NOTICE: Queue Not Full No Rotation");
                        }else{
                            match rotation {
                                Rotation::FIFO => {
                                    println!("[!] FIFO:Dropping First Session");
                                    let rm_sess = shared_stream.deref_mut().remove(0);
                                    rm_sess.stream.shutdown(Shutdown::Both);
                                }
                                Rotation::LIFO => {
                                    println!("[!] FIFO:Dropping First Session");
                                    let rm_sess = shared_stream.deref_mut().remove(vec_len - 1);
                                    rm_sess.stream.shutdown(Shutdown::Both);
                                }
                                Rotation::HOLD => {
                                    incoming_stream.shutdown(Shutdown::Both);
                                    println!("[!] NOTICE: Dropping incoming session queue full");
                                    continue;
                                }
                            }
                        }
                        //create thread to write incoming data to stream to STDOUT
                        let target_output = RawSocket::pipe_thread(
                            incoming_stream.try_clone().unwrap(),
                            std::io::stdout());
                        //Store the thread handle and the TCPStream in the vector
                        shared_stream.deref_mut().push(RawStream{
                            stream: incoming_stream,
                            target_addr: address_info,
                            recv_time: SystemTime::now(),
                            output_handle: target_output
                        });

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
    fn start(&mut self, rotate: Rotation) {
        //figure out how to pass in rotations
        self.start_listener(self.port, rotate);

    }

    //TODO: CLEANER CLOSE // For dropping a connection
    fn drop(&self) {
        let mut raw_sessions = self.connected_streams.lock().unwrap();
        for session in raw_sessions.deref_mut().iter_mut(){
            //this should kill the thread and shutdown the TCP connections
            session.stream.shutdown(Shutdown::Both).expect("[!] Error: Failed to close Stream");
            //as a precaution dropping thread here. Might cause issue
            //session.output_handle.join();
        }
        //drops all values in the vector. TODO: combine with loop above
        raw_sessions.deref_mut().clear();

    }

    fn close(&self) {
        todo!()
    }

    fn send_command(&self,cmd: String){
        let mut raw_sessions = self.connected_streams.lock().unwrap();
        match raw_sessions.deref_mut().first_mut(){
            None => {
                println!("[!] Warning: There are no sessions available to send the command to.")
            }
            Some(raw_stream) => {
                raw_stream.stream.write(cmd.as_bytes()).expect("[!] Error: Could not write to socket");
            }
        }
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_info(&self) {
        todo!()
    }
}