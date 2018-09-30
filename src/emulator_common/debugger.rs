// Copyright 2018 Remi Bernotavicius

use std::collections::HashSet;
use std::io::{self, Result};
use std::str;

#[cfg(test)]
use std::collections::HashMap;

pub struct SimulatedInstruction {
    memory_changed: HashSet<u16>,
}

impl SimulatedInstruction {
    fn new() -> SimulatedInstruction {
        SimulatedInstruction {
            memory_changed: HashSet::new(),
        }
    }

    pub fn set_memory(&mut self, address: u16, _value: u8) {
        self.memory_changed.insert(address);
    }
}

pub trait DebuggerOps {
    fn read_memory(&self, address: u16) -> u8;
    fn format<'a>(&self, &mut io::Write) -> Result<()>;
    fn next(&mut self);
    fn simulate_next(&mut self, &mut SimulatedInstruction);
    fn read_program_counter(&self) -> u16;
    fn crashed(&self) -> Option<&String>;
    fn set_program_counter(&mut self, address: u16);
    fn disassemble(&mut self, f: &mut io::Write) -> Result<()>;
}

pub struct Debugger<'a> {
    emulator: &'a mut DebuggerOps,
    running: bool,
    last_command: String,
    breakpoint: Option<u16>,
    watchpoint: Option<u16>,
    logging: bool,
    input: &'a mut io::BufRead,
    out: &'a mut io::Write,
}

impl<'a> Debugger<'a> {
    pub fn new(
        input: &'a mut io::BufRead,
        out: &'a mut io::Write,
        emulator: &'a mut DebuggerOps,
    ) -> Debugger<'a> {
        Debugger {
            emulator: emulator,
            running: false,
            last_command: String::new(),
            breakpoint: None,
            watchpoint: None,
            logging: false,
            input: input,
            out: out,
        }
    }

    fn state(&mut self) {
        self.emulator.format(self.out).unwrap();
        writeln!(self.out).unwrap();
    }

    fn disassemble(&mut self) {
        self.emulator.disassemble(self.out).unwrap();
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

        return true;
    }

    fn check_for_breakpoint_crash_or_interrupt(&mut self, is_interrupted: &Fn() -> bool) -> bool {
        if self.emulator.crashed().is_some() {
            writeln!(
                self.out,
                "Emulator crashed: {}",
                self.emulator.crashed().unwrap()
            ).unwrap();
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
        return true;
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

    fn run_emulator(&mut self, is_interrupted: &Fn() -> bool) {
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

    fn read_address(&mut self, iter: &mut str::SplitWhitespace) -> Option<u16> {
        match iter.next() {
            None => {
                writeln!(self.out, "provide an address").unwrap();
                return None;
            }
            Some(x) => match u16::from_str_radix(x, 16) {
                Err(_) => {
                    writeln!(self.out, "{} is not a valid address", x).unwrap();
                    return None;
                }
                Ok(x) => Some(x),
            },
        }
    }

    fn examine_memory(&mut self, start_address: u16) {
        let width = 16usize;
        let height = 20usize;

        let end_address = start_address.saturating_add((width * height - 1) as u16);
        for (i, address) in (start_address..=end_address).into_iter().enumerate() {
            if i % width == 0 {
                write!(
                    self.out,
                    "{}{:07x}:",
                    if i == 0 { "" } else { "\n" },
                    address
                ).unwrap();
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

    fn dispatch_command_inner(&mut self, command: &str, is_interrupted: &Fn() -> bool) {
        let mut iter = command.split_whitespace();
        let func = match iter.next() {
            None => "",
            Some(x) => x,
        };
        match func {
            "state" => self.state(),
            "disassemble" => self.disassemble(),
            "next" => self.next(iter.next().map_or(1, |v| v.parse().unwrap_or(1))),
            "exit" => self.exit(),
            "run" => self.run_emulator(is_interrupted),
            "break" => match self.read_address(&mut iter) {
                Some(address) => self.set_breakpoint(address),
                None => {}
            },
            "watch" => match self.read_address(&mut iter) {
                Some(address) => self.set_watchpoint(address),
                None => {}
            },
            "logging" => match iter.next() {
                Some("enable") => self.enable_logging(),
                Some("disable") => self.disable_logging(),
                _ => {
                    writeln!(self.out, "Choices are 'enable' or 'disable'").unwrap();
                    return;
                }
            },
            "set" => {
                match iter.next() {
                    Some("pc") => {}
                    Some(o) => {
                        writeln!(self.out, "Unknown operand {}", o).unwrap();
                        return;
                    }
                    None => {
                        writeln!(self.out, "Missing operand").unwrap();
                        return;
                    }
                }
                match self.read_address(&mut iter) {
                    Some(address) => self.emulator.set_program_counter(address),
                    None => {
                        writeln!(self.out, "Missing operand").unwrap();
                        return;
                    }
                }
            }
            "x" => match self.read_address(&mut iter) {
                Some(address) => self.examine_memory(address),
                None => {
                    writeln!(self.out, "Missing operand").unwrap();
                    return;
                }
            },
            "" => {
                let c = self.last_command.clone();
                if c == "" {
                    writeln!(self.out, "empty command").unwrap();
                } else {
                    self.dispatch_command(&c, is_interrupted);
                }
                return;
            }
            _ => {
                writeln!(self.out, "Unknown command {}", func).unwrap();
                return;
            }
        }
    }

    fn dispatch_command(&mut self, command: &str, is_interrupted: &Fn() -> bool) {
        for command in command.split(" && ") {
            self.dispatch_command_inner(command, is_interrupted);
        }
        if command != "" {
            self.last_command = command.into();
        }
    }

    fn process_command(&mut self, is_interrupted: &Fn() -> bool) {
        write!(self.out, "(debugger) ").unwrap();
        self.out.flush().unwrap();

        let mut buffer = String::new();
        self.input.read_line(&mut buffer).unwrap();

        // If we got EOF, cleanly exit
        if buffer.len() == 0 {
            self.exit();
            return;
        }

        let command = &buffer[0..buffer.len() - 1];

        self.dispatch_command(command, is_interrupted);
    }

    pub fn run(&mut self, is_interrupted: &Fn() -> bool) {
        self.running = true;
        while self.running {
            self.process_command(is_interrupted);
        }
    }
}

#[cfg(test)]
struct TestDebuggerOps {
    current_address: u16,
    memory: HashMap<u16, u8>,
    crash_message: Option<String>,
    memory_changed: HashSet<u16>,
}

#[cfg(test)]
impl TestDebuggerOps {
    fn new() -> TestDebuggerOps {
        TestDebuggerOps {
            current_address: 0,
            memory: HashMap::new(),
            crash_message: None,
            memory_changed: HashSet::new(),
        }
    }
}

#[cfg(test)]
impl DebuggerOps for TestDebuggerOps {
    fn read_memory(&self, address: u16) -> u8 {
        match self.memory.get(&address) {
            Some(data) => *data,
            None => 0,
        }
    }

    fn format<'a>(&self, s: &'a mut io::Write) -> Result<()> {
        write!(s, "TestDebuggerOps pc={:x}", self.current_address)
    }

    fn next(&mut self) {
        if self.crashed().is_none() {
            self.current_address += 1;
        }
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction) {
        for address in &self.memory_changed {
            instruction.set_memory(*address, 0);
        }
    }

    fn read_program_counter(&self) -> u16 {
        self.current_address
    }

    fn crashed(&self) -> Option<&String> {
        self.crash_message.as_ref()
    }

    fn set_program_counter(&mut self, address: u16) {
        self.current_address = address
    }

    fn disassemble(&mut self, _f: &mut io::Write) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
fn run_debugger_test_with_ops(ops: &mut DebuggerOps, input: &[&str], expected_output: &str) {
    let mut output_bytes = vec![];
    let input_str = input.join("\n") + "\n";
    let mut input_bytes = input_str.as_bytes();
    {
        let mut debugger = Debugger::new(&mut input_bytes, &mut output_bytes, ops);
        debugger.run(&|| false);
    }

    assert_eq!(str::from_utf8(&output_bytes).unwrap(), expected_output);
}

#[cfg(test)]
fn run_debugger_test(input: &[&str], expected_output: &str) {
    let mut test_ops = TestDebuggerOps::new();
    run_debugger_test_with_ops(&mut test_ops, input, expected_output)
}

#[test]
fn debugger_interrupt() {
    let mut test_ops = TestDebuggerOps::new();
    let mut output_bytes = vec![];
    let mut input_bytes = "run\n".as_bytes();
    {
        let mut debugger = Debugger::new(&mut input_bytes, &mut output_bytes, &mut test_ops);
        debugger.run(&|| true);
    }

    assert_eq!(
        str::from_utf8(&output_bytes).unwrap(),
        "(debugger) Interrupted\n\
         TestDebuggerOps pc=1\n\
         (debugger) exiting\n"
    );
}

#[test]
fn debugger_state() {
    run_debugger_test(
        &["state"],
        "\
         (debugger) \
         TestDebuggerOps pc=0\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_next() {
    run_debugger_test(
        &["next"],
        "\
         (debugger) \
         TestDebuggerOps pc=1\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_exit() {
    run_debugger_test(
        &["exit"],
        "\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_stops_on_breakpoint_when_calling_next() {
    run_debugger_test(
        &["break 2", "next", "next"],
        "\
         (debugger) \
         (debugger) \
         TestDebuggerOps pc=1\n\
         (debugger) \
         TestDebuggerOps pc=2\n\
         Hit breakpoint\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_stops_on_breakpoint_when_calling_run() {
    run_debugger_test(
        &["break 2", "run"],
        "\
         (debugger) \
         (debugger) \
         Hit breakpoint\n\
         TestDebuggerOps pc=2\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_stops_on_watchpoint_when_calling_run() {
    let mut test_ops = TestDebuggerOps::new();
    test_ops.memory_changed.insert(0x3);
    run_debugger_test_with_ops(
        &mut test_ops,
        &["watch 3", "run"],
        "\
         (debugger) \
         (debugger) \
         Hit watchpoint\n\
         TestDebuggerOps pc=1\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_errors_when_setting_breakpoint_without_address() {
    run_debugger_test(
        &["break"],
        "\
         (debugger) \
         provide an address\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_errors_when_setting_breakpoint_without_invalid_address() {
    run_debugger_test(
        &["break derp"],
        "\
         (debugger) \
         derp is not a valid address\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_errors_when_given_unknown_command() {
    run_debugger_test(
        &["derp"],
        "\
         (debugger) \
         Unknown command derp\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_repeats_last_command() {
    run_debugger_test(
        &["next", ""],
        "\
         (debugger) \
         TestDebuggerOps pc=1\n\
         (debugger) \
         TestDebuggerOps pc=2\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_errors_on_empty_command() {
    run_debugger_test(
        &[""],
        "\
         (debugger) \
         empty command\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_stops_when_emulator_crashes() {
    let mut test_ops = TestDebuggerOps::new();
    test_ops.crash_message = Some("test crash".to_string());
    run_debugger_test_with_ops(
        &mut test_ops,
        &["run"],
        "\
         (debugger) \
         Emulator crashed: test crash\n\
         TestDebuggerOps pc=0\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_can_set_current_address() {
    run_debugger_test(
        &["set pc 45", "state"],
        "\
         (debugger) \
         (debugger) \
         TestDebuggerOps pc=45\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_can_enable_logging() {
    run_debugger_test(
        &["logging enable", "break 2", "run"],
        "\
         (debugger) \
         (debugger) \
         (debugger) \
         TestDebuggerOps pc=1\n\
         TestDebuggerOps pc=2\n\
         Hit breakpoint\n\
         (debugger) \
         exiting\n\
         ",
    );
}

#[test]
fn debugger_can_disable_logging() {
    run_debugger_test(
        &["logging enable", "logging disable", "break 2", "run"],
        "\
         (debugger) \
         (debugger) \
         (debugger) \
         (debugger) \
         Hit breakpoint\n\
         TestDebuggerOps pc=2\n\
         (debugger) \
         exiting\n\
         ",
    );
}
