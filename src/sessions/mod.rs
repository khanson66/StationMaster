pub(crate) mod raw_socket;

pub trait SESSION {
    fn start(&mut self);
    fn close(&self);
    fn send_command(&self, cmd:String);
    fn get_name(&self) -> String;
}

// different types of rotations
pub enum Rotation{
    FIFO, //Drop oldest and add newest
    LIFO, //drop last item in vec and add newest, no real point of using but might be a use case
    HOLD, //Drop incoming sessions when full
}