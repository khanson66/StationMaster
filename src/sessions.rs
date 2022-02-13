struct SSH{
    hostname:String,
    port:i32,
    pid:i32
}

struct Netcat{
    hostname: String,
    port:i32,
    pid:i32
}

trait SESSION {
    fn new(hostname:String, port:i32) -> Self;
    fn close(&self);
    fn send_command(cmd:String) -> String;
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
}