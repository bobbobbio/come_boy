
use std::io::{self, Result};

/*
 * Warning: This file is generated.  Don't manually edit.
 * Instead edit opcodes/opcode_gen.py
 */

fn read_u16<T: io::Read>(
    mut stream: T) -> Result<u16>
{
    let mut narg : u16;
    let mut arg_buffer = [0; 1];
    try!(stream.read_exact(&mut arg_buffer));
    narg = arg_buffer[0] as u16;
    try!(stream.read_exact(&mut arg_buffer));
    narg |= (arg_buffer[0] as u16)  << 8;
    Ok(narg)
}

fn read_u8<T: io::Read>(
    mut stream: T) -> Result<u8>
{
    let mut arg_buffer = [0; 1];
    try!(stream.read_exact(&mut arg_buffer));
    Ok(arg_buffer[0])
}

#[derive(Debug)]
pub enum Register8080 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    PSW,
    SP,
}

pub trait InstructionSet8080 {
    fn instruction_cpi(&mut self, data1: u8) -> Result<()>;
    fn instruction_sub(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_jz(&mut self, address1: u16) -> Result<()>;
    fn instruction_cpo(&mut self, address1: u16) -> Result<()>;
    fn instruction_aci(&mut self, data1: u8) -> Result<()>;
    fn instruction_cmc(&mut self) -> Result<()>;
    fn instruction_cpe(&mut self, address1: u16) -> Result<()>;
    fn instruction_cma(&mut self) -> Result<()>;
    fn instruction_ani(&mut self, data1: u8) -> Result<()>;
    fn instruction_jm(&mut self, address1: u16) -> Result<()>;
    fn instruction_sbi(&mut self, data1: u8) -> Result<()>;
    fn instruction_rz(&mut self) -> Result<()>;
    fn instruction_lhld(&mut self, address1: u16) -> Result<()>;
    fn instruction_ei(&mut self) -> Result<()>;
    fn instruction_shld(&mut self, address1: u16) -> Result<()>;
    fn instruction_sim(&mut self) -> Result<()>;
    fn instruction_jc(&mut self, address1: u16) -> Result<()>;
    fn instruction_dad(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_jnc(&mut self, address1: u16) -> Result<()>;
    fn instruction_lda(&mut self, address1: u16) -> Result<()>;
    fn instruction_rp(&mut self) -> Result<()>;
    fn instruction_daa(&mut self) -> Result<()>;
    fn instruction_rnz(&mut self) -> Result<()>;
    fn instruction_jmp(&mut self, address1: u16) -> Result<()>;
    fn instruction_di(&mut self) -> Result<()>;
    fn instruction_rrc(&mut self) -> Result<()>;
    fn instruction_pop(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_ret(&mut self) -> Result<()>;
    fn instruction_rim(&mut self) -> Result<()>;
    fn instruction_rpe(&mut self) -> Result<()>;
    fn instruction_dcx(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_rc(&mut self) -> Result<()>;
    fn instruction_xchg(&mut self) -> Result<()>;
    fn instruction_rm(&mut self) -> Result<()>;
    fn instruction_cmp(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_dcr(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_rpo(&mut self) -> Result<()>;
    fn instruction_out(&mut self, data1: u8) -> Result<()>;
    fn instruction_cnz(&mut self, address1: u16) -> Result<()>;
    fn instruction_xri(&mut self, data1: u8) -> Result<()>;
    fn instruction_sta(&mut self, address1: u16) -> Result<()>;
    fn instruction_cm(&mut self, address1: u16) -> Result<()>;
    fn instruction_stc(&mut self) -> Result<()>;
    fn instruction_cc(&mut self, address1: u16) -> Result<()>;
    fn instruction_jp(&mut self, address1: u16) -> Result<()>;
    fn instruction_xra(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_push(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_add(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_cnc(&mut self, address1: u16) -> Result<()>;
    fn instruction_ldax(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_in(&mut self, data1: u8) -> Result<()>;
    fn instruction_cz(&mut self, address1: u16) -> Result<()>;
    fn instruction_mvi(&mut self, register1: Register8080, data2: u8) -> Result<()>;
    fn instruction_cp(&mut self, address1: u16) -> Result<()>;
    fn instruction_xthl(&mut self) -> Result<()>;
    fn instruction_stax(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_adi(&mut self, data1: u8) -> Result<()>;
    fn instruction_sui(&mut self, data1: u8) -> Result<()>;
    fn instruction_pchl(&mut self) -> Result<()>;
    fn instruction_inx(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_ana(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_jpo(&mut self, address1: u16) -> Result<()>;
    fn instruction_sphl(&mut self) -> Result<()>;
    fn instruction_rnc(&mut self) -> Result<()>;
    fn instruction_jnz(&mut self, address1: u16) -> Result<()>;
    fn instruction_hlt(&mut self) -> Result<()>;
    fn instruction_jpe(&mut self, address1: u16) -> Result<()>;
    fn instruction_mov(&mut self, register1: Register8080, register2: Register8080) -> Result<()>;
    fn instruction_inr(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_rar(&mut self) -> Result<()>;
    fn instruction_sbb(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_not_implemented(&mut self) -> Result<()>;
    fn instruction_call(&mut self, address1: u16) -> Result<()>;
    fn instruction_rlc(&mut self) -> Result<()>;
    fn instruction_ori(&mut self, data1: u8) -> Result<()>;
    fn instruction_rst(&mut self, implicit_data1: u8) -> Result<()>;
    fn instruction_ora(&mut self, register1: Register8080) -> Result<()>;
    fn instruction_nop(&mut self) -> Result<()>;
    fn instruction_ral(&mut self) -> Result<()>;
    fn instruction_lxi(&mut self, register1: Register8080, data2: u16) -> Result<()>;
    fn instruction_adc(&mut self, register1: Register8080) -> Result<()>;
}

pub fn dispatch_opcode<I: InstructionSet8080>(
    mut stream: &[u8],
    machine: &mut I) -> Result<(u8)>
{
    let size;
    match try!(read_u8(&mut stream)) {
        0x3e => {
            try!(machine.instruction_mvi(Register8080::A, try!(read_u8(&mut stream)))); size = 2
        }
        0x3d => {
            try!(machine.instruction_dcr(Register8080::A)); size = 1
        }
        0xe4 => {
            try!(machine.instruction_cpo(try!(read_u16(&mut stream)))); size = 3
        }
        0x3f => {
            try!(machine.instruction_cmc()); size = 1
        }
        0x3a => {
            try!(machine.instruction_lda(try!(read_u16(&mut stream)))); size = 3
        }
        0x3c => {
            try!(machine.instruction_inr(Register8080::A)); size = 1
        }
        0x3b => {
            try!(machine.instruction_dcx(Register8080::SP)); size = 1
        }
        0xff => {
            try!(machine.instruction_rst(7 as u8)); size = 1
        }
        0xfa => {
            try!(machine.instruction_jm(try!(read_u16(&mut stream)))); size = 3
        }
        0xda => {
            try!(machine.instruction_jc(try!(read_u16(&mut stream)))); size = 3
        }
        0xec => {
            try!(machine.instruction_cpe(try!(read_u16(&mut stream)))); size = 3
        }
        0x28 => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0x29 => {
            try!(machine.instruction_dad(Register8080::H)); size = 1
        }
        0xcf => {
            try!(machine.instruction_rst(1 as u8)); size = 1
        }
        0xf8 => {
            try!(machine.instruction_rm()); size = 1
        }
        0xeb => {
            try!(machine.instruction_xchg()); size = 1
        }
        0x22 => {
            try!(machine.instruction_shld(try!(read_u16(&mut stream)))); size = 3
        }
        0x23 => {
            try!(machine.instruction_inx(Register8080::H)); size = 1
        }
        0x20 => {
            try!(machine.instruction_rim()); size = 1
        }
        0x21 => {
            try!(machine.instruction_lxi(Register8080::H, try!(read_u16(&mut stream)))); size = 3
        }
        0x26 => {
            try!(machine.instruction_mvi(Register8080::H, try!(read_u8(&mut stream)))); size = 2
        }
        0x27 => {
            try!(machine.instruction_daa()); size = 1
        }
        0x24 => {
            try!(machine.instruction_inr(Register8080::H)); size = 1
        }
        0x25 => {
            try!(machine.instruction_dcr(Register8080::H)); size = 1
        }
        0xdb => {
            try!(machine.instruction_in(try!(read_u8(&mut stream)))); size = 2
        }
        0xef => {
            try!(machine.instruction_rst(5 as u8)); size = 1
        }
        0xe2 => {
            try!(machine.instruction_jpo(try!(read_u16(&mut stream)))); size = 3
        }
        0xee => {
            try!(machine.instruction_xri(try!(read_u8(&mut stream)))); size = 2
        }
        0xed => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0xdc => {
            try!(machine.instruction_cc(try!(read_u16(&mut stream)))); size = 3
        }
        0x35 => {
            try!(machine.instruction_dcr(Register8080::M)); size = 1
        }
        0x34 => {
            try!(machine.instruction_inr(Register8080::M)); size = 1
        }
        0x37 => {
            try!(machine.instruction_stc()); size = 1
        }
        0x36 => {
            try!(machine.instruction_mvi(Register8080::M, try!(read_u8(&mut stream)))); size = 2
        }
        0x31 => {
            try!(machine.instruction_lxi(Register8080::SP, try!(read_u16(&mut stream)))); size = 3
        }
        0x30 => {
            try!(machine.instruction_sim()); size = 1
        }
        0x33 => {
            try!(machine.instruction_inx(Register8080::SP)); size = 1
        }
        0x32 => {
            try!(machine.instruction_sta(try!(read_u16(&mut stream)))); size = 3
        }
        0xd4 => {
            try!(machine.instruction_cnc(try!(read_u16(&mut stream)))); size = 3
        }
        0xe8 => {
            try!(machine.instruction_rpe()); size = 1
        }
        0x39 => {
            try!(machine.instruction_dad(Register8080::SP)); size = 1
        }
        0x38 => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0xc0 => {
            try!(machine.instruction_rnz()); size = 1
        }
        0xe1 => {
            try!(machine.instruction_pop(Register8080::H)); size = 1
        }
        0xfe => {
            try!(machine.instruction_cpi(try!(read_u8(&mut stream)))); size = 2
        }
        0x88 => {
            try!(machine.instruction_adc(Register8080::B)); size = 1
        }
        0xdd => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0x89 => {
            try!(machine.instruction_adc(Register8080::C)); size = 1
        }
        0x2b => {
            try!(machine.instruction_dcx(Register8080::H)); size = 1
        }
        0x2c => {
            try!(machine.instruction_inr(Register8080::L)); size = 1
        }
        0xfd => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0x2a => {
            try!(machine.instruction_lhld(try!(read_u16(&mut stream)))); size = 3
        }
        0x2f => {
            try!(machine.instruction_cma()); size = 1
        }
        0xfc => {
            try!(machine.instruction_cm(try!(read_u16(&mut stream)))); size = 3
        }
        0x2d => {
            try!(machine.instruction_dcr(Register8080::L)); size = 1
        }
        0x2e => {
            try!(machine.instruction_mvi(Register8080::L, try!(read_u8(&mut stream)))); size = 2
        }
        0x5c => {
            try!(machine.instruction_mov(Register8080::E, Register8080::H)); size = 1
        }
        0x5b => {
            try!(machine.instruction_mov(Register8080::E, Register8080::E)); size = 1
        }
        0x5a => {
            try!(machine.instruction_mov(Register8080::E, Register8080::D)); size = 1
        }
        0xba => {
            try!(machine.instruction_cmp(Register8080::D)); size = 1
        }
        0x5f => {
            try!(machine.instruction_mov(Register8080::E, Register8080::A)); size = 1
        }
        0x5e => {
            try!(machine.instruction_mov(Register8080::E, Register8080::M)); size = 1
        }
        0x5d => {
            try!(machine.instruction_mov(Register8080::E, Register8080::L)); size = 1
        }
        0xc9 => {
            try!(machine.instruction_ret()); size = 1
        }
        0x40 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::B)); size = 1
        }
        0x41 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::C)); size = 1
        }
        0x42 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::D)); size = 1
        }
        0x43 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::E)); size = 1
        }
        0x44 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::H)); size = 1
        }
        0x45 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::L)); size = 1
        }
        0x46 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::M)); size = 1
        }
        0x47 => {
            try!(machine.instruction_mov(Register8080::B, Register8080::A)); size = 1
        }
        0x48 => {
            try!(machine.instruction_mov(Register8080::C, Register8080::B)); size = 1
        }
        0x49 => {
            try!(machine.instruction_mov(Register8080::C, Register8080::C)); size = 1
        }
        0xaf => {
            try!(machine.instruction_xra(Register8080::A)); size = 1
        }
        0xae => {
            try!(machine.instruction_xra(Register8080::M)); size = 1
        }
        0xad => {
            try!(machine.instruction_xra(Register8080::L)); size = 1
        }
        0xac => {
            try!(machine.instruction_xra(Register8080::H)); size = 1
        }
        0xab => {
            try!(machine.instruction_xra(Register8080::E)); size = 1
        }
        0xaa => {
            try!(machine.instruction_xra(Register8080::D)); size = 1
        }
        0xe6 => {
            try!(machine.instruction_ani(try!(read_u8(&mut stream)))); size = 2
        }
        0xea => {
            try!(machine.instruction_jpe(try!(read_u16(&mut stream)))); size = 3
        }
        0x4a => {
            try!(machine.instruction_mov(Register8080::C, Register8080::D)); size = 1
        }
        0x4b => {
            try!(machine.instruction_mov(Register8080::C, Register8080::E)); size = 1
        }
        0x4c => {
            try!(machine.instruction_mov(Register8080::C, Register8080::H)); size = 1
        }
        0x4d => {
            try!(machine.instruction_mov(Register8080::C, Register8080::L)); size = 1
        }
        0x4e => {
            try!(machine.instruction_mov(Register8080::C, Register8080::M)); size = 1
        }
        0x4f => {
            try!(machine.instruction_mov(Register8080::C, Register8080::A)); size = 1
        }
        0x53 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::E)); size = 1
        }
        0x52 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::D)); size = 1
        }
        0x51 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::C)); size = 1
        }
        0x50 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::B)); size = 1
        }
        0x57 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::A)); size = 1
        }
        0x56 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::M)); size = 1
        }
        0x55 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::L)); size = 1
        }
        0x54 => {
            try!(machine.instruction_mov(Register8080::D, Register8080::H)); size = 1
        }
        0xe5 => {
            try!(machine.instruction_push(Register8080::H)); size = 1
        }
        0x59 => {
            try!(machine.instruction_mov(Register8080::E, Register8080::C)); size = 1
        }
        0x58 => {
            try!(machine.instruction_mov(Register8080::E, Register8080::B)); size = 1
        }
        0xf4 => {
            try!(machine.instruction_cp(try!(read_u16(&mut stream)))); size = 3
        }
        0xfb => {
            try!(machine.instruction_ei()); size = 1
        }
        0xf9 => {
            try!(machine.instruction_sphl()); size = 1
        }
        0xf6 => {
            try!(machine.instruction_ori(try!(read_u8(&mut stream)))); size = 2
        }
        0xa9 => {
            try!(machine.instruction_xra(Register8080::C)); size = 1
        }
        0xa8 => {
            try!(machine.instruction_xra(Register8080::B)); size = 1
        }
        0xa7 => {
            try!(machine.instruction_ana(Register8080::A)); size = 1
        }
        0xa6 => {
            try!(machine.instruction_ana(Register8080::M)); size = 1
        }
        0xa5 => {
            try!(machine.instruction_ana(Register8080::L)); size = 1
        }
        0xa4 => {
            try!(machine.instruction_ana(Register8080::H)); size = 1
        }
        0xa3 => {
            try!(machine.instruction_ana(Register8080::E)); size = 1
        }
        0xa2 => {
            try!(machine.instruction_ana(Register8080::D)); size = 1
        }
        0xa1 => {
            try!(machine.instruction_ana(Register8080::C)); size = 1
        }
        0xa0 => {
            try!(machine.instruction_ana(Register8080::B)); size = 1
        }
        0xf5 => {
            try!(machine.instruction_push(Register8080::PSW)); size = 1
        }
        0x7a => {
            try!(machine.instruction_mov(Register8080::A, Register8080::D)); size = 1
        }
        0xf2 => {
            try!(machine.instruction_jp(try!(read_u16(&mut stream)))); size = 3
        }
        0x7c => {
            try!(machine.instruction_mov(Register8080::A, Register8080::H)); size = 1
        }
        0x7b => {
            try!(machine.instruction_mov(Register8080::A, Register8080::E)); size = 1
        }
        0x7e => {
            try!(machine.instruction_mov(Register8080::A, Register8080::M)); size = 1
        }
        0x7d => {
            try!(machine.instruction_mov(Register8080::A, Register8080::L)); size = 1
        }
        0x7f => {
            try!(machine.instruction_mov(Register8080::A, Register8080::A)); size = 1
        }
        0xf0 => {
            try!(machine.instruction_rp()); size = 1
        }
        0x68 => {
            try!(machine.instruction_mov(Register8080::L, Register8080::B)); size = 1
        }
        0x69 => {
            try!(machine.instruction_mov(Register8080::L, Register8080::C)); size = 1
        }
        0x66 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::M)); size = 1
        }
        0x67 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::A)); size = 1
        }
        0x64 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::H)); size = 1
        }
        0x65 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::L)); size = 1
        }
        0x62 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::D)); size = 1
        }
        0x63 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::E)); size = 1
        }
        0x60 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::B)); size = 1
        }
        0x61 => {
            try!(machine.instruction_mov(Register8080::H, Register8080::C)); size = 1
        }
        0x99 => {
            try!(machine.instruction_sbb(Register8080::C)); size = 1
        }
        0xd5 => {
            try!(machine.instruction_push(Register8080::D)); size = 1
        }
        0xce => {
            try!(machine.instruction_aci(try!(read_u8(&mut stream)))); size = 2
        }
        0xcd => {
            try!(machine.instruction_call(try!(read_u16(&mut stream)))); size = 3
        }
        0xb8 => {
            try!(machine.instruction_cmp(Register8080::B)); size = 1
        }
        0xb9 => {
            try!(machine.instruction_cmp(Register8080::C)); size = 1
        }
        0xca => {
            try!(machine.instruction_jz(try!(read_u16(&mut stream)))); size = 3
        }
        0xcc => {
            try!(machine.instruction_cz(try!(read_u16(&mut stream)))); size = 3
        }
        0xcb => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0xb2 => {
            try!(machine.instruction_ora(Register8080::D)); size = 1
        }
        0xb3 => {
            try!(machine.instruction_ora(Register8080::E)); size = 1
        }
        0xb0 => {
            try!(machine.instruction_ora(Register8080::B)); size = 1
        }
        0xb1 => {
            try!(machine.instruction_ora(Register8080::C)); size = 1
        }
        0xb6 => {
            try!(machine.instruction_ora(Register8080::M)); size = 1
        }
        0xb7 => {
            try!(machine.instruction_ora(Register8080::A)); size = 1
        }
        0xb4 => {
            try!(machine.instruction_ora(Register8080::H)); size = 1
        }
        0xb5 => {
            try!(machine.instruction_ora(Register8080::L)); size = 1
        }
        0xe3 => {
            try!(machine.instruction_xthl()); size = 1
        }
        0xd6 => {
            try!(machine.instruction_sui(try!(read_u8(&mut stream)))); size = 2
        }
        0x6f => {
            try!(machine.instruction_mov(Register8080::L, Register8080::A)); size = 1
        }
        0x6d => {
            try!(machine.instruction_mov(Register8080::L, Register8080::L)); size = 1
        }
        0x6e => {
            try!(machine.instruction_mov(Register8080::L, Register8080::M)); size = 1
        }
        0x6b => {
            try!(machine.instruction_mov(Register8080::L, Register8080::E)); size = 1
        }
        0x6c => {
            try!(machine.instruction_mov(Register8080::L, Register8080::H)); size = 1
        }
        0x6a => {
            try!(machine.instruction_mov(Register8080::L, Register8080::D)); size = 1
        }
        0x79 => {
            try!(machine.instruction_mov(Register8080::A, Register8080::C)); size = 1
        }
        0x78 => {
            try!(machine.instruction_mov(Register8080::A, Register8080::B)); size = 1
        }
        0x71 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::C)); size = 1
        }
        0x70 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::B)); size = 1
        }
        0x73 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::E)); size = 1
        }
        0x72 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::D)); size = 1
        }
        0x75 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::L)); size = 1
        }
        0x74 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::H)); size = 1
        }
        0x77 => {
            try!(machine.instruction_mov(Register8080::M, Register8080::A)); size = 1
        }
        0x76 => {
            try!(machine.instruction_hlt()); size = 1
        }
        0xc5 => {
            try!(machine.instruction_push(Register8080::B)); size = 1
        }
        0xc4 => {
            try!(machine.instruction_cnz(try!(read_u16(&mut stream)))); size = 3
        }
        0xc7 => {
            try!(machine.instruction_rst(0 as u8)); size = 1
        }
        0xc6 => {
            try!(machine.instruction_adi(try!(read_u8(&mut stream)))); size = 2
        }
        0xc1 => {
            try!(machine.instruction_pop(Register8080::B)); size = 1
        }
        0x8b => {
            try!(machine.instruction_adc(Register8080::E)); size = 1
        }
        0xc3 => {
            try!(machine.instruction_jmp(try!(read_u16(&mut stream)))); size = 3
        }
        0xc2 => {
            try!(machine.instruction_jnz(try!(read_u16(&mut stream)))); size = 3
        }
        0xbb => {
            try!(machine.instruction_cmp(Register8080::E)); size = 1
        }
        0xbc => {
            try!(machine.instruction_cmp(Register8080::H)); size = 1
        }
        0x8c => {
            try!(machine.instruction_adc(Register8080::H)); size = 1
        }
        0xbf => {
            try!(machine.instruction_cmp(Register8080::A)); size = 1
        }
        0xc8 => {
            try!(machine.instruction_rz()); size = 1
        }
        0xbd => {
            try!(machine.instruction_cmp(Register8080::L)); size = 1
        }
        0xbe => {
            try!(machine.instruction_cmp(Register8080::M)); size = 1
        }
        0xf1 => {
            try!(machine.instruction_pop(Register8080::PSW)); size = 1
        }
        0xe9 => {
            try!(machine.instruction_pchl()); size = 1
        }
        0xd8 => {
            try!(machine.instruction_rc()); size = 1
        }
        0xd9 => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0xf7 => {
            try!(machine.instruction_rst(6 as u8)); size = 1
        }
        0xf3 => {
            try!(machine.instruction_di()); size = 1
        }
        0xd0 => {
            try!(machine.instruction_rnc()); size = 1
        }
        0x9f => {
            try!(machine.instruction_sbb(Register8080::A)); size = 1
        }
        0x9e => {
            try!(machine.instruction_sbb(Register8080::M)); size = 1
        }
        0x9d => {
            try!(machine.instruction_sbb(Register8080::L)); size = 1
        }
        0x08 => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0x09 => {
            try!(machine.instruction_dad(Register8080::B)); size = 1
        }
        0x9a => {
            try!(machine.instruction_sbb(Register8080::D)); size = 1
        }
        0xd7 => {
            try!(machine.instruction_rst(2 as u8)); size = 1
        }
        0x04 => {
            try!(machine.instruction_inr(Register8080::B)); size = 1
        }
        0x05 => {
            try!(machine.instruction_dcr(Register8080::B)); size = 1
        }
        0x06 => {
            try!(machine.instruction_mvi(Register8080::B, try!(read_u8(&mut stream)))); size = 2
        }
        0x07 => {
            try!(machine.instruction_rlc()); size = 1
        }
        0x00 => {
            try!(machine.instruction_nop()); size = 1
        }
        0x01 => {
            try!(machine.instruction_lxi(Register8080::B, try!(read_u16(&mut stream)))); size = 3
        }
        0x02 => {
            try!(machine.instruction_stax(Register8080::B)); size = 1
        }
        0x03 => {
            try!(machine.instruction_inx(Register8080::B)); size = 1
        }
        0x84 => {
            try!(machine.instruction_add(Register8080::H)); size = 1
        }
        0x85 => {
            try!(machine.instruction_add(Register8080::L)); size = 1
        }
        0x86 => {
            try!(machine.instruction_add(Register8080::M)); size = 1
        }
        0x87 => {
            try!(machine.instruction_add(Register8080::A)); size = 1
        }
        0x80 => {
            try!(machine.instruction_add(Register8080::B)); size = 1
        }
        0x81 => {
            try!(machine.instruction_add(Register8080::C)); size = 1
        }
        0x82 => {
            try!(machine.instruction_add(Register8080::D)); size = 1
        }
        0x83 => {
            try!(machine.instruction_add(Register8080::E)); size = 1
        }
        0x1f => {
            try!(machine.instruction_rar()); size = 1
        }
        0x1e => {
            try!(machine.instruction_mvi(Register8080::E, try!(read_u8(&mut stream)))); size = 2
        }
        0x1d => {
            try!(machine.instruction_dcr(Register8080::E)); size = 1
        }
        0x1c => {
            try!(machine.instruction_inr(Register8080::E)); size = 1
        }
        0x1b => {
            try!(machine.instruction_dcx(Register8080::D)); size = 1
        }
        0x1a => {
            try!(machine.instruction_ldax(Register8080::D)); size = 1
        }
        0xde => {
            try!(machine.instruction_sbi(try!(read_u8(&mut stream)))); size = 2
        }
        0xdf => {
            try!(machine.instruction_rst(3 as u8)); size = 1
        }
        0xd1 => {
            try!(machine.instruction_pop(Register8080::D)); size = 1
        }
        0xd2 => {
            try!(machine.instruction_jnc(try!(read_u16(&mut stream)))); size = 3
        }
        0xd3 => {
            try!(machine.instruction_out(try!(read_u8(&mut stream)))); size = 2
        }
        0x9c => {
            try!(machine.instruction_sbb(Register8080::H)); size = 1
        }
        0x9b => {
            try!(machine.instruction_sbb(Register8080::E)); size = 1
        }
        0x8d => {
            try!(machine.instruction_adc(Register8080::L)); size = 1
        }
        0x8e => {
            try!(machine.instruction_adc(Register8080::M)); size = 1
        }
        0x8f => {
            try!(machine.instruction_adc(Register8080::A)); size = 1
        }
        0xe0 => {
            try!(machine.instruction_rpo()); size = 1
        }
        0xe7 => {
            try!(machine.instruction_rst(4 as u8)); size = 1
        }
        0x8a => {
            try!(machine.instruction_adc(Register8080::D)); size = 1
        }
        0x19 => {
            try!(machine.instruction_dad(Register8080::D)); size = 1
        }
        0x18 => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0x17 => {
            try!(machine.instruction_ral()); size = 1
        }
        0x16 => {
            try!(machine.instruction_mvi(Register8080::D, try!(read_u8(&mut stream)))); size = 2
        }
        0x15 => {
            try!(machine.instruction_dcr(Register8080::D)); size = 1
        }
        0x14 => {
            try!(machine.instruction_inr(Register8080::D)); size = 1
        }
        0x13 => {
            try!(machine.instruction_inx(Register8080::D)); size = 1
        }
        0x12 => {
            try!(machine.instruction_stax(Register8080::D)); size = 1
        }
        0x11 => {
            try!(machine.instruction_lxi(Register8080::D, try!(read_u16(&mut stream)))); size = 3
        }
        0x10 => {
            try!(machine.instruction_not_implemented()); size = 1
        }
        0x97 => {
            try!(machine.instruction_sub(Register8080::A)); size = 1
        }
        0x96 => {
            try!(machine.instruction_sub(Register8080::M)); size = 1
        }
        0x95 => {
            try!(machine.instruction_sub(Register8080::L)); size = 1
        }
        0x94 => {
            try!(machine.instruction_sub(Register8080::H)); size = 1
        }
        0x93 => {
            try!(machine.instruction_sub(Register8080::E)); size = 1
        }
        0x92 => {
            try!(machine.instruction_sub(Register8080::D)); size = 1
        }
        0x91 => {
            try!(machine.instruction_sub(Register8080::C)); size = 1
        }
        0x90 => {
            try!(machine.instruction_sub(Register8080::B)); size = 1
        }
        0x0d => {
            try!(machine.instruction_dcr(Register8080::C)); size = 1
        }
        0x0e => {
            try!(machine.instruction_mvi(Register8080::C, try!(read_u8(&mut stream)))); size = 2
        }
        0x0f => {
            try!(machine.instruction_rrc()); size = 1
        }
        0x98 => {
            try!(machine.instruction_sbb(Register8080::B)); size = 1
        }
        0x0a => {
            try!(machine.instruction_ldax(Register8080::B)); size = 1
        }
        0x0b => {
            try!(machine.instruction_dcx(Register8080::B)); size = 1
        }
        0x0c => {
            try!(machine.instruction_inr(Register8080::C)); size = 1
        }

        _ => panic!("Unknown opcode"),
   };
   Ok((size))
}

pub struct OpcodePrinter<'a> {
    stream_out: &'a mut io::Write
}
impl<'a> OpcodePrinter<'a> {
    pub fn new(stream_out: &'a mut io::Write) -> OpcodePrinter<'a>
    {
        return OpcodePrinter {
            stream_out: stream_out
        };
    }
}
impl<'a> InstructionSet8080 for OpcodePrinter<'a> {
    fn instruction_cpi(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CPI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_sub(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SUB"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_jz(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_cpo(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CPO"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_aci(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ACI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_cmc(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CMC"));
        Ok(())
    }
    fn instruction_cpe(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CPE"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_cma(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CMA"));
        Ok(())
    }
    fn instruction_ani(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ANI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_jm(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JM"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_sbi(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SBI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_rz(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RZ"));
        Ok(())
    }
    fn instruction_lhld(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LHLD"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_ei(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "EI"));
        Ok(())
    }
    fn instruction_shld(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SHLD"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_sim(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SIM"));
        Ok(())
    }
    fn instruction_jc(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_dad(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DAD"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_jnc(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JNC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_lda(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LDA"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_rp(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RP"));
        Ok(())
    }
    fn instruction_daa(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DAA"));
        Ok(())
    }
    fn instruction_rnz(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RNZ"));
        Ok(())
    }
    fn instruction_jmp(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JMP"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_di(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DI"));
        Ok(())
    }
    fn instruction_rrc(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RRC"));
        Ok(())
    }
    fn instruction_pop(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "POP"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_ret(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RET"));
        Ok(())
    }
    fn instruction_rim(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RIM"));
        Ok(())
    }
    fn instruction_rpe(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RPE"));
        Ok(())
    }
    fn instruction_dcx(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DCX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_rc(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RC"));
        Ok(())
    }
    fn instruction_xchg(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XCHG"));
        Ok(())
    }
    fn instruction_rm(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RM"));
        Ok(())
    }
    fn instruction_cmp(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CMP"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_dcr(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "DCR"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_rpo(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RPO"));
        Ok(())
    }
    fn instruction_out(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "OUT"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_cnz(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CNZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_xri(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XRI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_sta(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "STA"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_cm(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CM"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_stc(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "STC"));
        Ok(())
    }
    fn instruction_cc(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_jp(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JP"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_xra(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XRA"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_push(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "PUSH"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_add(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ADD"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_cnc(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CNC"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_ldax(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LDAX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_in(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "IN"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_cz(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_mvi(&mut self, register1: Register8080, data2: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "MVI"));
        try!(write!(self.stream_out, " {:?}", register1));
        try!(write!(self.stream_out, " #${:02x}", data2));
        Ok(())
    }
    fn instruction_cp(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CP"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_xthl(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "XTHL"));
        Ok(())
    }
    fn instruction_stax(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "STAX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_adi(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ADI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_sui(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SUI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_pchl(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "PCHL"));
        Ok(())
    }
    fn instruction_inx(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "INX"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_ana(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ANA"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_jpo(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JPO"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_sphl(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SPHL"));
        Ok(())
    }
    fn instruction_rnc(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RNC"));
        Ok(())
    }
    fn instruction_jnz(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JNZ"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_hlt(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "HLT"));
        Ok(())
    }
    fn instruction_jpe(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "JPE"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_mov(&mut self, register1: Register8080, register2: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "MOV"));
        try!(write!(self.stream_out, " {:?}", register1));
        try!(write!(self.stream_out, " {:?}", register2));
        Ok(())
    }
    fn instruction_inr(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "INR"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_rar(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RAR"));
        Ok(())
    }
    fn instruction_sbb(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "SBB"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_not_implemented(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "-"));
        Ok(())
    }
    fn instruction_call(&mut self, address1: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "CALL"));
        try!(write!(self.stream_out, " ${:02x}", address1));
        Ok(())
    }
    fn instruction_rlc(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RLC"));
        Ok(())
    }
    fn instruction_ori(&mut self, data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ORI"));
        try!(write!(self.stream_out, " #${:02x}", data1));
        Ok(())
    }
    fn instruction_rst(&mut self, implicit_data1: u8) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RST"));
        try!(write!(self.stream_out, " {}", implicit_data1));
        Ok(())
    }
    fn instruction_ora(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ORA"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
    fn instruction_nop(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "NOP"));
        Ok(())
    }
    fn instruction_ral(&mut self) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "RAL"));
        Ok(())
    }
    fn instruction_lxi(&mut self, register1: Register8080, data2: u16) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "LXI"));
        try!(write!(self.stream_out, " {:?}", register1));
        try!(write!(self.stream_out, " #${:02x}", data2));
        Ok(())
    }
    fn instruction_adc(&mut self, register1: Register8080) -> Result<()>
    {
        try!(write!(self.stream_out, "{:04}", "ADC"));
        try!(write!(self.stream_out, " {:?}", register1));
        Ok(())
    }
}