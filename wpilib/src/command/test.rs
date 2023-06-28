
use wpilib_macros::{subsystem, subsystem_methods};

use crate::command::{
    manager::CommandManager,
    subsystem::Subsystem, Command, commands::CommandTrait, ConditionalScheduler};

use super::commands::CommandBuilder;

#[test]
fn test_command() {
    fn schedule_test() {
        struct TestCommand {}
        impl CommandTrait for TestCommand {}

        let command = TestCommand {};

        CommandManager::schedule(Command::custom(Box::new(command)));
    }

    schedule_test();

    std::thread::spawn(|| {
        schedule_test();
        CommandManager::run();
    }).join().unwrap();
}

// static UUID: u8 = 0;

// subsystem!{ name: TestSubsystem, upper: TESTSUBSYSTEM }

struct TestSubsystem {
    _motor: String,
    _is_motor_running: bool,
}

subsystem!(TestSubsystem, 1);

#[subsystem_methods]
impl TestSubsystem {
    #[new]
    fn constructor() -> Self {
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
            vec![Self::uuid()])
            .with_name("Activate Motor")
    }
}

impl Subsystem for TestSubsystem {
    fn get_default_command(&self) -> Option<Command> {
        Some(Self::cmd_activate_motor())
    }
}

#[test]
fn test_subsystem() {
    register_subsystem!(TestSubsystem);
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
}

#[test]
fn test_conditional_scheduler() {
    let mut scheduler = ConditionalScheduler::new();
    scheduler.add_cond(|| true, || TestSubsystem::cmd_activate_motor());

    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
}
