use std::collections::HashSet;

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
        String::from("unamed command")
    }
}

pub struct CommandBuilder {
    init: Option<Box<dyn FnMut()>>,
    periodic: Option<Box<dyn FnMut()>>,
    end: Option<Box<dyn FnMut(bool)>>,
    is_finished: Option<Box<dyn FnMut() -> bool>>,
    requirements: Vec<u8>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self {
            init: None,
            periodic: None,
            end: None,
            is_finished: None,
            requirements: Vec::new(),
        }
    }

    pub fn init(mut self, init: impl FnMut() + 'static) -> Self {
        self.init = Some(Box::new(init));
        self
    }

    pub fn periodic(mut self, periodic: impl FnMut() + 'static) -> Self {
        self.periodic = Some(Box::new(periodic));
        self
    }

    pub fn end(mut self, end: impl FnMut(bool) + 'static) -> Self {
        self.end = Some(Box::new(end));
        self
    }

    pub fn is_finished(mut self, is_finished: impl FnMut() -> bool + 'static) -> Self {
        self.is_finished = Some(Box::new(is_finished));
        self
    }

    pub fn with_requirements(mut self, requirements: Vec<u8>) -> Self {
        self.requirements = requirements;
        self
    }

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
        CommandBuilder::new().init(init).with_requirements(requirements).build()
    }

    pub fn run_only(periodic: impl FnMut() + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().periodic(periodic).with_requirements(requirements).build()
    }

    pub fn end_only(end: impl FnMut(bool) + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().end(end).with_requirements(requirements).build()
    }

    pub fn run_start(init: impl FnMut() + 'static, periodic: impl FnMut() + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().init(init).periodic(periodic).with_requirements(requirements).build()
    }

    pub fn run_end(periodic: impl FnMut() + 'static, end: impl FnMut(bool) + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().periodic(periodic).end(end).with_requirements(requirements).build()
    }

    pub fn start_end(init: impl FnMut() + 'static, end: impl FnMut(bool) + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().init(init).end(end).with_requirements(requirements).build()
    }

    pub fn run_start_end(init: impl FnMut() + 'static, periodic: impl FnMut() + 'static, end: impl FnMut(bool) + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().init(init).periodic(periodic).end(end).with_requirements(requirements).build()
    }

    pub fn run_until(is_finished: impl FnMut() -> bool + 'static, periodic: impl FnMut() + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().is_finished(is_finished).periodic(periodic).with_requirements(requirements).build()
    }

    pub fn run_end_until(is_finished: impl FnMut() -> bool + 'static, periodic: impl FnMut() + 'static, end: impl FnMut(bool) + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().is_finished(is_finished).periodic(periodic).end(end).with_requirements(requirements).build()
    }

    pub fn start_run_until(init: impl FnMut() + 'static, is_finished: impl FnMut() -> bool + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().init(init).is_finished(is_finished).with_requirements(requirements).build()
    }

    pub fn all(init: impl FnMut() + 'static, periodic: impl FnMut() + 'static, end: impl FnMut(bool) + 'static, is_finished: impl FnMut() -> bool + 'static, requirements: Vec<u8>) -> Command {
        CommandBuilder::new().init(init).periodic(periodic).end(end).is_finished(is_finished).with_requirements(requirements).build()
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
        if let Some(is_finished) = self.is_finished.as_mut() {
            is_finished()
        } else {
            false
        }
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.requirements.clone()
    }
}
unsafe impl Send for SimpleBuiltCommand {}


pub struct ParallelBuiltCommand {
    commands: Vec<Command>,
    finished: Vec<bool>,
    requirements: HashSet<u8>,
    race: bool,
}
impl CommandTrait for ParallelBuiltCommand {
    fn init(&mut self) {
        for command in self.commands.iter_mut() {
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
        if !self.race {
            self.finished.iter().all(|&finished| finished)
        } else {
            self.finished.iter().any(|&finished| finished)
        }
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.requirements.clone().into_iter().collect()
    }

    fn get_name(&self) -> String {
        self.commands.iter().map(|command| command.get_name()).collect::<Vec<_>>().join(",")
    }
}
unsafe impl Send for ParallelBuiltCommand {}

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
            self.commands.get_mut(self.current)
                .map(|command| command.end(true));
        }
    }

    fn is_finished(&mut self) -> bool {
        self.current >= self.commands.len()
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.requirements.clone().into_iter().collect()
    }

    fn get_name(&self) -> String {
        self.commands.iter().map(|command| command.get_name()).collect::<Vec<_>>().join("->")
    }
}
unsafe impl Send for SequentialCommand {}

pub struct ProxyCommand {
    command_supplier: Box<dyn FnMut() -> Command>,
    command: Option<Command>,
}
impl CommandTrait for ProxyCommand {
    fn init(&mut self) {
        if self.command.is_none() {
            self.command = Some((self.command_supplier)());
        }
        self.command.as_mut().unwrap().init();
    }

    fn periodic(&mut self) {
        self.command.as_mut().unwrap().periodic();
    }

    fn end(&mut self, interrupted: bool) {
        self.command.as_mut().unwrap().end(interrupted);
    }

    fn is_finished(&mut self) -> bool {
        self.command.as_mut().unwrap().is_finished()
    }

    fn get_requirements(&self) -> Vec<u8> {
        self.command.as_ref().unwrap().get_requirements()
    }

    fn get_name(&self) -> String {
        self.command.as_ref().unwrap().get_name()
    }
}

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
        self.start_instant.unwrap().elapsed() >= self.duration
    }

    fn get_requirements(&self) -> Vec<u8> {
        vec![]
    }

    fn get_name(&self) -> String {
        format!("TimedCommand({:?})", self.duration)
    }
}

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
    Wait(WaitCommand)
}
impl CommandTrait for Command {
    fn init(&mut self) {
        match self {
            Command::Parallel(command) => command.init(),
            Command::Sequential(command) => command.init(),
            Command::Simple(command) => command.init(),
            Command::Custom(command) => command.init(),
            Command::Named(command) => command.init(),
            Command::Wait(command) => command.init(),
        }
    }

    fn periodic(&mut self) {
        match self {
            Command::Parallel(command) => command.periodic(),
            Command::Sequential(command) => command.periodic(),
            Command::Simple(command) => command.periodic(),
            Command::Custom(command) => command.periodic(),
            Command::Named(command) => command.periodic(),
            Command::Wait(command) => command.periodic(),
        }
    }

    fn end(&mut self, interrupted: bool) {
        match self {
            Command::Parallel(command) => command.end(interrupted),
            Command::Sequential(command) => command.end(interrupted),
            Command::Simple(command) => command.end(interrupted),
            Command::Custom(command) => command.end(interrupted),
            Command::Named(command) => command.end(interrupted),
            Command::Wait(command) => command.end(interrupted),
        }
    }

    fn is_finished(&mut self) -> bool {
        match self {
            Command::Parallel(command) => command.is_finished(),
            Command::Sequential(command) => command.is_finished(),
            Command::Simple(command) => command.is_finished(),
            Command::Custom(command) => command.is_finished(),
            Command::Named(command) => command.is_finished(),
            Command::Wait(command) => command.is_finished(),
        }
    }

    fn get_requirements(&self) -> Vec<u8> {
        match self {
            Command::Parallel(command) => command.get_requirements(),
            Command::Sequential(command) => command.get_requirements(),
            Command::Simple(command) => command.get_requirements(),
            Command::Custom(command) => command.get_requirements(),
            Command::Named(command) => command.get_requirements(),
            Command::Wait(command) => command.get_requirements(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            Command::Parallel(command) => command.get_name(),
            Command::Sequential(command) => command.get_name(),
            Command::Simple(command) => command.get_name(),
            Command::Custom(command) => command.get_name(),
            Command::Named(command) => command.get_name(),
            Command::Wait(command) => command.get_name(),
        }
    }
}
unsafe impl Send for Command {}
impl Command {
    pub fn along_with(self, other: Command) -> Command {
        Command::Parallel(
            ParallelBuiltCommand{
                requirements: self.get_requirements().into_iter().chain(other.get_requirements().into_iter()).collect(),
                commands: vec![self, other],
                finished: vec![false, false],
                race: false,
            })
    }

    pub fn along_with_many(self, others: Vec<Command>) -> Command {
        let mut commands = vec![self];
        commands.extend(others);
        Command::Parallel(
            ParallelBuiltCommand{
                finished: vec![false; commands.len()],
                requirements: commands.iter().map(|command| command.get_requirements()).flatten().collect(),
                commands,
                race: false,
            })
    }

    pub fn race_with(self, other: Command) -> Command {
        Command::Parallel(
            ParallelBuiltCommand{
                requirements: self.get_requirements().into_iter().chain(other.get_requirements().into_iter()).collect(),
                commands: vec![self, other],
                finished: vec![false, false],
                race: true,
            })
    }

    pub fn race_with_many(self, others: Vec<Command>) -> Command {
        let mut commands = vec![self];
        commands.extend(others);
        Command::Parallel(
            ParallelBuiltCommand{
                finished: vec![false; commands.len()],
                requirements: commands.iter().map(|command| command.get_requirements()).flatten().collect(),
                commands,
                race: true,
            })
    }

    pub fn before(self, other: Command) -> Command {
        Command::Sequential(
            SequentialCommand{
                requirements: self.get_requirements().into_iter().chain(other.get_requirements().into_iter()).collect(),
                commands: vec![self, other],
                current: 0,
            })
    }

    pub fn after(self, other: Command) -> Command {
        Command::Sequential(
            SequentialCommand{
                requirements: self.get_requirements().into_iter().chain(other.get_requirements().into_iter()).collect(),
                commands: vec![other, self],
                current: 0,
            })
    }

    pub fn and_then_many(self, others: Vec<Command>) -> Command{
        let mut commands = vec![self];
        commands.extend(others);
        Command::Sequential(
            SequentialCommand{
                requirements: commands.iter().map(|command| command.get_requirements()).flatten().collect(),
                commands,
                current: 0,
            })
    }

    pub fn with_name(self, name: &str) -> Command {
        Command::Named(
            NamedCommand{
                name: String::from(name),
                command: Box::new(self),
            })
    }

    pub fn wait_for(self, seconds: f64) -> Command {
        Command::Wait(
            WaitCommand{
                duration: std::time::Duration::from_secs_f64(seconds),
                start_instant: None,
            })
    }

    pub fn custom(command: Box<dyn CommandTrait + Send>) -> Command {
        Command::Custom(command)
    }

    pub fn empty() -> Command {
        CommandBuilder::start_only(|| {}, vec![])
    }
}