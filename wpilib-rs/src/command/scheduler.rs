use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use super::{Command, commands::CommandTrait};

static MANAGER: Mutex<Lazy<CommandManager>> = Mutex::new(Lazy::new(|| CommandManager::new()));

type CommandIndex = usize;
type SubsystemUUID = u8;

pub struct CommandManager {
    periodic_callbacks: Vec<Box<dyn Fn() + Send>>,
    commands: Vec<Option<Command>>,
    default_commands: HashMap<SubsystemUUID, CommandIndex>,
    requirements: HashMap<SubsystemUUID, CommandIndex>,
    initialized_commands: HashSet<CommandIndex>,
    orphaned_commands: HashSet<CommandIndex>,
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
        }
    }

    // pub fn register_subsystem(subsystem: &Box<dyn super::Subsystem>) {
    //     let mut scheduler = MANAGER.lock();
    //     let uuid = subsystem.get_uuid();
    //     let cmd_idx = scheduler.add_command(subsystem.get_default_command().unwrap_or(Command::empty()));
    //     scheduler.default_commands.insert(uuid, cmd_idx);
    // }

    pub fn register_subsystem(uuid: u8, periodic_callback: fn(), default_command: Command) {
        let mut scheduler = MANAGER.lock();
        scheduler.periodic_callbacks.push(Box::new(periodic_callback));
        let cmd_idx = scheduler.add_command(default_command);
        scheduler.default_commands.insert(uuid, cmd_idx);
    }

    pub fn run() {
        let mut scheduler = MANAGER.lock();
        scheduler.run_subsystems();
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
}
