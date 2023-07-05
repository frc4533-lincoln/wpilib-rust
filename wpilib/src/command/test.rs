use wpilib_macros::{subsystem, subsystem_methods};

use crate::command::{
    commands::CommandTrait, manager::CommandManager, Command,
    ConditionalScheduler,
};

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
    })
    .join()
    .unwrap();
}

struct TestSubsystem {
    motor_running: bool,
}

subsystem!(TestSubsystem);

#[subsystem_methods]
impl TestSubsystem {
    #[new]
    fn constructor() -> Self {
        Self {
            motor_running: false,
        }
    }

    #[periodic]
    fn periodic(&self) {
        println!("Periodic");
    }

    #[default_command]
    pub fn cmd_activate_motor(&self) -> Command {
        if !self.is_motor_running() {
            CommandBuilder::start_only(
                || {
                    Self::start_motor()
                },
                vec![Self::suid()],
            )
            .with_name("Activate Motor")
        } else {
            Default::default()
        }
    }

    pub fn is_motor_running(&self) -> bool {
        self.motor_running
    }

    pub fn start_motor(&mut self) {
        self.motor_running = true;
    }


    #[allow(dead_code)]
    fn motor_name() -> String {
        "test".to_string()
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
    scheduler.add_cond(|_| true, || TestSubsystem::cmd_activate_motor());

    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
}
