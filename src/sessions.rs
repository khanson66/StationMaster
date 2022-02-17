pub(crate) mod netcat;
pub(crate) mod ssh;

pub trait SESSION {
    fn new(hostname:String, port:i32) -> Self;
    fn close(&self);
    fn send_command(cmd:String) -> String;
    fn get_info();
}