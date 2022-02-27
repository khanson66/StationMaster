//easy baby stuff alexs can do
use std::{fmt, io};
use crate::sessions::SESSION;
//straight out of reverseshell 
use rustyline::error::ReadlineError;
use rustyline::Cmd;
use rustyline::Editor;
use rustyline::KeyEvent;
//for convinience
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::Result;

#[derive(Debug)]
//im just playing around with just plain implementation accross the board. ignore the codes below
pub struct Bash{
    port:u32,
    //handle:Option<thread::JoinHandle<()>>,
    name: String,
    //description:String,
    //stream: Option<TcpStream>
}

impl Bash{ 
    fn new(name: String, port: u32) -> self{ 
        todo! 
    }

    // fn start_listener(&mut self, port: u32) -> std::io::Result<()>{
    //     let listener = std::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port))?;
    //     println!("Started listener on port {}", port);

    //     //TODO: Split this off eventually to separate function to handle multiple incoming streams
    //     let (mut stream, _) = listener.accept()?;
    //     println!("Connection received on {}, via Bash from {}",port, stream.peer_addr().unwrap());

    //     let _t = Bash::pipe_thread(stream.try_clone().unwrap(), std::io::stdout());
    //     self.stream = Option::from(stream);

    //     return Ok(());
    // }

    fn listener(hostname:String,port:String) -> Result<TcpListener>{
        let pair = format!("{}:{}",hostname,port); 
        return TcpListener::bind(pair);
    }
    
    fn stream_handling(listener: Result<TcpListener>) -> -> io::Result<(TcpStream, SocketAddr)>{
        let (mut stream, _) = listener.accept()?;
        return stream;
    }
    
    fn editor_handling(stream: Result<(TcpStream, SocketAddr)) ->std::io::Result<()> {
    
    let mut line_editor = Editor::<()>::new();
    line_editor.bind_sequence(KeyEvent::ctrl('R'), Cmd::HistorySearchBackward);
    loop {
        let readline = line_editor.readline(">> ");
        match readline {
            Ok(command) => {
                line_editor.add_history_entry(command.as_str());

                println!("{}", command);

                // Clone command to increase its lifetime
                let command = command.clone() + "\n";

                // Send a TCP message
                stream
                    .write(command.as_bytes())
                    .expect("Faild to send TCP.");
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    return Ok(());
    }
    fn pipe_thread<R, W>(mut r: R, mut w: W) -> thread::JoinHandle<()>
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