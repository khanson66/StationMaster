use std::{fmt, io};
use crate::sessions::SESSION;
use std::{sync, thread, time};
use std::fmt::Display;
use std::fs::read;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub struct Netcat{
    port:u32,
    handle:Option<thread::JoinHandle<()>>,
    name: String,
    description:String,
    //TODO: Find a better way of implementing this. Does not feel right but might work
    tx_command: Sender<String>,
    rx_command: Arc<Mutex<Receiver<String>>>,
}

impl Netcat {

    pub(crate) fn new(name: String, port: u32) -> Self {
        let (tx_command, rx_command) = channel();
        Netcat {
            port,
            handle: None,
            name,
            description: "Netcat connection".to_string(),
            tx_command,
            rx_command: Arc::new(Mutex::new(rx_command)),
        }
    }


    //Starts listening to port selected
    fn start_listener(&self, rx_command: Arc<Mutex<Receiver<String>>>, port:u32) -> io::Result<()>{
        let listener = std::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port))?;
        println!("Started listener on port {}", port);

        //TODO: Split this off eventually to separate function to handle multiple incoming streams
        let (mut stream, _) = listener.accept()?;
        println!("Connection received on {}, via NetCat from {}",port, stream.peer_addr().unwrap());

        let _t = Netcat::pipe_thread(stream.try_clone().unwrap(), std::io::stdout());

        loop {
            //execution is paused until a command a string is sent
            let readline = rx_command.lock().unwrap().recv();
            //TODO: Better solution for killing thread
            if readline==Ok("<>_kill".to_string()) {
             break;
            }
            match readline {
                Ok(command) => {
                    // Clone command to increase its lifetime
                    let command = command.clone() + "\n";

                    // Send a TCP message
                    stream
                        .write(command.as_bytes())
                        .expect("Failed to send TCP.");
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

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

impl fmt::Display for Netcat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, host/port: {}:{}", self.name, "localhost",self.port)
    }
}

impl SESSION for Netcat{
    fn start(&mut self) {
        let t = Arc::clone(&self.rx_command);
        self.handle = Some(thread::spawn(move|| {
            self.start_listener(t, self.port);
        }));
    }

    //TODO: CLEANER CLOSE
    fn close(&mut self) {
        self.tx_command.send("<>_kill".to_string()).unwrap();
    }

    fn send_command(&self,cmd: String){
        self.tx_command.send(cmd).unwrap();
    }

    fn get_info() {
        todo!()
    }
}