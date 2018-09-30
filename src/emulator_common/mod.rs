// Copyright 2017 Remi Bernotavicius

use std::collections::HashSet;
use std::io::{self, Result};
use std::ops::Range;
use std::str;

#[cfg(test)]
use std::collections::HashMap;

/*  ___       _       _  ___   ___   ___   ___  ____            _     _
 * |_ _|_ __ | |_ ___| |( _ ) / _ \ ( _ ) / _ \|  _ \ ___  __ _(_)___| |_ ___ _ __
 *  | || '_ \| __/ _ \ |/ _ \| | | |/ _ \| | | | |_) / _ \/ _` | / __| __/ _ \ '__|
 *  | || | | | ||  __/ | (_) | |_| | (_) | |_| |  _ <  __/ (_| | \__ \ ||  __/ |
 * |___|_| |_|\__\___|_|\___/ \___/ \___/ \___/|_| \_\___|\__, |_|___/\__\___|_|
 *                                                        |___/
 */

#[derive(Debug, Clone, Copy)]
pub enum Intel8080Register {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    A = 6,

    // Conatins all of the condition bits.
    FLAGS = 7,

    // Stack Pointer (2 bytes)
    SP = 8,

    // Special fake register called 'Program Status Word'. It refers to register pair, A and
    // FLAGS.
    PSW = 10,

    // Special fake register called 'Memory'. Represents the data stored at address contained in
    // HL.
    M = 11,

    Count = 12,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryDescription {
    Instruction,
    Data(u16),
    ASCII(u16),
}

pub trait MemoryAccessor {
    fn read_memory(&self, address: u16) -> u8;
    fn set_memory(&mut self, address: u16, value: u8);

    fn read_memory_u16(&self, address: u16) -> u16 {
        if address == 0xFFFF {
            return self.read_memory(address) as u16;
        }

        return (self.read_memory(address + 1) as u16) << 8 | (self.read_memory(address) as u16);
    }

    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory(address, value as u8);

        if address == 0xFFFF {
            return;
        }

        self.set_memory(address + 1, (value >> 8) as u8);
    }

    fn describe_address(&self, address: u16) -> MemoryDescription;
}

pub struct MemoryStream<'a> {
    memory_accessor: &'a MemoryAccessor,
    index: u16,
}

impl<'a> MemoryStream<'a> {
    pub fn new(memory_accessor: &'a MemoryAccessor, index: u16) -> MemoryStream<'a> {
        return MemoryStream {
            memory_accessor: memory_accessor,
            index: index,
        };
    }
}

impl<'a> io::Read for MemoryStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for i in 0..buf.len() {
            buf[i] = self.memory_accessor.read_memory(self.index + (i as u16));
        }
        self.index += buf.len() as u16;

        Ok(buf.len())
    }
}

pub struct MemoryIterator<'a> {
    memory_accessor: &'a MemoryAccessor,
    current_address: u16,
    ending_address: u16,
}

impl<'a> Iterator for MemoryIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.current_address < self.ending_address {
            let mem = self.memory_accessor.read_memory(self.current_address);
            self.current_address += 1;
            return Some(mem);
        } else {
            return None;
        }
    }
}

impl<'a> MemoryIterator<'a> {
    pub fn new(memory_accessor: &'a MemoryAccessor, range: Range<u16>) -> MemoryIterator {
        assert!(range.start < range.end);
        return MemoryIterator {
            memory_accessor: memory_accessor,
            current_address: range.start,
            ending_address: range.end,
        };
    }
}

pub struct SimpleMemoryAccessor {
    pub memory: [u8; 0x10000],
}

impl SimpleMemoryAccessor {
    pub fn new() -> SimpleMemoryAccessor {
        return SimpleMemoryAccessor {
            memory: [0u8; 0x10000],
        };
    }
}

impl MemoryAccessor for SimpleMemoryAccessor {
    fn read_memory(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        return MemoryDescription::Instruction;
    }
}

/*   ___                      _      ____       _       _
 *  / _ \ _ __   ___ ___   __| | ___|  _ \ _ __(_)_ __ | |_ ___ _ __
 * | | | | '_ \ / __/ _ \ / _` |/ _ \ |_) | '__| | '_ \| __/ _ \ '__|
 * | |_| | |_) | (_| (_) | (_| |  __/  __/| |  | | | | | ||  __/ |
 *  \___/| .__/ \___\___/ \__,_|\___|_|   |_|  |_|_| |_|\__\___|_|
 *       |_|
 */

pub trait InstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8], address: u16) -> Result<()>;
    fn get_instruction<R: io::Read>(&self, stream: R) -> Option<Vec<u8>>;
}

pub trait InstructionPrinterFactory<'a> {
    type Output: InstructionPrinter<'a>;
    fn new(&self, &'a mut io::Write) -> Self::Output;
}

/*  ____  _                                  _     _
 * |  _ \(_)___  __ _ ___ ___  ___ _ __ ___ | |__ | | ___ _ __
 * | | | | / __|/ _` / __/ __|/ _ \ '_ ` _ \| '_ \| |/ _ \ '__|
 * | |_| | \__ \ (_| \__ \__ \  __/ | | | | | |_) | |  __/ |
 * |____/|_|___/\__,_|___/___/\___|_| |_| |_|_.__/|_|\___|_|
 *
 */

pub struct Disassembler<'a, PF: for<'b> InstructionPrinterFactory<'b> + Copy> {
    pub index: u16,
    memory_accessor: &'a MemoryAccessor,
    opcode_printer_factory: PF,
    stream_out: &'a mut io::Write,
}

impl<'a, PF: for<'b> InstructionPrinterFactory<'b> + Copy> Disassembler<'a, PF> {
    pub fn new(
        memory_accessor: &'a MemoryAccessor,
        opcode_printer_factory: PF,
        stream_out: &'a mut io::Write,
    ) -> Disassembler<'a, PF> {
        return Disassembler {
            index: 0,
            memory_accessor: memory_accessor,
            opcode_printer_factory: opcode_printer_factory,
            stream_out: stream_out,
        };
    }

    fn display_data(&mut self, data: &[u8], include_opcodes: bool, mut index: u16) -> Result<()> {
        let iter = &mut data.iter().peekable();
        while iter.peek().is_some() {
            if include_opcodes {
                try!(write!(self.stream_out, "{:07x}          ", index));
            }
            try!(write!(
                self.stream_out,
                "{:04} ${:02X}",
                "db",
                iter.next().unwrap()
            ));
            index += 1;
            for d in iter.take(15) {
                try!(write!(self.stream_out, ",${:02X}", d));
                index += 1;
            }
            if iter.peek().is_some() {
                try!(writeln!(self.stream_out));
            }
        }
        Ok(())
    }

    fn disassemble_data(&mut self, len: u16, include_opcodes: bool) -> Result<()> {
        let start = self.index;
        let mut data = vec![];
        for _ in 0..len {
            data.push(self.memory_accessor.read_memory(self.index));
            self.index += 1;
        }

        self.display_data(&data, include_opcodes, start)
    }

    fn disassemble_ascii(&mut self, len: u16, include_opcodes: bool) -> Result<()> {
        let start = self.index;
        let mut data = vec![];
        for _ in 0..len {
            data.push(self.memory_accessor.read_memory(self.index));
            self.index += 1;
        }

        match str::from_utf8(&data) {
            Ok(s) => {
                if include_opcodes {
                    try!(write!(self.stream_out, "{:07x}          ", start));
                }
                write!(self.stream_out, "{:04} \"{}\"", "db", s)
            }
            Err(_) => self.display_data(&data, include_opcodes, start),
        }
    }

    pub fn disassemble_one(&mut self, include_opcodes: bool) -> Result<()> {
        match self.memory_accessor.describe_address(self.index) {
            MemoryDescription::Instruction => self.disassemble_one_instruction(include_opcodes),
            MemoryDescription::Data(len) => self.disassemble_data(len, include_opcodes),
            MemoryDescription::ASCII(len) => self.disassemble_ascii(len, include_opcodes),
        }
    }

    pub fn disassemble_multiple(&mut self) -> Result<()> {
        let context = 10;
        let mut previous = vec![];
        let current = self.index;

        let start = current.saturating_sub((context + 5) * 3);

        {
            let mut dis = Disassembler::new(
                self.memory_accessor,
                self.opcode_printer_factory,
                &mut previous,
            );
            dis.disassemble(start..current, true).unwrap();
        }

        let lines = str::from_utf8(&previous).unwrap();
        let skip = lines.lines().count().saturating_sub(context as usize);

        for line in lines.lines().skip(skip) {
            try!(writeln!(self.stream_out, "{}", line));
        }

        try!(self.disassemble_one(true));
        try!(writeln!(self.stream_out, " <---"));

        for _ in 0..(context - 1) {
            try!(self.disassemble_one(true));
            try!(writeln!(self.stream_out));
        }
        try!(self.disassemble_one(true));

        self.index = current;

        Ok(())
    }

    fn disassemble_one_instruction(&mut self, include_opcodes: bool) -> Result<()> {
        let mut printed_instr: Vec<u8> = vec![];
        let instr: Vec<u8>;
        let printed;
        {
            let mut opcode_printer = self.opcode_printer_factory.new(&mut printed_instr);
            let stream = MemoryStream::new(self.memory_accessor, self.index);
            printed = match opcode_printer.get_instruction(stream) {
                Some(res) => {
                    try!(opcode_printer.print_instruction(&res, self.index));
                    instr = res;
                    true
                }
                None => {
                    instr = vec![self.memory_accessor.read_memory(self.index)];
                    false
                }
            };
        }

        let str_instr = match printed {
            true => str::from_utf8(&printed_instr).unwrap(),
            false => "-",
        };

        if include_opcodes {
            let mut raw_assembly = String::new();
            for code in &instr {
                raw_assembly.push_str(format!("{:02x} ", code).as_str());
            }

            try!(write!(
                self.stream_out,
                "{:07x} {:9}{}",
                self.index, raw_assembly, str_instr
            ));
        } else {
            try!(write!(self.stream_out, "{}", str_instr));
        }

        self.index += instr.len() as u16;

        Ok(())
    }

    pub fn disassemble(&mut self, range: Range<u16>, include_opcodes: bool) -> Result<()> {
        self.index = range.start;
        while self.index < range.end {
            try!(self.disassemble_one(include_opcodes));
            try!(writeln!(self.stream_out, ""));
        }
        Ok(())
    }
}

#[cfg(test)]
struct TestInstructionPrinter<'a> {
    stream_out: &'a mut io::Write,
}

#[cfg(test)]
impl<'a> InstructionPrinter<'a> for TestInstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8], _address: u16) -> Result<()> {
        match stream[0] {
            0x1 => write!(self.stream_out, "TEST1").unwrap(),
            0x2 => write!(self.stream_out, "TEST2").unwrap(),
            0x3 => write!(self.stream_out, "TEST3").unwrap(),
            _ => panic!("Unknown Opcode {}", stream[0]),
        };
        Ok(())
    }
    fn get_instruction<R: io::Read>(&self, mut stream: R) -> Option<Vec<u8>> {
        let mut instr = vec![0];
        stream.read(&mut instr).unwrap();
        let size = match instr[0] {
            0x1 => 1,
            0x2 => 2,
            0x3 => 3,
            _ => return None,
        };
        instr.resize(size, 0);
        stream.read(&mut instr[1..]).unwrap();
        return Some(instr);
    }
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct TestInstructionPrinterFactory;

#[cfg(test)]
impl<'a> InstructionPrinterFactory<'a> for TestInstructionPrinterFactory {
    type Output = TestInstructionPrinter<'a>;
    fn new(&self, stream_out: &'a mut io::Write) -> TestInstructionPrinter<'a> {
        return TestInstructionPrinter {
            stream_out: stream_out,
        };
    }
}

#[cfg(test)]
pub fn do_disassembler_test<PF: for<'b> InstructionPrinterFactory<'b> + Copy>(
    opcode_printer_factory: PF,
    test_rom: &[u8],
    expected_str: &str,
) {
    let mut output = vec![];
    {
        let mut ma = SimpleMemoryAccessor::new();
        ma.memory[0..test_rom.len()].clone_from_slice(test_rom);
        let mut disassembler = Disassembler::new(&ma, opcode_printer_factory, &mut output);
        disassembler
            .disassemble(0u16..test_rom.len() as u16, true)
            .unwrap();
    }
    assert_eq!(str::from_utf8(&output).unwrap(), expected_str);
}

#[test]
fn disassembler_test_single_byte_instructions() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x1, 0x1, 0x1],
        "\
         0000000 01       TEST1\n\
         0000001 01       TEST1\n\
         0000002 01       TEST1\n\
         ",
    );
}

#[test]
fn disassembler_test_multiple_byte_instructions() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x1, 0x2, 0x0, 0x3, 0x0, 0x0],
        "\
         0000000 01       TEST1\n\
         0000001 02 00    TEST2\n\
         0000003 03 00 00 TEST3\n\
         ",
    );
}

#[test]
fn disassembler_test_instruction_arguments_are_printed() {
    do_disassembler_test(
        TestInstructionPrinterFactory,
        &[0x3, 0xff, 0xfe, 0x3, 0xfd, 0xfc],
        "\
         0000000 03 ff fe TEST3\n\
         0000003 03 fd fc TEST3\n\
         ",
    );
}

/*  ____       _
 * |  _ \  ___| |__  _   _  __ _  __ _  ___ _ __
 * | | | |/ _ \ '_ \| | | |/ _` |/ _` |/ _ \ '__|
 * | |_| |  __/ |_) | |_| | (_| | (_| |  __/ |
 * |____/ \___|_.__/ \__,_|\__, |\__, |\___|_|
 *                         |___/ |___/
 */

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
