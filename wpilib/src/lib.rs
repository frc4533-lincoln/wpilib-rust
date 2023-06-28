extern crate wpilib_macros;

use robots::UserRobot;


#[cfg(feature = "command")]
pub mod command;
pub mod math;
pub mod robots;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTypes {
    Init,
    Periodic,
    End,
}

pub fn wpilib_main(_robot: Box<dyn UserRobot>) {

}
