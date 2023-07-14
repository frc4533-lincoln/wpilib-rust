use std::{collections::{HashMap, HashSet}, ops::Deref, sync::Arc};

use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};

use super::{commands::CommandTrait, Command};

static MANAGER: Mutex<Lazy<CommandManager>> = Mutex::new(Lazy::new(CommandManager::new));

type SubsystemSUID = u8;

pub trait Subsystem {
    fn periodic(&self) {}
}

type SubsystemArc = Arc<Mutex<dyn Subsystem + Sync + Send>>;
#[derive(Debug)]
pub struct SubsystemRef<T: Subsystem + Sync + Send>(pub Arc<Mutex<T>>);

impl<T: Subsystem + Sync + Send + 'static> SubsystemRef<T> {
    pub fn get_arc(&self) -> SubsystemArc {
        self.0.clone()
    }
    pub fn get_arc_impl(&self) -> Arc<Mutex<T>> {
        self.0.clone()
    }
}
impl<T: Subsystem + Sync + Send + 'static> Clone for SubsystemRef<T> {
    fn clone(&self) -> Self {
        Self{
            0: self.get_arc_impl()
        }
    }
}

// impl<T: Subsystem + Sync + Send + 'static> Deref for SubsystemRef<T> {
//     type Target = T;
//     #[inline]
//     fn deref(&self) -> &<Self as Deref>::Target {
//         self.0.lock().deref()
//     }
// }



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandIndex {
    DefaultCommand(usize),
    Command(usize),
}

pub struct CommandManager {
    periodic_callbacks: Vec<SubsystemArc>,
    commands: Vec<Option<Command>>,
    interrupt_state: HashMap<CommandIndex, bool>,
    default_commands: Vec<Option<Command>>,
    subsystem_to_default: HashMap<SubsystemSUID, CommandIndex>,
    requirements: HashMap<SubsystemSUID, CommandIndex>,
    initialized_commands: HashSet<CommandIndex>,
    orphaned_commands: HashSet<CommandIndex>,
    cond_schedulers: Vec<ConditionalScheduler>,
    suid: SubsystemSUID,
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
            suid: 0
        }
    }

    pub fn register_subsystem(suid: SubsystemSUID, subsystem: SubsystemArc, default_command: Option<Command>) {
        let mut scheduler = MANAGER.lock();
        scheduler
            .periodic_callbacks
            .push(subsystem);
        scheduler.default_commands.push(default_command);
        let idx = scheduler.default_commands.len() - 1;
        scheduler
            .subsystem_to_default
            .insert(suid, CommandIndex::DefaultCommand(idx));
        scheduler
            .interrupt_state
            .insert(CommandIndex::DefaultCommand(idx), false);

        drop(scheduler);
    }

    pub fn get_suid() -> SubsystemSUID{
        let mut scheduler = MANAGER.lock();
        let newsuid = scheduler.suid;
        scheduler.suid = scheduler.suid + 1;
        drop(scheduler);
        newsuid
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
            callback.lock().periodic();
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
                CommandIndex::DefaultCommand(cmd) => &mut self.default_commands[*cmd],
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
                if self.interrupt_state[index] {
                    command.end(true);
                    match *index {
                        CommandIndex::Command(idx) => to_remove.push(idx),
                        CommandIndex::DefaultCommand(_) => {}
                    }
                }
            }
        }
        for index in to_remove {
            self.initialized_commands
                .remove(&CommandIndex::Command(index));
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

    pub(super) fn cond_schedule(&mut self, command: Command) -> CommandIndex {
        let requirements = command.get_requirements();
        let index = self.add_command(command);
        if requirements.is_empty() {
            self.orphaned_commands.insert(index);
        } else {
            for requirement in requirements {
                //TODO: implement cancellation policy
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
                //TODO: implement cancellation policy
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
                }
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

    pub fn clear_cond_schedulers() {
        let mut manager = MANAGER.lock();
        manager.cond_schedulers.clear();
    }

    #[cfg(test)]
    pub fn purge_state_test() {
        let mut manager = MANAGER.lock();
        manager.periodic_callbacks.clear();
        manager.commands.clear();
        manager.interrupt_state.clear();
        manager.default_commands.clear();
        manager.subsystem_to_default.clear();
        manager.requirements.clear();
        manager.initialized_commands.clear();
        manager.orphaned_commands.clear();
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

pub trait Condition: Send{
    fn get_condition(&mut self) -> ConditionResponse;
    fn clone_boxed(&self) -> Box<dyn Condition>;
}

impl Clone for Box<dyn Condition> {
    fn clone(&self) -> Self {
        self.deref().clone_boxed()
    }
}



pub trait BoxedFn: Send + Sync {
    fn clone_boxed(&self) -> Box<dyn BoxedFn>;
    fn call(&self) -> Command;
}

impl<F: Fn() -> Command + Send + Sync + Clone + 'static> BoxedFn for F {
    fn clone_boxed(&self) -> Box<dyn BoxedFn> {
        Box::new(self.clone())
    }
    fn call(&self) -> Command{
        self()
    }

}

impl Clone for Box<dyn BoxedFn> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}

#[derive(Clone)]
pub struct ConditionalScheduler {
    active_commands: HashMap<usize, CommandIndex>,
    conds: Vec<(Box<dyn Condition>, Box<dyn BoxedFn>)>,
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
        for i in 0..self.conds.len() {
            let (cond, cmd) = &mut self.conds[i];
            let condition_result = cond.get_condition();

            match condition_result {
                ConditionResponse::Start => {
                    let command = cmd.call();
                    let cmd_idx = manager.cond_schedule(command);
                    println!("start");
                    self.active_commands.insert(i, cmd_idx);
                }
                ConditionResponse::Continue => {
                    self.active_commands.entry(i).or_insert_with(|| {
                        let command = cmd.call();
                        let cmd_idx = manager.cond_schedule(command);
                        println!("continue");
                        cmd_idx
                    });
                }
                ConditionResponse::Stop => {
                    if self.active_commands.contains_key(&i) {
                        manager.interrupt_command(self.active_commands[&i]);
                        self.active_commands.remove(&i);
                    }
                }
                ConditionResponse::NoChange => {
                    println!("no_change");
                }
            }
        }
    }

    pub fn add_cond(&mut self, cond: impl Condition, cmd: impl BoxedFn + 'static) {
        self.conds.push((cond.clone_boxed(), Box::new(cmd)));
    }
}
impl std::fmt::Debug for ConditionalScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConditionalScheduler").finish()
    }
}

#[macro_export]
macro_rules! register_subsystem {
    ($name:ident) => {
        {
            let instance = SubsystemRef{0: std::sync::Arc::<parking_lot::Mutex<$name>>::new(parking_lot::Mutex::<$name>::new($name::new()))};
            CommandManager::register_subsystem(
                CommandManager::get_suid(),
                instance.get_arc(),
                Some(instance.default_command()),
            );
            instance.clone()
        }
    };
}
