pub(crate) mod netcat;
pub(crate) mod ssh;

pub trait SESSION {
    fn start(&mut self);
    fn close(&mut self);
    fn send_command(&self, cmd:String);
    fn get_info();
}