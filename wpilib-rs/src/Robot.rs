use std::{time::{Instant, Duration}, future::Future};

use parking_lot::Mutex;
use tokio::{runtime::Runtime, task::futures};


static PERIODIC_TIME: Mutex<f64> = Mutex::new(0.02);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotMode {
    Disabled = 0,
    Teleop = 1,
    Autonomous = 2,
    Test = 3,
}
impl RobotMode {
    pub fn is_disabled(&self) -> bool {
        match self {
            RobotMode::Disabled => true,
            _ => false
        }
    }
    pub fn is_autonomous(&self) -> bool {
        match self {
            RobotMode::Autonomous => true,
            _ => false
        }
    }
    pub fn is_teleop(&self) -> bool {
        match self {
            RobotMode::Teleop => true,
            _ => false
        }
    }
    pub fn is_test(&self) -> bool {
        match self {
            RobotMode::Test => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarwareType {
    Sim = 0,
    Rio1 = 1,
    Rio2 = 2,
}

pub trait RobotCore {
    fn get_name(&self) -> String;

    fn start(&mut self);

    fn end(&mut self);

    fn get_mode(&self) -> RobotMode;

    fn get_hardware(&self) -> HarwareType;

    fn add_callback(&mut self, callback: Box<dyn Fn()>);
}

pub trait UserRobot {
    //robot
    fn robot_init(&mut self);
    fn robot_periodic(&mut self);
    fn robot_end(&mut self);

    //disabled
    fn robot_disabled_init(&mut self) {}
    fn robot_disabled_periodic(&mut self) {}
    fn robot_disabled_end(&mut self) {}

    //autonomous
    fn robot_autonomous_init(&mut self) {}
    fn robot_autonomous_periodic(&mut self) {}
    fn robot_autonomous_end(&mut self) {}

    //teleop
    fn robot_teleop_init(&mut self) {}
    fn robot_teleop_periodic(&mut self) {}
    fn robot_teleop_end(&mut self) {}

    //test
    fn robot_test_init(&mut self) {}
    fn robot_test_periodic(&mut self) {}
    fn robot_test_end(&mut self) {}


    //sim
    fn sim_init(&mut self) {}

    fn sim_periodic(&mut self) {}
}

pub struct RobotCoreImpl {
    name: String,
    user_robot: Box<dyn UserRobot>,
    callbacks: Vec<Box<dyn Fn()>>
}
impl RobotCore for RobotCoreImpl {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self) {
        self.user_robot.robot_init();

        if self.get_hardware() == HarwareType::Sim {
            self.user_robot.sim_init();
        }

        let mut last_mode = self.get_mode();
        let mut start;

        loop {
            start = Instant::now();

            let mode = self.get_mode();

            if mode != last_mode {
                match mode {
                    RobotMode::Disabled => {
                        self.user_robot.robot_disabled_init();
                    },
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_init();
                    },
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_init();
                    },
                    RobotMode::Test => {
                        self.user_robot.robot_test_init();
                    },
                }
                match last_mode {
                    RobotMode::Disabled => {
                        self.user_robot.robot_disabled_end();
                    },
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_end();
                    },
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_end();
                    },
                    RobotMode::Test => {
                        self.user_robot.robot_test_end();
                    },
                }
            }

            match mode {
                RobotMode::Disabled => {
                    self.user_robot.robot_disabled_periodic();
                },
                RobotMode::Autonomous => {
                    self.user_robot.robot_autonomous_periodic();
                },
                RobotMode::Teleop => {
                    self.user_robot.robot_teleop_periodic();
                },
                RobotMode::Test => {
                    self.user_robot.robot_test_periodic();
                },
            }
            last_mode = mode;

            self.user_robot.robot_periodic();

            if self.get_hardware() == HarwareType::Sim {
                self.user_robot.sim_periodic();
            }


            //todo, make this more reliable
            std::thread::sleep(
                Duration::from_secs_f64(start.elapsed().as_secs_f64() - *PERIODIC_TIME.lock())
            );
        }
    }

    fn end(&mut self) {
        
    }

    fn get_mode(&self) -> RobotMode {
        RobotMode::Disabled
    }

    fn get_hardware(&self) -> HarwareType {
        HarwareType::Sim
    }

    fn add_callback(&mut self, callback: Box<dyn Fn()>) {
        self.callbacks.push(callback);
    }
}