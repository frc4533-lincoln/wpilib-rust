
#[cfg(feature = "command")]
pub mod command;
pub mod math;
pub mod Robot;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTypes {
    Init,
    Periodic,
    End,
}