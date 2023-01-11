// Copyright 2018 Remi Bernotavicius

use crate::io;
use alloc::collections::BTreeSet;
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, num, result, str};

pub struct SimulatedInstruction {
    memory_changed: BTreeSet<u16>,
}

impl SimulatedInstruction {
    fn new() -> SimulatedInstruction {
        SimulatedInstruction {
            memory_changed: BTreeSet::new(),
        }
    }

    pub fn set_memory(&mut self, address: u16, _value: u8) {
        self.memory_changed.insert(address);
    }
}

pub trait DebuggerOps {
    fn read_memory(&self, address: u16) -> u8;
    fn format(&self, _: &mut dyn io::Write) -> io::Result<()>;
    fn next(&mut self);
    fn simulate_next(&mut self, _: &mut SimulatedInstruction);
    fn read_program_counter(&self) -> u16;
    fn read_call_stack(&self) -> Vec<u16>;
    fn crashed(&self) -> Option<&String>;
    fn set_program_counter(&mut self, address: u16);
    fn disassemble(&mut self, address: u16, f: &mut dyn io::Write) -> io::Result<()>;
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(e: num::ParseIntError) -> Self {
        ParseError {
            message: e.to_string(),
        }
    }
}

type Result<T> = result::Result<T, ParseError>;

pub struct Debugger<'a> {
    emulator: &'a mut dyn DebuggerOps,
    running: bool,
    last_command: String,
    breakpoint: Option<u16>,
    watchpoint: Option<u16>,
    logging: bool,
    input: &'a mut dyn Iterator<Item = io::Result<String>>,
    out: &'a mut dyn io::Write,
}

impl<'a> Debugger<'a> {
    pub fn new(
        input: &'a mut dyn Iterator<Item = io::Result<String>>,
        out: &'a mut dyn io::Write,
        emulator: &'a mut dyn DebuggerOps,
    ) -> Debugger<'a> {
        Debugger {
            emulator,
            running: false,
            last_command: String::new(),
            breakpoint: None,
            watchpoint: None,
            logging: false,
            input,
            out,
        }
    }

    fn state(&mut self) {
        self.emulator.format(self.out).unwrap();
        writeln!(self.out).unwrap();
    }

    fn disassemble(&mut self, address: u16) {
        self.emulator.disassemble(address, self.out).unwrap();
        writeln!(self.out).unwrap();
    }

    fn check_for_watchpoint(&mut self) -> bool {
        if self.watchpoint.is_some() {
            let mut instruction = SimulatedInstruction::new();
            self.emulator.simulate_next(&mut instruction);
            let address = self.watchpoint.unwrap();
            if instruction.memory_changed.contains(&address) {
                writeln!(self.out, "Hit watchpoint").unwrap();
                return false;
            }
        }

        true
    }

    fn check_for_breakpoint_crash_or_interrupt(
        &mut self,
        is_interrupted: &dyn Fn() -> bool,
    ) -> bool {
        if self.emulator.crashed().is_some() {
            writeln!(
                self.out,
                "Emulator crashed: {}",
                self.emulator.crashed().unwrap()
            )
            .unwrap();
            return false;
        }
        if Some(self.emulator.read_program_counter()) == self.breakpoint {
            writeln!(self.out, "Hit breakpoint").unwrap();
            return false;
        }
        if is_interrupted() {
            writeln!(self.out, "Interrupted").unwrap();
            return false;
        }
        true
    }

    fn next(&mut self, times: usize) {
        assert!(times > 0);

        for _ in 0..(times - 1) {
            self.check_for_watchpoint();
            self.emulator.next();
            self.check_for_breakpoint_crash_or_interrupt(&|| false);
        }

        self.check_for_watchpoint();
        self.emulator.next();
        self.state();
        self.check_for_breakpoint_crash_or_interrupt(&|| false);
    }

    fn exit(&mut self) {
        self.running = false;
        writeln!(self.out, "exiting").unwrap();
    }

    fn set_breakpoint(&mut self, address: u16) {
        self.breakpoint = Some(address);
    }

    fn set_watchpoint(&mut self, address: u16) {
        self.watchpoint = Some(address);
    }

    fn run_emulator(&mut self, is_interrupted: &dyn Fn() -> bool) {
        self.emulator.next();
        if self.logging {
            self.state();
        }
        while self.check_for_breakpoint_crash_or_interrupt(is_interrupted)
            && self.check_for_watchpoint()
        {
            self.emulator.next();
            if self.logging {
                self.state();
            }
        }
        if !self.logging {
            self.state();
        }
    }

    fn parse_string<'b>(
        &mut self,
        iter: &mut dyn Iterator<Item = &'b str>,
        message: &str,
    ) -> Result<&'b str> {
        match iter.next() {
            None => Err(ParseError::new(message.into())),
            Some(x) => Ok(x),
        }
    }

    fn parse_address(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<u16> {
        Ok(u16::from_str_radix(
            self.parse_string(iter, "provide an address")?,
            16,
        )?)
    }

    fn examine_memory(&mut self, start_address: u16) {
        let width = 16usize;
        let height = 20usize;

        let end_address = start_address.saturating_add((width * height - 1) as u16);
        for (i, address) in (start_address..=end_address).enumerate() {
            if i % width == 0 {
                write!(
                    self.out,
                    "{}{:07x}:",
                    if i == 0 { "" } else { "\n" },
                    address
                )
                .unwrap();
            }
            if i % 8 == 0 {
                write!(self.out, " ").unwrap();
            }
            write!(self.out, " {:02x}", self.emulator.read_memory(address)).unwrap();
        }
        writeln!(self.out).unwrap();
    }

    fn enable_logging(&mut self) {
        self.logging = true;
    }

    fn disable_logging(&mut self) {
        self.logging = false;
    }

    fn backtrace(&mut self) {
        let mut frames = self.emulator.read_call_stack();
        frames.push(self.emulator.read_program_counter());
        for (n, address) in frames.into_iter().rev().enumerate() {
            writeln!(self.out, "#{n} 0x{address:02x}").unwrap();
        }
    }

    fn parse_backtrace(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        self.parse_end(iter)?;
        self.backtrace();
        Ok(())
    }

    fn parse_end(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        if iter.next().is_some() {
            Err(ParseError::new("extra input".into()))
        } else {
            Ok(())
        }
    }

    fn parse_state(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        self.parse_end(iter)?;
        self.state();
        Ok(())
    }

    fn parse_disassemble(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let mut address = self.emulator.read_program_counter();
        if let Some(v) = iter.next() {
            address = u16::from_str_radix(v, 16)?;
        }
        self.parse_end(iter)?;
        self.disassemble(address);
        Ok(())
    }

    fn parse_next(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let mut times = 1;
        if let Some(v) = iter.next() {
            times = v.parse()?;
        };
        self.parse_end(iter)?;

        if times == 0 {
            return Err(ParseError::new("invalid argument".into()));
        }
        self.next(times);
        Ok(())
    }

    fn parse_exit(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        self.parse_end(iter)?;
        self.exit();
        Ok(())
    }

    fn parse_run(
        &mut self,
        iter: &mut dyn Iterator<Item = &str>,
        is_interrupted: &dyn Fn() -> bool,
    ) -> Result<()> {
        self.parse_end(iter)?;
        self.run_emulator(is_interrupted);
        Ok(())
    }

    fn parse_break(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let address = self.parse_address(iter)?;
        self.parse_end(iter)?;

        self.set_breakpoint(address);
        Ok(())
    }

    fn parse_watch(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let address = self.parse_address(iter)?;
        self.parse_end(iter)?;

        self.set_watchpoint(address);
        Ok(())
    }

    fn parse_logging(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let help_msg = "Choices are 'enable' or 'disable'";
        let command = self.parse_string(iter, help_msg)?;
        self.parse_end(iter)?;

        match command {
            "enable" => self.enable_logging(),
            "disable" => self.disable_logging(),
            _ => {
                return Err(ParseError::new(help_msg.into()));
            }
        };
        Ok(())
    }

    fn parse_set(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let operand = self.parse_string(iter, "provide an operand")?;
        if operand != "pc" {
            return Err(ParseError::new(format!("unknown operand {operand}")));
        }
        let address = self.parse_address(iter)?;
        self.parse_end(iter)?;

        self.emulator.set_program_counter(address);
        Ok(())
    }

    fn parse_x(&mut self, iter: &mut dyn Iterator<Item = &str>) -> Result<()> {
        let address = self.parse_address(iter)?;
        self.parse_end(iter)?;

        self.examine_memory(address);
        Ok(())
    }

    fn dispatch_command_inner(
        &mut self,
        command: &str,
        is_interrupted: &dyn Fn() -> bool,
    ) -> Result<()> {
        let mut iter = command.split_whitespace();
        let func = self.parse_string(&mut iter, "empty command")?;
        match func {
            "backtrace" => self.parse_backtrace(&mut iter)?,
            "break" => self.parse_break(&mut iter)?,
            "disassemble" => self.parse_disassemble(&mut iter)?,
            "exit" => self.parse_exit(&mut iter)?,
            "logging" => self.parse_logging(&mut iter)?,
            "next" => self.parse_next(&mut iter)?,
            "run" => self.parse_run(&mut iter, is_interrupted)?,
            "set" => self.parse_set(&mut iter)?,
            "state" => self.parse_state(&mut iter)?,
            "watch" => self.parse_watch(&mut iter)?,
            "x" => self.parse_x(&mut iter)?,
            _ => {
                return Err(ParseError::new(format!("unknown command {func}")));
            }
        };
        Ok(())
    }

    fn dispatch_command(&mut self, command: &str, is_interrupted: &dyn Fn() -> bool) -> Result<()> {
        let mut command: String = command.into();
        if command.is_empty() {
            command = self.last_command.clone();
        }

        for command in command.split(" && ") {
            self.dispatch_command_inner(command, is_interrupted)?;
        }

        self.last_command = command;

        Ok(())
    }

    fn process_command(&mut self, is_interrupted: &dyn Fn() -> bool) {
        write!(self.out, "(debugger) ").unwrap();
        self.out.flush().unwrap();

        if let Some(res) = self.input.next() {
            let command = res.unwrap();

            if let Err(e) = self.dispatch_command(&command, is_interrupted) {
                writeln!(self.out, "Error: {}", &e.message).unwrap();
            }
        } else {
            // If we got EOF, cleanly exit
            self.exit();
        }
    }

    pub fn run(&mut self, is_interrupted: &dyn Fn() -> bool) {
        self.running = true;
        while self.running {
            self.process_command(is_interrupted);
        }
    }
}

#[cfg(test)]
mod tests;
