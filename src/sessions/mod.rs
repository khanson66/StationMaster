pub(crate) mod raw_socket;

pub trait SESSION {
    fn start(&mut self);
    fn close(&mut self);
    fn send_command(&self, cmd:String);
    fn get_name(&self) -> String;
}