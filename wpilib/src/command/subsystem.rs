

pub trait Subsystem {

    fn periodic(&mut self) {}

    fn get_default_command(&self) -> Option<super::Command> {
        None
    }

    fn test_init(&mut self) {}

    fn test_periodic(&mut self) {}

    fn test_end(&mut self) {}
}



