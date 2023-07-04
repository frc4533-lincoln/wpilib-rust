use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use parking_lot::Mutex;

use crate::{command::CommandManager, if_not_athena, if_sim};

static PERIODIC_TIME: Mutex<f64> = Mutex::new(0.02);

pub fn set_periodic_time(time: f64) {
    *PERIODIC_TIME.lock() = time;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotMode {
    Disabled = 0,
    Teleop = 1,
    Autonomous = 2,
    Test = 3,
}
impl RobotMode {
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }
    #[must_use]
    pub const fn is_autonomous(&self) -> bool {
        matches!(self, Self::Autonomous)
    }
    #[must_use]
    pub const fn is_teleop(&self) -> bool {
        matches!(self, Self::Teleop)
    }
    #[must_use]
    pub const fn is_test(&self) -> bool {
        matches!(self, Self::Test)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareType {
    Sim = 0,
    Athena = 1,
    Other = 2,
}

pub trait RobotCore {
    fn start(&mut self);

    fn end(&mut self);

    fn get_mode(&self) -> RobotMode;
}

pub trait UserRobot: Send + Sync {
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
    user_robot: Box<dyn UserRobot>,
}
impl RobotCore for RobotCoreImpl {
    #[no_panic::no_panic]
    fn start(&mut self) {
        if_not_athena! {
            println!("WARNING: Running on non-Athena hardware. This is not officially supported.");
        }

        self.user_robot.robot_init();

        if_sim! {
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
                    }
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_init();
                    }
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_init();
                    }
                    RobotMode::Test => {
                        self.user_robot.robot_test_init();
                    }
                }
                match last_mode {
                    RobotMode::Disabled => {
                        self.user_robot.robot_disabled_end();
                    }
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_end();
                    }
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_end();
                    }
                    RobotMode::Test => {
                        self.user_robot.robot_test_end();
                    }
                }
            }

            match mode {
                RobotMode::Disabled => {
                    self.user_robot.robot_disabled_periodic();
                }
                RobotMode::Autonomous => {
                    self.user_robot.robot_autonomous_periodic();
                }
                RobotMode::Teleop => {
                    self.user_robot.robot_teleop_periodic();
                }
                RobotMode::Test => {
                    self.user_robot.robot_test_periodic();
                }
            }
            last_mode = mode;

            self.user_robot.robot_periodic();

            #[cfg(feature = "command")]
            {
                CommandManager::run();
            }

            if_sim! {
                self.user_robot.sim_periodic();
            }

            //todo, make this more reliable
            std::thread::sleep(Duration::from_secs_f64(
                start.elapsed().as_secs_f64() - *PERIODIC_TIME.lock(),
            ));
        }
    }

    fn end(&mut self) {}

    fn get_mode(&self) -> RobotMode {
        RobotMode::Disabled
    }
}
impl Debug for RobotCoreImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RobotCoreImpl").finish()
    }
}

#[no_panic::no_panic]
pub fn run_robot(user_robot: Box<dyn UserRobot>) {
    let mut robot = RobotCoreImpl { user_robot };
    robot.start();
    tracing::info!("Robot exited");
    robot.end();
}
