pub mod opcodes;

use std::io::Result;
use z8080::opcodes::opcode_gen::{InstructionSet8080, Register8080};

struct _Z8080Emulator;

impl InstructionSet8080 for _Z8080Emulator {
    fn instruction_cpi(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sub(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jz(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cpo(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_aci(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cmc(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cpe(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cma(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ani(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jm(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sbi(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rz(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_lhld(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ei(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_shld(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sim(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jc(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_dad(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jnc(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_lda(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rp(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_daa(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rnz(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jmp(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_di(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rrc(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_pop(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ret(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rim(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rpe(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_dcx(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rc(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_xchg(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rm(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cmp(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_dcr(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rpo(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_out(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cnz(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_xri(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sta(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cm(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_stc(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cc(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jp(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_xra(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_push(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_add(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cnc(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ldax(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_in(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cz(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_mvi(&mut self, _register1: Register8080, _data2: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_cp(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_xthl(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_stax(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_adi(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sui(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_pchl(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_inx(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ana(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jpo(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sphl(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rnc(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jnz(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_hlt(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_jpe(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_mov(&mut self, _register1: Register8080, _register2: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_inr(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rar(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_sbb(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_not_implemented(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_call(&mut self, _address1: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rlc(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ori(&mut self, _data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_rst(&mut self, _implicit_data1: u8) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ora(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
    fn instruction_nop(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_ral(&mut self) -> Result<()>
    {
        Ok(())
    }
    fn instruction_lxi(&mut self, _register1: Register8080, _data2: u16) -> Result<()>
    {
        Ok(())
    }
    fn instruction_adc(&mut self, _register1: Register8080) -> Result<()>
    {
        Ok(())
    }
}

pub fn run_emulator<'a>(_rom: &'a [u8]) {
}
