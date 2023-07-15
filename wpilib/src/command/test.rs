use wpilib_macros::{subsystem, subsystem_methods, command};
use wpilib::command::manager::{Subsystem, SubsystemRef};

crate_namespace!();

use crate::{
    command::{
        commands::CommandTrait,
        conditions::{self},
        manager::CommandManager,
        Command, ConditionalScheduler,
    },
    crate_namespace,
};

use super::{
    commands::CommandBuilder,
    manager::{Condition, ConditionResponse},
};

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
            calls: 0,
        }
    }
    fn is_motor_running(&self) -> bool {
        self.motor_running
    }

    fn start_motor(&mut self) {
        self.motor_running = true;
    }
    fn stop_motor(&mut self) {
        self.motor_running = false;
    }

    fn inner_add_call(&mut self) {
        self.calls += 1;
    }

    fn add_call(&mut self) {
        self.inner_add_call()
    }

    fn sub_call(&mut self) {
        self.calls -= 1;
    }
    fn get_calls(&mut self) -> i32 {
        self.calls
    }
    fn set_default_running(&mut self) {
        self.default_running = true;
    }

    fn is_default_running(&mut self) -> bool {
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
        CommandBuilder::new().init(
            command!{self,
                {
                    println!("cmd_activate_motor"); 
                    __self.add_call();
                    __self.start_motor();
                }
            }
        )
        .periodic(|| ())
        .is_finished(|| false)
        .end(
            |b| ()
        )
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
        Box::new(Immediately {})
    }
}

fn test_on_true() {
    CommandManager::clear_cond_schedulers();
    CommandManager::cancel_all();
    let instance = register_subsystem!(TestSubsystem);
    let mut scheduler = ConditionalScheduler::new();

    let cond = conditions::on_true(|| true);
    
        
    scheduler.add_cond(cond , {
        let clone = instance.clone();
        move || clone.cmd_activate_motor()
    });

    assert!(!instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 0);

    CommandManager::add_cond_scheduler(scheduler);
    CommandManager::run();
    CommandManager::run();
    CommandManager::run();
    assert!(instance.0.lock().is_motor_running());
    assert_eq!(instance.0.lock().get_calls(), 1);
}

fn run_in_clean_state(func: fn()) {
    func();
    CommandManager::purge_state_test();
    
}


#[test]
fn parent_test() {
    run_in_clean_state(test_command);
    run_in_clean_state(test_subsystem);
    run_in_clean_state(test_on_true);
}