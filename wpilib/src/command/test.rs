use std::sync::Arc;

use wpilib_macros::{subsystem, subsystem_methods};

use crate::command::{
    commands::CommandTrait, manager::{CommandManager, OnTrue}, Command,
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
    calls: i32,
}

subsystem!(TestSubsystem);

#[subsystem_methods]
impl TestSubsystem {
    #[new]
    fn constructor() -> Self {
        Self {
            motor_running: false,
            calls: 0
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

    pub fn add_call(&mut self){
        self.calls += 1;
    }

    pub fn get_calls(&mut self) -> i32{
        self.calls
    }
    #[default_command]
    pub fn cmd_activate_motor(&self) -> Command {
        if !self.is_motor_running() {
            CommandBuilder::start_only(
                || {
                    Self::add_call();
                    Self::start_motor();
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
    
    let cond = OnTrue{function: Arc::new(|| true), last_state: false};
    
    scheduler.add_cond(cond , || TestSubsystem::cmd_activate_motor());

    assert!(!TestSubsystem::is_motor_running());
    assert_eq!(TestSubsystem::get_calls(), 0);


    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    CommandManager::run();
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
    assert_eq!(TestSubsystem::get_calls(), 1);
}
