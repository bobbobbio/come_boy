use std::io::{self, Error, ErrorKind, Result};
use std::fmt::{Write};
use std::str;

mod opcode_gen;

fn read_bytes<T: io::Read>(mut stream: T, bytes: u8, mut asm: &mut String) -> Result<u16>
{
    let mut narg : u16 = 0;
    for i in 0..bytes {
        let mut arg_buffer = [0; 1];
        let n = try!(stream.read(&mut arg_buffer));
        if n == 0 {
            return Err(Error::new(ErrorKind::Other, "Unexpected EOF"))
        }
        narg |= (arg_buffer[0] as u16)  << (i * 8);
        write!(&mut asm, " {:02x}", arg_buffer[0]).ok();
    }
    Ok(narg)
}

fn print_opcode_with_table<T: io::Read, U: io::Write>(
    mut stream_in: T,
    index: &mut u64,
    mut stream_out: U,
    lookup_opcode: fn(u8) -> (&'static str, u8, Vec<&'static str>)) -> Result<()>
{
    let mut buffer = [0; 1];
    let n = try!(stream_in.read(&mut buffer));
    if n == 0 {
        return Err(Error::new(ErrorKind::Other, "Unexpected EOF"))
    }

    let (instr, size, args) = lookup_opcode(buffer[0]);

    let mut formatted_op = String::new();
    write!(&mut formatted_op, "{:4}", instr).ok();

    let mut asm = String::new();
    write!(&mut asm, "{:02x}", buffer[0]).ok();

    let mut byte_args = 0;
    for arg in args {
        if arg.starts_with("D") && arg != "D" {
            let bytes = arg[1..].parse::<u8>().ok().expect("parse error") / 8;
            byte_args += bytes;
            let narg = try!(read_bytes(&mut stream_in, bytes, &mut asm));
            write!(&mut formatted_op, " #${:02x}", narg).ok();
        } else if arg == "adr" {
            byte_args += 2;
            let narg = try!(read_bytes(&mut stream_in, 2, &mut asm));
            write!(&mut formatted_op, " ${:02x}", narg).ok();
        } else {
            write!(&mut formatted_op, " {}", arg).ok();
        }
    }
    assert_eq!(byte_args, size - 1);

    let _ = writeln!(stream_out, "{:07} {:8} {}", *index, asm, formatted_op);
    *index += size as u64;
    Ok(())
}

#[cfg(test)]
fn test_opcode_lookup(opcode: u8) -> (&'static str, u8, Vec<&'static str>) {
    match opcode {
        1 => ("TEST_INSTR1", 3, vec!["adr"]),
        _ => ("Unknown", 1, vec![])
    }
}

#[test]
fn print_opcode_test() {
    let mut index: u64 = 0;
    let code: &[u8] = &[1, 8, 1];
    let mut output: Vec<u8> = vec![];
    print_opcode_with_table(code, &mut index, &mut output, test_opcode_lookup).ok().expect("");
    assert_eq!(str::from_utf8(output.as_slice()).unwrap(), "0000000 01 08 01 TEST_INSTR1 $108\n");
}

pub fn print_opcode<T: io::Read>(
    stream_in: T,
    index: &mut u64) -> Result<()>
{
    print_opcode_with_table(stream_in, index, io::stdout(), opcode_gen::lookup_opcode)
}
