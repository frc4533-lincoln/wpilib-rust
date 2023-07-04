use std::{collections::{HashMap, HashSet}, ops::Deref};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use super::{commands::CommandTrait, Command};

static MANAGER: Mutex<Lazy<CommandManager>> = Mutex::new(Lazy::new(CommandManager::new));

type SubsystemSUID = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandIndex {
    DefaultCommand(usize),
    Command(usize)
}

pub struct CommandManager {
    periodic_callbacks: Vec<fn()>,
    commands: Vec<Option<Command>>,
    interrupt_state: HashMap<CommandIndex, bool>,
    default_commands: Vec<Option<Command>>,
    subsystem_to_default: HashMap<SubsystemSUID, CommandIndex>,
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
            default_commands: Vec::new(),
            subsystem_to_default: HashMap::new(),
            requirements: HashMap::new(),
            initialized_commands: HashSet::new(),
            orphaned_commands: HashSet::new(),
            cond_schedulers: Vec::new(),
        }
    }

    pub fn register_subsystem(suid: SubsystemSUID, periodic_callback: fn(), default_command: Option<Command>) {
        let mut scheduler = MANAGER.lock();
        scheduler
            .periodic_callbacks
            .push(periodic_callback);
        scheduler.default_commands.push(default_command);
        let idx = scheduler.default_commands.len() - 1;
        scheduler.subsystem_to_default.insert(suid, CommandIndex::DefaultCommand(idx));
        scheduler.interrupt_state.insert(CommandIndex::DefaultCommand(idx), false);
        
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
        for (suid, cmd_idx) in &self.subsystem_to_default {
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
        let mut to_remove: Vec<usize> = Vec::new();
        let mut cmds = self.requirements.values().collect::<Vec<&CommandIndex>>();
        cmds.extend(self.orphaned_commands.iter());

        for index in cmds {
            if let Some(command) = match index {
                CommandIndex::Command(cmd) => &mut self.commands[*cmd],
                CommandIndex::DefaultCommand(cmd) => &mut self.default_commands[*cmd]
            } {
                if !self.initialized_commands.contains(index) {
                    command.init();
                    self.initialized_commands.insert(*index);
                }
                command.periodic();
                if command.is_finished() {
                    command.end(false);
                    match *index {
                        CommandIndex::Command(idx) => to_remove.push(idx),
                        CommandIndex::DefaultCommand(_) => {}
                    }
                }
                if self.interrupt_state[index]{
                    command.end(true);
                    match *index {
                        CommandIndex::Command(idx) => to_remove.push(idx),
                        CommandIndex::DefaultCommand(_) => {}
                    }
                }
            }
        }
        for index in to_remove {
            self.initialized_commands.remove(&CommandIndex::Command(index));
            if let Some(cmd) = self.commands.remove(index) {
                let requirements = cmd.get_requirements();
                if requirements.is_empty() {
                    self.orphaned_commands.remove(&CommandIndex::Command(index));
                } else {
                    for req in cmd.get_requirements() {
                        self.requirements.remove(&req);
                        let idx = self.subsystem_to_default[&req];
                        self.requirements.insert(req, idx);
                    }
                }
            }
        }
    }

    fn add_command(&mut self, command: Command) -> CommandIndex {
        if let Some(index) = self.commands.iter().position(Option::is_none) {
            self.commands[index] = Some(command);
            let cmd_idx = CommandIndex::Command(index);
            self.interrupt_state.insert(cmd_idx, false);
            cmd_idx
        } else {
            self.commands.push(Some(command));
            let cmd_idx = CommandIndex::Command(self.commands.len() - 1);
            self.interrupt_state.insert(cmd_idx, false);
            cmd_idx
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
        for maybe_command in &mut scheduler.commands {
            match maybe_command {
                Some(command) => {
                    command.end(true);
                },
                None => {}
            }
        }
        scheduler.commands.clear();
        scheduler.requirements.clear();
        scheduler.initialized_commands.clear();
        scheduler.orphaned_commands.clear();

        
    }

    pub fn add_cond_scheduler(scheduler: ConditionalScheduler) {
        let mut manager = MANAGER.lock();
        manager.cond_schedulers.push(scheduler);
    }

    pub fn clear_cond_schedulers(){
        let mut manager = MANAGER.lock();
        manager.cond_schedulers.clear();
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
                    println!("start"); 
                    self.active_commands.insert(i, cmd_idx);
                },
                ConditionResponse::Continue => {
                    if !self.active_commands.contains_key(&i) {
                        let command = cmd();
                        let cmd_idx = manager.cond_schedule(command);
                        println!("continue"); 
                        self.active_commands.insert(i, cmd_idx);    
                    }
                },
                ConditionResponse::Stop => {
                    if self.active_commands.contains_key(&i) {
                        manager.interrupt_command(*self.active_commands.get(&i).unwrap());
                        self.active_commands.remove(&i);    
                    }
                },
                ConditionResponse::NoChange => {
                    println!("no_change"); 
                }
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
