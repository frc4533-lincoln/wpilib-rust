use wpilib_macros::{subsystem, subsystem_methods};

crate_namespace!();

use crate::{command::{
    commands::CommandTrait,
    conditions::{self},
    manager::CommandManager,
    Command, ConditionalScheduler,
}, crate_namespace};

use super::{
    commands::CommandBuilder,
    manager::{Condition, ConditionResponse},
};

struct TestSubsystem {
    motor_running: bool,
    default_running: bool,
    calls: i32,
}

subsystem!(TestSubsystem);

#[subsystem_methods]
impl TestSubsystem {
    #[new]
    fn constructor() -> Self {
        Self {
            motor_running: false,
            default_running: false,
            calls: 0,
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
    pub fn stop_motor(&mut self) {
        self.motor_running = false;
    }

    #[dont_static]
    fn inner_add_call(&mut self) {
        self.calls += 1;
    }

    pub fn add_call(&mut self) {
        self.inner_add_call()
    }

    pub fn sub_call(&mut self) {
        self.calls -= 1;
    }
    pub fn get_calls(&mut self) -> i32 {
        self.calls
    }
    pub fn set_default_running(&mut self) {
        self.default_running = true;
    }

    pub fn is_default_running(&mut self) -> bool {
        self.default_running
    }

    #[default_command]
    pub fn default(&self) -> Command {
        CommandBuilder::new()
            .init(|| ())
            .periodic(|| {
                println!("default");
                Self::set_default_running();
            })
            .is_finished(|| false)
            .end(|interrupted| if interrupted {})
            .with_requirements(vec![Self::suid()])
            .build()
            .with_name("Activate Motor")
    }

    pub fn cmd_activate_motor(&self) -> Command {
        CommandBuilder::new()
            .init(|| {
                println!("cmd_activate_motor");
                Self::add_call();
                Self::start_motor();
            })
            .periodic(|| ())
            .is_finished(|| false)
            .end(|interrupted| {
                if interrupted {
                    Self::sub_call();
                }
                Self::stop_motor();
            })
            .with_requirements(vec![Self::suid()])
            .build()
            .with_name("Activate Motor")
    }

    #[allow(dead_code)]
    fn motor_name() -> String {
        "test".to_string()
    }

    pub fn reset(&mut self) {
        self.calls = 0;
        self.motor_running = false;
    }
}

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

fn test_subsystem() {
    register_subsystem!(TestSubsystem);
    assert!(!TestSubsystem::is_default_running());
    CommandManager::run();
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(TestSubsystem::is_default_running());
}

struct Immediately {}

impl Condition for Immediately {
    fn get_condition(&mut self) -> ConditionResponse {
        ConditionResponse::Start
    }
    fn clone_boxed(&self) -> Box<dyn Condition> {
        Box::new(Immediately {})
    }
}

fn test_on_true() {
    let mut scheduler = ConditionalScheduler::new();

    let cond = conditions::on_true(|| true);

    scheduler.add_cond(&cond, || TestSubsystem::cmd_activate_motor());

    assert!(!TestSubsystem::is_motor_running());
    assert_eq!(TestSubsystem::get_calls(), 0);

    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    CommandManager::run();
    CommandManager::run();
    assert!(TestSubsystem::is_motor_running());
    assert_eq!(TestSubsystem::get_calls(), 1);
}

fn run_in_clean_state(func: fn()) {
    func();
    CommandManager::purge_state_test();
    TestSubsystem::reset();
}

#[test]
fn parent_test() {
    run_in_clean_state(test_command);
    run_in_clean_state(test_subsystem);
    run_in_clean_state(test_on_true);
}
