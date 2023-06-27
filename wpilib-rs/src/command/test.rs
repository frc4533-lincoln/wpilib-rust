use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::command::{
    manager::{CommandManager, ConditionalScheduler},
    traits::Subsystem, Command, commands::{CommandBuilder, CommandTrait},
};

#[test]
fn test_command() {
    struct TestCommand {}
    impl CommandTrait for TestCommand {}
    unsafe impl Send for TestCommand {}

    let command = TestCommand {};

    CommandManager::schedule(Command::custom(Box::new(command)));
}

static UUID: u8 = 0;
static SUBSYSTEM: Lazy<Mutex<Box<TestSubsystem>>> = Lazy::new(|| Mutex::new(Box::new(TestSubsystem::new())));

struct TestSubsystem {
    _motor: String,
    _is_motor_running: bool,
}
impl Subsystem for TestSubsystem {
    fn get_name(&self) -> String {
        "TestSubsystem".to_string()
    }

    fn get_uuid(&self) -> u8 {
        UUID
    }
}
impl TestSubsystem {
    fn new() -> Self {
        Self {
            _motor: "Motor".to_string(),
            _is_motor_running: false,
        }
    }

    fn cmd_activate_motor(&mut self) -> Command {
        CommandBuilder::start_only(
            || {
                SUBSYSTEM.lock()._is_motor_running = true;
                tracing::info!("Motor activated");
            },
            vec![self.get_uuid()])
            .with_name("Activate Motor")
    }
}
unsafe impl Send for TestSubsystem {}

#[test]
fn test_subsystem() {
    let default_command = {
        let mut subsystem = SUBSYSTEM.lock();
        subsystem.cmd_activate_motor()
    };
    CommandManager::register_subsystem(UUID, || SUBSYSTEM.lock().periodic(), default_command);
    CommandManager::run();
    assert!(SUBSYSTEM.lock()._is_motor_running);
}

#[test]
fn test_conditional_scheduler() {
    let mut scheduler = ConditionalScheduler::new();
    scheduler.add_cond(|| true, || SUBSYSTEM.lock().cmd_activate_motor());

    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    assert!(SUBSYSTEM.lock()._is_motor_running);
}
