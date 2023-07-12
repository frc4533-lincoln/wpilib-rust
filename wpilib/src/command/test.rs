
use std::{sync::Arc, ops::Deref};

use parking_lot::{Mutex, MutexGuard};
use wpilib_macros::{subsystem, subsystem_methods};

use crate::command::{
    commands::CommandTrait, manager::CommandManager, Command,
    conditions::{self},
    ConditionalScheduler,
};

use super::{commands::CommandBuilder, manager::{Condition, ConditionResponse, Subsystem, SubsystemRef}};

#[test]
fn test_command() {
    CommandManager::cancel_all();
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
    default_running: bool,
    calls: i32,
}


impl TestSubsystem {
    fn new() -> Self {
        Self {
            motor_running: false,
            default_running: false,
            calls: 0
        }
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


    pub fn add_call(&mut self){
        self.calls += 1;
    }

    pub fn sub_call(&mut self){
        self.calls -= 1;
    }
    pub fn get_calls(&mut self) -> i32{
        self.calls
    }
    pub fn set_default_running(&mut self){
        self.default_running = true;
    }

    pub fn is_default_running(&mut self) -> bool{
        self.default_running
    }
}
impl SubsystemRef<TestSubsystem> {
    pub fn default_command(&self) -> Command {
        let clone = self.clone();
        CommandBuilder::new().init(|| ())
        .periodic(move || {
            println!("default");
            clone.0.lock().set_default_running();
        })
        .is_finished(|| false)
        .end(|interrupted| if interrupted {})
        .with_requirements(vec![1])
        .build()
        .with_name("Activate Motor")
    
    }
    pub fn cmd_activate_motor(&self) -> Command {
        let clone1 = self.clone();
        let clone2 = self.clone();
        CommandBuilder::new().init(
            move || {
                println!("cmd_activate_motor"); 
                clone1.0.lock().add_call();
                clone1.0.lock().start_motor();
            }
        )
        .periodic(|| ())
        .is_finished(|| false)
        .end(move |interrupted| {
            if interrupted {
                clone2.0.lock().sub_call();
            }
            clone2.0.lock().stop_motor();
        })
        .with_requirements(vec![1])
        .build()
        .with_name("Activate Motor")
    }

    #[allow(dead_code)]
    fn motor_name() -> String {
        "test".to_string()
    }

    pub fn reset(&mut self) {
        self.0.lock().calls = 0;
        self.0.lock().motor_running = false;
    }
}
impl Subsystem for TestSubsystem {
    fn periodic(&self) {
        println!("Periodic");
    }
}



#[test]
fn test_subsystem() {
    CommandManager::clear_cond_schedulers();
    CommandManager::cancel_all();
    // CommandManager::register_subsystem(
    //     TestSubsystem::suid(),
    //     || TestSubsystem::periodic(),
    //     Some(TestSubsystem::default_command()),
    // );
    let instance = register_subsystem!(TestSubsystem);
    assert!(!instance.0.lock().is_default_running());
    CommandManager::run();
    assert!(instance.0.lock().is_default_running());
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
fn test_on_true() {
    CommandManager::clear_cond_schedulers();
    CommandManager::cancel_all();
    let instance = register_subsystem!(TestSubsystem);
    let mut scheduler = ConditionalScheduler::new();
    
    let cond = conditions::on_true(|| true);
    
    let clone = instance.clone();    
    scheduler.add_cond(cond , move || clone.cmd_activate_motor());

    assert!(!instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 0);


    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    CommandManager::run();
    CommandManager::run();
    assert!(instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 1);
}

struct StateStruct {
    state_var: bool
}

static STATE: Mutex<StateStruct> = Mutex::new(StateStruct { state_var: false });

fn get_state() -> bool{
    let state = STATE.lock();
    state.state_var
}
fn set_state(b: bool) {
    let mut state = STATE.lock();
    state.state_var = b;
}

#[test]
fn test_while_true() {
    CommandManager::clear_cond_schedulers();
    CommandManager::cancel_all();
    let instance = register_subsystem!(TestSubsystem);
    let mut scheduler = ConditionalScheduler::new();

    let cond = conditions::while_true(|| get_state());

    let clone = instance.clone();    
    
    scheduler.add_cond(cond , move || clone.cmd_activate_motor());

    assert!(!instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 0);


    set_state(false);
    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    assert!(!instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 0);

    set_state(true);
    CommandManager::run();
    assert!(instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 1);

    CommandManager::run();
    assert!(instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 1);
    set_state(false);

    CommandManager::run();
    assert_eq!(instance.0.lock().get_calls(), 0);
    assert!(!instance.0.lock().is_motor_running());

}
