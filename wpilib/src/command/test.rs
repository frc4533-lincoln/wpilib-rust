use wpilib_macros::{subsystem, subsystem_methods};

use crate::command::{
    commands::CommandTrait, manager::CommandManager, Command,
    ConditionalScheduler,
};

use super::{commands::CommandBuilder, manager::{Condition, ConditionResponse}};

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

    pub fn is_motor_running(&self) -> bool {
        self.motor_running
    }

    pub fn start_motor(&mut self) {
        self.motor_running = true;
    }

    #[default_command]
    pub fn cmd_activate_motor(&self) -> Command {
        if !self.is_motor_running() {
            CommandBuilder::start_only(
                || {
                    println!("cmd_activate_motor");
                    Self::start_motor()
                },
                vec![Self::suid()],
            )
            .with_name("Activate Motor")
        } else {
            Default::default()
        }
    }

    #[allow(dead_code)]
    fn motor_name() -> String {
        "test".to_string()
    }
}



#[test]
fn test_subsystem() {
    // CommandManager::register_subsystem(
    //     TestSubsystem::suid(),
    //     || TestSubsystem::periodic(),
    //     Some(TestSubsystem::default_command()),
    // );
    register_subsystem!(TestSubsystem);
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
}


struct Immediately {}

impl Condition for Immediately {
    fn get_condition(&mut self) -> ConditionResponse {
        ConditionResponse::Start
    }
    fn clone_boxed(&self) -> Box<dyn Condition> {
        Box::new(Immediately{})
    }
}

#[test]
fn test_conditional_scheduler() {
    let mut scheduler = ConditionalScheduler::new();
    scheduler.add_cond(Immediately{} , || TestSubsystem::cmd_activate_motor());

    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
}
