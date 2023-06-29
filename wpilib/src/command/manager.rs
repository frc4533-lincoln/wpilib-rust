use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use super::{commands::CommandTrait, Command};

static MANAGER: Mutex<Lazy<CommandManager>> = Mutex::new(Lazy::new(|| CommandManager::new()));

type CommandIndex = usize;
type SubsystemUUID = u8;

pub struct CommandManager {
    periodic_callbacks: Vec<Box<dyn Fn() + Send + Sync>>,
    commands: Vec<Option<Command>>,
    default_commands: HashMap<SubsystemUUID, CommandIndex>,
    requirements: HashMap<SubsystemUUID, CommandIndex>,
    initialized_commands: HashSet<CommandIndex>,
    orphaned_commands: HashSet<CommandIndex>,
    cond_schedulers: Vec<ConditionalScheduler>,
}
impl std::fmt::Debug for CommandManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandManager")
            .field("periodic_callbacks", &self.periodic_callbacks.len())
            .field("commands", &self.commands)
            .field("default_commands", &self.default_commands)
            .field("requirements", &self.requirements)
            .field("initialized_commands", &self.initialized_commands)
            .field("orphaned_commands", &self.orphaned_commands)
            .field("cond_schedulers", &self.cond_schedulers)
            .finish()
    }
}
impl CommandManager {
    fn new() -> Self {
        Self {
            periodic_callbacks: Vec::new(),
            commands: Vec::new(),
            default_commands: HashMap::new(),
            requirements: HashMap::new(),
            initialized_commands: HashSet::new(),
            orphaned_commands: HashSet::new(),
            cond_schedulers: Vec::new(),
        }
    }

    pub fn register_subsystem(uuid: u8, periodic_callback: fn(), default_command: Option<Command>) {
        let mut scheduler = MANAGER.lock();
        scheduler
            .periodic_callbacks
            .push(Box::new(periodic_callback));
        let cmd_idx = scheduler.add_command(default_command.unwrap_or_default());
        scheduler.default_commands.insert(uuid, cmd_idx);
    }

    /// Will run all periodic callbacks, run all conditional schedulers, init all un-initialized commands, and run all commands
    /// in that order.
    pub fn run() {
        let mut scheduler = MANAGER.lock();
        scheduler.run_subsystems();
        scheduler.run_cond_schedulers();
        scheduler.run_commands();
    }

    fn run_subsystems(&mut self) {
        for callback in &self.periodic_callbacks {
            callback();
        }
        for (uuid, cmd_idx) in &self.default_commands {
            if !self.requirements.contains_key(uuid) {
                self.requirements.insert(*uuid, *cmd_idx);
            }
        }
    }

    fn run_cond_schedulers(&mut self) {
        let mut new_conds = self.cond_schedulers.clone();
        for cond in &mut new_conds {
            cond.poll(self);
        }
        //to keep the edits to the schedulers store
        self.cond_schedulers = new_conds;
    }

    fn run_commands(&mut self) {
        let mut to_remove: Vec<CommandIndex> = Vec::new();
        let mut cmds = self.requirements.values().collect::<Vec<&CommandIndex>>();
        cmds.extend(self.orphaned_commands.iter());

        for index in cmds {
            if let Some(command) = self.commands[*index].as_mut() {
                if !self.initialized_commands.contains(index) {
                    command.init();
                    self.initialized_commands.insert(*index);
                }
                command.periodic();
                if command.is_finished() {
                    command.end(false);
                    to_remove.push(*index);
                }
            }
        }
        for index in to_remove {
            self.initialized_commands.remove(&index);
            if let Some(cmd) = self.commands.remove(index) {
                let requirements = cmd.get_requirements();
                if requirements.is_empty() {
                    self.orphaned_commands.remove(&index);
                } else {
                    for req in cmd.get_requirements() {
                        self.requirements.remove(&req);
                        if let Some(default) = self.default_commands.remove(&req) {
                            self.requirements.insert(req, default);
                        }
                    }
                }
            }
        }
    }

    fn add_command(&mut self, command: Command) -> usize {
        if let Some(index) = self.commands.iter().position(|x| x.is_none()) {
            self.commands[index] = Some(command);
            index
        } else {
            self.commands.push(Some(command));
            self.commands.len() - 1
        }
    }

    pub(super) fn cond_schedule(&mut self, command: Command) {
        let requirements = command.get_requirements();
        let index = self.add_command(command);
        if requirements.is_empty() {
            self.orphaned_commands.insert(index);
        } else {
            for requirement in requirements {
                //TODO: implement cancelation policy
                self.requirements.insert(requirement, index);
            }
        }
    }

    pub fn schedule(command: Command) {
        let mut scheduler = MANAGER.lock();
        let requirements = command.get_requirements();
        let index = scheduler.add_command(command);
        if requirements.is_empty() {
            scheduler.orphaned_commands.insert(index);
        } else {
            for requirement in requirements {
                //TODO: implement cancelation policy
                scheduler.requirements.insert(requirement, index);
            }
        }
    }

    pub fn cancel_all() {
        let mut scheduler = MANAGER.lock();
        scheduler.commands.clear();
        scheduler.requirements.clear();
        scheduler.initialized_commands.clear();
        scheduler.orphaned_commands.clear();
    }

    pub fn add_cond_scheduler(scheduler: ConditionalScheduler) {
        let mut manager = MANAGER.lock();
        manager.cond_schedulers.push(scheduler);
    }
}

#[derive(Clone, Debug)]
pub struct ConditionalScheduler {
    store: HashMap<String, f32>,
    conds: Vec<(fn(&mut Self) -> bool, fn() -> Command)>,
}
impl ConditionalScheduler {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            conds: Vec::new(),
        }
    }

    pub fn poll(&mut self, manager: &mut CommandManager) {
        for (cond, cmd) in self.conds.clone() {
            if cond(self) {
                let command = cmd();
                manager.cond_schedule(command);
            }
        }
    }

    pub fn add_cond(&mut self, cond: fn(&mut Self) -> bool, cmd: fn() -> Command) {
        self.conds.push((cond, cmd));
    }

    pub fn store_int(&mut self, name: &str, value: i32) {
        self.store.insert(name.to_string(), value as f32);
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.store.get(name).map(|x| *x as i32)
    }

    pub fn store_float(&mut self, name: &str, value: f32) {
        self.store.insert(name.to_string(), value);
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        self.store.get(name).map(|x| *x)
    }

    pub fn store_bool(&mut self, name: &str, value: bool) {
        self.store.insert(name.to_string(), value as i32 as f32);
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.store.get(name).map(|x| *x as i32 != 0)
    }
}

#[macro_export]
macro_rules! register_subsystem {
    ($name:ident) => {
        CommandManager::register_subsystem(
            $name::uuid(),
            || $name::get_static().periodic(),
            $name::get_static().get_default_command(),
        );
    };
}
