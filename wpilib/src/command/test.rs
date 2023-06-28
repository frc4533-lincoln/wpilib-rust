
use wpilib_macros::{subsystem, subsystem_methods};

use crate::command::{
    manager::CommandManager,
    subsystem::Subsystem, Command, commands::CommandTrait};

use super::commands::CommandBuilder;

#[test]
fn test_command() {
    struct TestCommand {}
    impl CommandTrait for TestCommand {}
    unsafe impl Send for TestCommand {}

    let command = TestCommand {};

    CommandManager::schedule(Command::custom(Box::new(command)));

}

static UUID: u8 = 0;

// subsystem!{ name: TestSubsystem, upper: TESTSUBSYSTEM }

struct TestSubsystem {
    _motor: String,
    _is_motor_running: bool,
}

subsystem!(TestSubsystem);

#[subsystem_methods]
impl TestSubsystem {
    #[new]
    fn new() -> Self {
        Self {
            _motor: "Motor".to_string(),
            _is_motor_running: false,
        }
    }

    fn is_motor_running(&self) -> bool {
        self._is_motor_running
    }


    fn cmd_activate_motor(&mut self) -> Command {
        CommandBuilder::start_only(
            || {
                Self::get_static()._is_motor_running = true;
            },
            vec![self.get_uuid()])
            .with_name("Activate Motor")
    }
}

impl Subsystem for TestSubsystem {
    fn get_name(&self) -> String {
        "TestSubsystem".to_string()
    }

    fn get_uuid(&self) -> u8 {
        UUID
    }
}

#[test]
fn test_subsystem() {
    let default_command = TestSubsystem::cmd_activate_motor();
    CommandManager::register_subsystem(UUID, || TestSubsystem::get_static().periodic(), default_command);
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
}

// #[test]
// fn test_conditional_scheduler() {
//     let mut scheduler = ConditionalScheduler::new();
//     scheduler.add_cond(|| true, || TESTSUBSYSTEM.lock().cmd_activate_motor());

//     CommandManager::add_cond_scheduler(scheduler);
//     CommandManager::run();
//     assert!(TESTSUBSYSTEM.lock()._is_motor_running);
// }
