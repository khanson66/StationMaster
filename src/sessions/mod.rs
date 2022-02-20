pub(crate) mod netcat;
pub(crate) mod ssh;

pub trait SESSION {
    fn start(&self);
    fn close(&self);
    fn send_command(&self, cmd:String);
    fn get_info();
}