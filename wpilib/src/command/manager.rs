use std::{collections::{HashMap, HashSet}, ops::Deref};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use super::{commands::CommandTrait, Command};

static MANAGER: Mutex<Lazy<CommandManager>> = Mutex::new(Lazy::new(CommandManager::new));

type CommandIndex = usize;
type SubsystemSUID = u8;

pub struct CommandManager {
    periodic_callbacks: Vec<fn()>,
    commands: Vec<Option<Command>>,
    interrupt_state: HashMap<CommandIndex, bool>,
    default_commands: HashMap<SubsystemSUID, CommandIndex>,
    requirements: HashMap<SubsystemSUID, CommandIndex>,
    initialized_commands: HashSet<CommandIndex>,
    orphaned_commands: HashSet<CommandIndex>,
    cond_schedulers: Vec<ConditionalScheduler>,
}
impl std::fmt::Debug for CommandManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandManager")
            .field("periodic_callbacks", &self.periodic_callbacks.len())
            .field("commands", &self.commands)
            .field("interrupt_state", &self.interrupt_state)
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
            interrupt_state: HashMap::new(),
            default_commands: HashMap::new(),
            requirements: HashMap::new(),
            initialized_commands: HashSet::new(),
            orphaned_commands: HashSet::new(),
            cond_schedulers: Vec::new(),
        }
    }

    pub fn register_subsystem(suid: u8, periodic_callback: fn(), default_command: Option<Command>) {
        let mut scheduler = MANAGER.lock();
        scheduler
            .periodic_callbacks
            .push(periodic_callback);
        let cmd_idx = scheduler.add_command(default_command.unwrap_or_default());
        scheduler.default_commands.insert(suid, cmd_idx);
        drop(scheduler);
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
        for (suid, cmd_idx) in &self.default_commands {
            if !self.requirements.contains_key(suid) {
                self.requirements.insert(*suid, *cmd_idx);
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
                if self.interrupt_state[index]{
                    command.end(true);
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

    fn add_command(&mut self, command: Command) -> CommandIndex {
        if let Some(index) = self.commands.iter().position(Option::is_none) {
            self.commands[index] = Some(command);
            self.interrupt_state.insert(index, false);
            index
        } else {
            self.commands.push(Some(command));
            self.interrupt_state.insert(self.commands.len() - 1, false);
            self.commands.len() - 1
        }
    }

    fn interrupt_command(&mut self, idx: CommandIndex) {
        if self.interrupt_state.contains_key(&idx) {
            self.interrupt_state.insert(idx, true);
        }
    }

    pub(super) fn cond_schedule(&mut self, command: Command) -> CommandIndex{
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
        index
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

#[derive(Debug, Clone, Copy)]
pub enum ConditionResponse {
    Start,
    Continue,
    Stop,
    NoChange,
}

pub trait Condition: Send {
    fn get_condition(&mut self) -> ConditionResponse;
    fn clone_boxed(&self) -> Box<dyn Condition>;
}

impl Clone for Box<dyn Condition>{
    fn clone(&self) -> Self {
        self.deref().clone_boxed()
    }
}



#[derive(Clone)]
pub struct ConditionalScheduler {
    active_commands: HashMap<usize, CommandIndex>,
    conds: Vec<(Box<dyn Condition>, fn() -> Command)>,
}

impl ConditionalScheduler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_commands: HashMap::new(),
            conds: Vec::new(),
        }
    }

    pub fn poll(&mut self, manager: &mut CommandManager) {
        
        for i in 0..self.conds.len(){
            let (cond, cmd) = &mut self.conds[i];
            let condition_result = cond.get_condition();

            match condition_result{
                ConditionResponse::Start => {
                    let command = cmd();
                    let cmd_idx = manager.cond_schedule(command);
                    self.active_commands.insert(i, cmd_idx);
                },
                ConditionResponse::Continue => {
                    if !self.active_commands.contains_key(&i) {
                        let command = cmd();
                        let cmd_idx = manager.cond_schedule(command);
                        self.active_commands.insert(i, cmd_idx);    
                    }
                },
                ConditionResponse::Stop => {
                    if self.active_commands.contains_key(&i) {
                        manager.interrupt_command(*self.active_commands.get(&i).unwrap());
                        self.active_commands.remove(&i);    
                    }
                },
                ConditionResponse::NoChange => {}
            }
        }
    }

    pub fn add_cond(&mut self, cond: impl Condition, cmd: fn() -> Command) {
        self.conds.push((cond.clone_boxed(), cmd));
    }
}
impl std::fmt::Debug for ConditionalScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConditionalScheduler")
            .finish()
    }
}

#[macro_export]
macro_rules! register_subsystem {
    ($name:ident) => {
        CommandManager::register_subsystem(
            $name::suid(),
            || $name::periodic(),
            Some($name::default_command()),
        );
    };
}
