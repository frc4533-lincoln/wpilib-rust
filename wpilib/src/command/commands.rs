use crate::command;
use std::{collections::HashSet, fmt::Debug};

pub trait CommandTrait {
    fn init(&mut self) {}

    fn periodic(&mut self) {}

    fn end(&mut self, _interrupted: bool) {}

    fn is_finished(&mut self) -> bool {
        false
    }

    // fn add_requirements(&mut self, _subsystems: Vec<u8>) {}

    fn get_requirements(&self) -> Vec<u8> {
        Vec::new()
    }

    fn run_when_disabled(&self) -> bool {
        false
    }

    fn cancel_incoming(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        String::from("unnamed command")
    }
}

#[allow(missing_debug_implementations)]
pub struct CommandBuilder {
    init: Option<Box<dyn FnMut()>>,
    periodic: Option<Box<dyn FnMut()>>,
    end: Option<Box<dyn FnMut(bool)>>,
    is_finished: Option<Box<dyn FnMut() -> bool>>,
    requirements: Vec<u8>,
}

impl CommandBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            init: None,
            periodic: None,
            end: None,
            is_finished: None,
            requirements: Vec::new(),
        }
    }

    #[must_use]
    pub fn init(mut self, init: impl FnMut() + 'static) -> Self {
        self.init = Some(Box::new(init));
        self
    }

    #[must_use]
    pub fn periodic(mut self, periodic: impl FnMut() + 'static) -> Self {
        self.periodic = Some(Box::new(periodic));
        self
    }

    #[must_use]
    pub fn end(mut self, end: impl FnMut(bool) + 'static) -> Self {
        self.end = Some(Box::new(end));
        self
    }

    #[must_use]
    pub fn is_finished(mut self, is_finished: impl FnMut() -> bool + 'static) -> Self {
        self.is_finished = Some(Box::new(is_finished));
        self
    }

    #[must_use]
    pub fn with_requirements(mut self, requirements: Vec<u8>) -> Self {
        self.requirements = requirements;
        self
    }

    #[must_use]
    pub fn build(self) -> Command {
        Command::Simple(SimpleBuiltCommand {
            init: self.init,
            periodic: self.periodic,
            end: self.end,
            is_finished: self.is_finished,
            requirements: self.requirements,
        })
    }
}

impl CommandBuilder {
    pub fn start_only(init: impl FnMut() + 'static, requirements: Vec<u8>) -> Command {
        Self::new()
            .init(init)
            .with_requirements(requirements)
            .build()
    }

    pub fn run_only(periodic: impl FnMut() + 'static, requirements: Vec<u8>) -> Command {
        Self::new()
            .periodic(periodic)
            .with_requirements(requirements)
            .build()
    }

    pub fn end_only(end: impl FnMut(bool) + 'static, requirements: Vec<u8>) -> Command {
        Self::new().end(end).with_requirements(requirements).build()
    }

    pub fn run_start(
        init: impl FnMut() + 'static,
        periodic: impl FnMut() + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .init(init)
            .periodic(periodic)
            .with_requirements(requirements)
            .build()
    }

    pub fn run_end(
        periodic: impl FnMut() + 'static,
        end: impl FnMut(bool) + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .periodic(periodic)
            .end(end)
            .with_requirements(requirements)
            .build()
    }

    pub fn start_end(
        init: impl FnMut() + 'static,
        end: impl FnMut(bool) + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .init(init)
            .end(end)
            .with_requirements(requirements)
            .build()
    }

    pub fn run_start_end(
        init: impl FnMut() + 'static,
        periodic: impl FnMut() + 'static,
        end: impl FnMut(bool) + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .init(init)
            .periodic(periodic)
            .end(end)
            .with_requirements(requirements)
            .build()
    }

    pub fn run_until(
        is_finished: impl FnMut() -> bool + 'static,
        periodic: impl FnMut() + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .is_finished(is_finished)
            .periodic(periodic)
            .with_requirements(requirements)
            .build()
    }

    pub fn run_end_until(
        is_finished: impl FnMut() -> bool + 'static,
        periodic: impl FnMut() + 'static,
        end: impl FnMut(bool) + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .is_finished(is_finished)
            .periodic(periodic)
            .end(end)
            .with_requirements(requirements)
            .build()
    }

    pub fn start_run_until(
        init: impl FnMut() + 'static,
        is_finished: impl FnMut() -> bool + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .init(init)
            .is_finished(is_finished)
            .with_requirements(requirements)
            .build()
    }

    pub fn all(
        init: impl FnMut() + 'static,
        periodic: impl FnMut() + 'static,
        end: impl FnMut(bool) + 'static,
        is_finished: impl FnMut() -> bool + 'static,
        requirements: Vec<u8>,
    ) -> Command {
        Self::new()
            .init(init)
            .periodic(periodic)
            .end(end)
            .is_finished(is_finished)
            .with_requirements(requirements)
            .build()
    }
}

pub struct SimpleBuiltCommand {
    init: Option<Box<dyn FnMut()>>,
    periodic: Option<Box<dyn FnMut()>>,
    end: Option<Box<dyn FnMut(bool)>>,
    is_finished: Option<Box<dyn FnMut() -> bool>>,
    requirements: Vec<u8>,
}
impl CommandTrait for SimpleBuiltCommand {
    fn init(&mut self) {
        if let Some(init) = self.init.as_mut() {
            init();
        }
    }

    fn periodic(&mut self) {
        if let Some(periodic) = self.periodic.as_mut() {
            periodic();
        }
    }

    fn end(&mut self, interrupted: bool) {
        if let Some(end) = self.end.as_mut() {
            end(interrupted);
        }
    }

    fn is_finished(&mut self) -> bool {
        self.is_finished
            .as_mut()
            .map_or(false, |is_finished| is_finished())
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.requirements.clone()
    }
}
impl Debug for SimpleBuiltCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("SimpleBuiltCommand")
            .field("init", &self.init.is_some())
            .field("periodic", &self.periodic.is_some())
            .field("end", &self.end.is_some())
            .field("is_finished", &self.is_finished.is_some())
            .field("requirements", &self.requirements)
            .finish()
    }
}

#[derive(Debug)]
pub struct ParallelBuiltCommand {
    commands: Vec<Command>,
    finished: Vec<bool>,
    requirements: HashSet<u8>,
    race: bool,
}
impl CommandTrait for ParallelBuiltCommand {
    fn init(&mut self) {
        for command in &mut self.commands {
            command.init();
        }
    }

    fn periodic(&mut self) {
        for (i, command) in self.commands.iter_mut().enumerate() {
            if !self.finished[i] {
                command.periodic();
                if command.is_finished() {
                    command.end(false);
                    self.finished[i] = true;
                }
            }
        }
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            for (i, command) in self.commands.iter_mut().enumerate() {
                if !self.finished[i] {
                    command.end(true);
                    self.finished[i] = true;
                }
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        if self.race {
            self.finished.iter().any(|&finished| finished)
        } else {
            self.finished.iter().all(|&finished| finished)
        }
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.requirements.clone().into_iter().collect()
    }

    fn get_name(&self) -> String {
        self.commands
            .iter()
            .map(command::commands::CommandTrait::get_name)
            .collect::<Vec<_>>()
            .join(",")
    }
}
// unsafe impl Send for ParallelBuiltCommand {}

#[derive(Debug)]
pub struct SequentialCommand {
    commands: Vec<Command>,
    current: usize,
    requirements: HashSet<u8>,
}
impl CommandTrait for SequentialCommand {
    fn init(&mut self) {
        self.commands[self.current].init();
    }

    fn periodic(&mut self) {
        self.commands[self.current].periodic();
        if self.commands[self.current].is_finished() {
            self.commands[self.current].end(false);
            self.current += 1;
            if self.current < self.commands.len() {
                self.commands[self.current].init();
            }
        }
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            if let Some(command) = self.commands.get_mut(self.current) {
                command.end(true);
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.current >= self.commands.len()
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.requirements.clone().into_iter().collect()
    }

    fn get_name(&self) -> String {
        self.commands
            .iter()
            .map(command::commands::CommandTrait::get_name)
            .collect::<Vec<_>>()
            .join("->")
    }
}

pub struct ProxyCommand {
    command_supplier: Box<dyn FnMut() -> Command>,
    command: Option<Box<Command>>,
}
impl CommandTrait for ProxyCommand {
    fn init(&mut self) {
        if self.command.is_none() {
            self.command = Some(Box::new((self.command_supplier)()));
        }
        self.command.as_mut().expect("Command Empty").init();
    }

    fn periodic(&mut self) {
        self.command.as_mut().expect("Command Empty").periodic();
    }

    fn end(&mut self, interrupted: bool) {
        self.command
            .as_mut()
            .expect("Command Empty")
            .end(interrupted);
    }

    fn is_finished(&mut self) -> bool {
        self.command.as_mut().expect("Command Empty").is_finished()
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.command
            .as_ref()
            .expect("Command Empty")
            .get_requirements()
    }

    fn get_name(&self) -> String {
        self.command.as_ref().expect("Command Empty").get_name()
    }
}
impl Debug for ProxyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut dbg_struct = f.debug_struct("ProxyCommand");
        if let Some(command) = &self.command {
            dbg_struct.field("command", command);
        } else {
            dbg_struct.field("command", &"None");
        };
        dbg_struct.finish()
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct WaitCommand {
    start_instant: Option<std::time::Instant>,
    duration: std::time::Duration,
}
impl CommandTrait for WaitCommand {
    fn init(&mut self) {
        self.start_instant = Some(std::time::Instant::now());
    }

    fn periodic(&mut self) {}

    fn end(&mut self, _interrupted: bool) {}

    fn is_finished(&mut self) -> bool {
        self.start_instant.expect("Command Empty").elapsed() >= self.duration
    }

    fn get_requirements(&self) -> Vec<u8> {
        vec![]
    }

    fn get_name(&self) -> String {
        format!("TimedCommand({:?})", self.duration)
    }
}

#[derive(Debug)]
pub struct NamedCommand {
    name: String,
    command: Box<Command>,
}
impl CommandTrait for NamedCommand {
    fn init(&mut self) {
        self.command.init();
    }

    fn periodic(&mut self) {
        self.command.periodic();
    }

    fn end(&mut self, interrupted: bool) {
        self.command.end(interrupted);
    }

    fn is_finished(&mut self) -> bool {
        self.command.is_finished()
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.command.get_requirements()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub enum Command {
    Parallel(ParallelBuiltCommand),
    Sequential(SequentialCommand),
    Simple(SimpleBuiltCommand),
    Custom(Box<dyn CommandTrait + Send>),
    Named(NamedCommand),
    Wait(WaitCommand),
    Proxy(ProxyCommand),
}
impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Parallel(command) => f
                .debug_struct("Parallel")
                .field("command", command)
                .finish(),
            Self::Sequential(command) => f
                .debug_struct("Sequential")
                .field("command", command)
                .finish(),
            Self::Simple(command) => f.debug_struct("Simple").field("command", command).finish(),
            Self::Custom(_) => f.debug_struct("Custom").finish(),
            Self::Named(command) => f.debug_struct("Named").field("command", command).finish(),
            Self::Wait(command) => f.debug_struct("Wait").field("command", command).finish(),
            Self::Proxy(command) => f.debug_struct("Proxy").field("command", command).finish(),
        }
    }
}
impl CommandTrait for Command {
    fn init(&mut self) {
        match self {
            Self::Parallel(command) => command.init(),
            Self::Sequential(command) => command.init(),
            Self::Simple(command) => command.init(),
            Self::Custom(command) => command.init(),
            Self::Named(command) => command.init(),
            Self::Wait(command) => command.init(),
            Self::Proxy(command) => command.init(),
        }
    }

    fn periodic(&mut self) {
        match self {
            Self::Parallel(command) => command.periodic(),
            Self::Sequential(command) => command.periodic(),
            Self::Simple(command) => command.periodic(),
            Self::Custom(command) => command.periodic(),
            Self::Named(command) => command.periodic(),
            Self::Wait(command) => command.periodic(),
            Self::Proxy(command) => command.periodic(),
        }
    }

    fn end(&mut self, interrupted: bool) {
        match self {
            Self::Parallel(command) => command.end(interrupted),
            Self::Sequential(command) => command.end(interrupted),
            Self::Simple(command) => command.end(interrupted),
            Self::Custom(command) => command.end(interrupted),
            Self::Named(command) => command.end(interrupted),
            Self::Wait(command) => command.end(interrupted),
            Self::Proxy(command) => command.end(interrupted),
        }
    }

    fn is_finished(&mut self) -> bool {
        match self {
            Self::Parallel(command) => command.is_finished(),
            Self::Sequential(command) => command.is_finished(),
            Self::Simple(command) => command.is_finished(),
            Self::Custom(command) => command.is_finished(),
            Self::Named(command) => command.is_finished(),
            Self::Wait(command) => command.is_finished(),
            Self::Proxy(command) => command.is_finished(),
        }
    }

    fn get_requirements(&self) -> Vec<u8> {
        match self {
            Self::Parallel(command) => command.get_requirements(),
            Self::Sequential(command) => command.get_requirements(),
            Self::Simple(command) => command.get_requirements(),
            Self::Custom(command) => command.get_requirements(),
            Self::Named(command) => command.get_requirements(),
            Self::Wait(command) => command.get_requirements(),
            Self::Proxy(command) => command.get_requirements(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            Self::Parallel(command) => command.get_name(),
            Self::Sequential(command) => command.get_name(),
            Self::Simple(command) => command.get_name(),
            Self::Custom(command) => command.get_name(),
            Self::Named(command) => command.get_name(),
            Self::Wait(command) => command.get_name(),
            Self::Proxy(command) => command.get_name(),
        }
    }
}
unsafe impl Send for Command {}

impl Command {
    #[must_use]
    pub fn along_with(self, other: Self) -> Self {
        Self::Parallel(ParallelBuiltCommand {
            requirements: self
                .get_requirements()
                .into_iter()
                .chain(other.get_requirements().into_iter())
                .collect(),
            commands: vec![self, other],
            finished: vec![false, false],
            race: false,
        })
    }

    #[must_use]
    pub fn along_with_many(self, others: Vec<Self>) -> Self {
        let mut commands = vec![self];
        commands.extend(others);
        Self::Parallel(ParallelBuiltCommand {
            finished: vec![false; commands.len()],
            requirements: commands
                .iter()
                .flat_map(command::commands::CommandTrait::get_requirements)
                .collect(),
            commands,
            race: false,
        })
    }

    #[must_use]
    pub fn race_with(self, other: Self) -> Self {
        Self::Parallel(ParallelBuiltCommand {
            requirements: self
                .get_requirements()
                .into_iter()
                .chain(other.get_requirements().into_iter())
                .collect(),
            commands: vec![self, other],
            finished: vec![false, false],
            race: true,
        })
    }

    #[must_use]
    pub fn race_with_many(self, others: Vec<Self>) -> Self {
        let mut commands = vec![self];
        commands.extend(others);
        Self::Parallel(ParallelBuiltCommand {
            finished: vec![false; commands.len()],
            requirements: commands
                .iter()
                .flat_map(command::commands::CommandTrait::get_requirements)
                .collect(),
            commands,
            race: true,
        })
    }

    #[must_use]
    pub fn before(self, other: Self) -> Self {
        Self::Sequential(SequentialCommand {
            requirements: self
                .get_requirements()
                .into_iter()
                .chain(other.get_requirements().into_iter())
                .collect(),
            commands: vec![self, other],
            current: 0,
        })
    }

    #[must_use]
    pub fn after(self, other: Self) -> Self {
        Self::Sequential(SequentialCommand {
            requirements: self
                .get_requirements()
                .into_iter()
                .chain(other.get_requirements().into_iter())
                .collect(),
            commands: vec![other, self],
            current: 0,
        })
    }

    #[must_use]
    pub fn and_then_many(self, others: Vec<Self>) -> Self {
        let mut commands = vec![self];
        commands.extend(others);
        Self::Sequential(SequentialCommand {
            requirements: commands
                .iter()
                .flat_map(command::commands::CommandTrait::get_requirements)
                .collect(),
            commands,
            current: 0,
        })
    }

    #[must_use]
    pub fn with_name(self, name: &str) -> Self {
        Self::Named(NamedCommand {
            name: String::from(name),
            command: Box::new(self),
        })
    }

    #[must_use]
    pub fn wait_for(self, seconds: f64) -> Self {
        Self::Wait(WaitCommand {
            duration: std::time::Duration::from_secs_f64(seconds),
            start_instant: None,
        })
    }

    #[must_use]
    pub fn custom(command: Box<dyn CommandTrait + Send>) -> Self {
        Self::Custom(command)
    }

    #[must_use]
    pub fn empty() -> Self {
        CommandBuilder::start_only(|| {}, vec![])
    }
}
impl Default for Command {
    fn default() -> Self {
        Self::empty()
    }
}
