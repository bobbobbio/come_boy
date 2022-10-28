// copyright 2021 Remi Bernotavicius

use super::{Debugger, DebuggerOps, SimulatedInstruction};
use std::collections::{BTreeSet, HashMap};
use std::{io, str};

struct TestDebuggerOps {
    current_address: u16,
    memory: HashMap<u16, u8>,
    crash_message: Option<String>,
    memory_changed: BTreeSet<u16>,
    call_stack: Vec<u16>,
}

impl TestDebuggerOps {
    fn new() -> TestDebuggerOps {
        TestDebuggerOps {
            current_address: 0,
            memory: HashMap::new(),
            crash_message: None,
            memory_changed: BTreeSet::new(),
            call_stack: Vec::new(),
        }
    }
}

impl DebuggerOps for TestDebuggerOps {
    fn read_memory(&self, address: u16) -> u8 {
        match self.memory.get(&address) {
            Some(data) => *data,
            None => 0,
        }
    }

    fn format<'a>(&self, s: &mut dyn io::Write) -> io::Result<()> {
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
        write!(f, "assembly at {address:02x}")
    }

    fn read_call_stack(&self) -> Vec<u16> {
        self.call_stack.clone()
    }
}

fn run_debugger_test_with_ops(ops: &mut dyn DebuggerOps, input: &[&str], expected_output: &str) {
    let mut output_bytes = vec![];
    let mut input_iter = input.iter().map(|l| Ok(l.to_string()));
    println!("{input:?}");
    {
        let mut debugger = Debugger::new(&mut input_iter, &mut output_bytes, ops);
        debugger.run(&|| false);
    }

    assert_eq!(str::from_utf8(&output_bytes).unwrap(), expected_output);
}

fn run_debugger_test(input: &[&str], expected_output: &str) {
    let mut test_ops = TestDebuggerOps::new();
    run_debugger_test_with_ops(&mut test_ops, input, expected_output)
}

#[test]
fn debugger_interrupt() {
    let mut test_ops = TestDebuggerOps::new();
    let mut output_bytes = vec![];
    let mut input_iter = std::iter::once(Ok("run".to_string()));
    {
        let mut debugger = Debugger::new(&mut input_iter, &mut output_bytes, &mut test_ops);
        debugger.run(&|| true);
    }

    assert_eq!(
        str::from_utf8(&output_bytes).unwrap(),
        "(debugger) Interrupted\n\
         TestDebuggerOps pc=1\n\
         (debugger) exiting\n"
    );
}

struct DebuggerTest {
    input: Vec<&'static str>,
    expected_log: String,
    ops: TestDebuggerOps,
}

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
        self.expected_log += &format!("{line}\n");
    }

    fn expect_state(&mut self, pc: u16) {
        self.expect_line(&format!("TestDebuggerOps pc={pc}"));
    }

    fn expect_error(&mut self, msg: &str) {
        self.expect_line(&format!("Error: {msg}"));
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
