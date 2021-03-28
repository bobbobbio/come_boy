// Copyright 2018 Remi Bernotavicius

use std::collections::HashSet;
use std::{error, fmt, io, num, result, str};

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
    fn format<'a>(&self, _: &mut dyn io::Write) -> io::Result<()>;
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

impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
    input: &'a mut dyn io::BufRead,
    out: &'a mut dyn io::Write,
}

impl<'a> Debugger<'a> {
    pub fn new(
        input: &'a mut dyn io::BufRead,
        out: &'a mut dyn io::Write,
        emulator: &'a mut dyn DebuggerOps,
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

        return true;
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
        for (i, address) in (start_address..=end_address).into_iter().enumerate() {
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
            write!(self.out, "#{} 0x{:02x}\n", n, address).unwrap();
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
            return Err(ParseError::new(format!("unknown operand {}", operand)));
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
                return Err(ParseError::new(format!("unknown command {}", func)));
            }
        };
        Ok(())
    }

    fn dispatch_command(&mut self, command: &str, is_interrupted: &dyn Fn() -> bool) -> Result<()> {
        let mut command: String = command.into();
        if command == "" {
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

        let mut buffer = String::new();
        self.input.read_line(&mut buffer).unwrap();

        // If we got EOF, cleanly exit
        if buffer.len() == 0 {
            self.exit();
            return;
        }

        let command = &buffer[0..buffer.len() - 1];

        if let Err(e) = self.dispatch_command(command, is_interrupted) {
            write!(self.out, "Error: {}\n", &e.message).unwrap();
        };
    }

    pub fn run(&mut self, is_interrupted: &dyn Fn() -> bool) {
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
    call_stack: Vec<u16>,
}

#[cfg(test)]
impl TestDebuggerOps {
    fn new() -> TestDebuggerOps {
        TestDebuggerOps {
            current_address: 0,
            memory: HashMap::new(),
            crash_message: None,
            memory_changed: HashSet::new(),
            call_stack: Vec::new(),
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

    fn format<'a>(&self, s: &'a mut dyn io::Write) -> io::Result<()> {
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

    fn disassemble(&mut self, address: u16, f: &mut dyn io::Write) -> io::Result<()> {
        write!(f, "assembly at {:02x}", address)
    }

    fn read_call_stack(&self) -> Vec<u16> {
        self.call_stack.clone()
    }
}

#[cfg(test)]
fn run_debugger_test_with_ops(ops: &mut dyn DebuggerOps, input: &[&str], expected_output: &str) {
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

#[cfg(test)]
struct DebuggerTest {
    input: Vec<&'static str>,
    expected_log: String,
    ops: TestDebuggerOps,
}

#[cfg(test)]
impl DebuggerTest {
    fn new() -> Self {
        DebuggerTest {
            input: vec![],
            expected_log: String::new(),
            ops: TestDebuggerOps::new(),
        }
    }

    fn run_command(&mut self, cmd: &'static str) {
        self.expected_log += "(debugger) ";
        self.input.push(cmd);
    }

    fn expect_line(&mut self, line: &str) {
        self.expected_log += &format!("{}\n", line);
    }

    fn expect_state(&mut self, pc: u16) {
        self.expect_line(&format!("TestDebuggerOps pc={}", pc));
    }

    fn expect_error(&mut self, msg: &str) {
        self.expect_line(&format!("Error: {}", msg));
    }

    fn expect_breakpoint(&mut self) {
        self.expect_line("Hit breakpoint");
    }

    fn expect_watchpoint(&mut self) {
        self.expect_line("Hit watchpoint");
    }

    fn run(&mut self) {
        self.expect_line("(debugger) exiting");

        run_debugger_test_with_ops(&mut self.ops, &self.input[..], &self.expected_log);
    }
}

#[cfg(test)]
impl Drop for DebuggerTest {
    fn drop(&mut self) {
        self.run()
    }
}

#[test]
fn debugger_state() {
    let mut test = DebuggerTest::new();

    test.run_command("state");
    test.expect_state(0);
}

#[test]
fn debugger_next() {
    let mut test = DebuggerTest::new();

    test.run_command("next");
    test.expect_state(1);
}

#[test]
fn debugger_next_multiple() {
    let mut test = DebuggerTest::new();

    test.run_command("next 2");
    test.expect_state(2);
}

#[test]
fn debugger_next_invalid_input() {
    let mut test = DebuggerTest::new();

    test.run_command("next -1");
    test.expect_error("invalid digit found in string");
}

#[test]
fn debugger_next_zero_is_invalid_input() {
    let mut test = DebuggerTest::new();

    test.run_command("next 0");
    test.expect_error("invalid argument");
}

#[test]
fn debugger_exit() {
    run_debugger_test(&["exit"], "(debugger) exiting\n");
}

#[test]
fn debugger_stops_on_breakpoint_when_calling_next() {
    let mut test = DebuggerTest::new();

    test.run_command("break 2");

    test.run_command("next");
    test.expect_state(1);

    test.run_command("next");
    test.expect_state(2);
    test.expect_breakpoint();
}

#[test]
fn debugger_stops_on_breakpoint_when_calling_run() {
    let mut test = DebuggerTest::new();

    test.run_command("break 2");
    test.run_command("run");
    test.expect_breakpoint();
    test.expect_state(2);
}

#[test]
fn debugger_stops_on_watchpoint_when_calling_run() {
    let mut test = DebuggerTest::new();
    test.ops.memory_changed.insert(0x3);

    test.run_command("watch 3");
    test.run_command("run");
    test.expect_watchpoint();
    test.expect_state(1);
}

#[test]
fn debugger_errors_when_setting_breakpoint_without_address() {
    let mut test = DebuggerTest::new();
    test.ops.memory_changed.insert(0x3);

    test.run_command("break");
    test.expect_error("provide an address");
}

#[test]
fn debugger_errors_when_setting_breakpoint_without_invalid_address() {
    let mut test = DebuggerTest::new();
    test.ops.memory_changed.insert(0x3);

    test.run_command("break derp");
    test.expect_error("invalid digit found in string");
}

#[test]
fn debugger_errors_when_given_unknown_command() {
    let mut test = DebuggerTest::new();
    test.ops.memory_changed.insert(0x3);

    test.run_command("derp");
    test.expect_error("unknown command derp");
}

#[test]
fn debugger_repeats_last_command() {
    let mut test = DebuggerTest::new();

    test.run_command("next");
    test.expect_state(1);

    test.run_command("");
    test.expect_state(2);
}

#[test]
fn debugger_errors_on_empty_command() {
    let mut test = DebuggerTest::new();

    test.run_command("");
    test.expect_error("empty command");
}

#[test]
fn debugger_stops_when_emulator_crashes() {
    let mut test = DebuggerTest::new();
    test.ops.crash_message = Some("test crash".into());

    test.run_command("run");
    test.expect_line("Emulator crashed: test crash");
    test.expect_state(0);
}

#[test]
fn debugger_can_set_current_address() {
    let mut test = DebuggerTest::new();

    test.run_command("set pc 45");
    test.run_command("state");
    test.expect_state(45);
}

#[test]
fn debugger_set_invalid_input() {
    let mut test = DebuggerTest::new();

    test.run_command("set");
    test.expect_error("provide an operand");

    test.run_command("set poot");
    test.expect_error("unknown operand poot");

    test.run_command("set pc asdfd");
    test.expect_error("invalid digit found in string");
}

#[test]
fn debugger_can_enable_logging() {
    let mut test = DebuggerTest::new();

    test.run_command("logging enable");
    test.run_command("break 2");
    test.run_command("run");

    test.expect_state(1);
    test.expect_state(2);
    test.expect_breakpoint();
}

#[test]
fn debugger_can_disable_logging() {
    let mut test = DebuggerTest::new();

    test.run_command("logging enable");
    test.run_command("logging disable");
    test.run_command("break 2");
    test.run_command("run");

    test.expect_breakpoint();
    test.expect_state(2);
}

#[test]
fn debugger_logging_invalid_input() {
    let mut test = DebuggerTest::new();

    test.run_command("logging derp");
    test.expect_error("Choices are 'enable' or 'disable'");

    test.run_command("logging");
    test.expect_error("Choices are 'enable' or 'disable'");
}

#[test]
fn debugger_extra_input_fails() {
    let commands = &[
        "backtrace xxx",
        "break 1 xxx",
        "disassemble 123 xxx",
        "exit xxx",
        "logging enable xxx",
        "next 1 xxx",
        "run xxx",
        "set pc 12 xxx",
        "state xxx",
        "watch 3 xxx",
        "x 2 xxx",
    ];

    let mut test = DebuggerTest::new();
    for command in commands {
        test.run_command(command);
        test.expect_error("extra input");
    }
}

#[test]
fn debugger_multiple_commands() {
    let mut test = DebuggerTest::new();

    test.run_command("state && next");
    test.expect_state(0);
    test.expect_state(1);

    test.run_command("next && next && next");
    test.expect_state(2);
    test.expect_state(3);
    test.expect_state(4);

    test.run_command("");
    test.expect_state(5);
    test.expect_state(6);
    test.expect_state(7);
}

#[test]
fn debugger_examine_memory() {
    let mut test = DebuggerTest::new();

    test.run_command("x 2");

    test.expect_line("0000002:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000012:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000022:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000032:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000042:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000052:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000062:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000072:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000082:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000092:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("00000a2:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("00000b2:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("00000c2:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("00000d2:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("00000e2:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("00000f2:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000102:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000112:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000122:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
    test.expect_line("0000132:  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00");
}

#[test]
fn debugger_examine_memory_missing_address() {
    let mut test = DebuggerTest::new();

    test.run_command("x");
    test.expect_error("provide an address");
}

#[test]
fn debugger_backtrace() {
    let mut test = DebuggerTest::new();
    test.ops.current_address = 0x1121;
    test.ops
        .call_stack
        .append(&mut vec![0x1234, 0x5678, 0x9abc]);

    test.run_command("backtrace");
    test.expect_line("#0 0x1121");
    test.expect_line("#1 0x9abc");
    test.expect_line("#2 0x5678");
    test.expect_line("#3 0x1234");
}

#[test]
fn debugger_empty_backtrace() {
    let mut test = DebuggerTest::new();
    test.ops.current_address = 0x1121;

    test.run_command("backtrace");
    test.expect_line("#0 0x1121");
}

#[test]
fn debugger_disassemble_address() {
    let mut test = DebuggerTest::new();

    test.run_command("disassemble 10");
    test.expect_line("assembly at 10");
}

#[test]
fn debugger_disassemble() {
    let mut test = DebuggerTest::new();

    test.run_command("disassemble");
    test.expect_line("assembly at 00");
}
