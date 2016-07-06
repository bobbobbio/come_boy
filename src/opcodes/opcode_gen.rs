
pub fn lookup_opcode(
    opcode: u8) -> (&'static str, u8, Vec<&'static str>)
{
    match opcode {
        62 => ("MVI", 2, vec!["A", "D8"]),
        61 => ("DCR", 1, vec!["A"]),
        228 => ("CPO", 3, vec!["adr"]),
        63 => ("CMC", 1, vec![]),
        58 => ("LDA", 3, vec!["adr"]),
        60 => ("INR", 1, vec!["A"]),
        59 => ("DCX", 1, vec!["SP"]),
        255 => ("RST", 1, vec!["7"]),
        250 => ("JM", 3, vec!["adr"]),
        218 => ("JC", 3, vec!["adr"]),
        236 => ("CPE", 3, vec!["adr"]),
        40 => ("-", 1, vec![]),
        41 => ("DAD", 1, vec!["H"]),
        207 => ("RST", 1, vec!["1"]),
        248 => ("RM", 1, vec![]),
        235 => ("XCHG", 1, vec![]),
        34 => ("SHLD", 3, vec!["adr"]),
        35 => ("INX", 1, vec!["H"]),
        32 => ("RIM", 1, vec![]),
        33 => ("LXI", 3, vec!["H", "D16"]),
        38 => ("MVI", 2, vec!["H", "D8"]),
        39 => ("DAA", 1, vec![]),
        36 => ("INR", 1, vec!["H"]),
        37 => ("DCR", 1, vec!["H"]),
        219 => ("IN", 2, vec!["D8"]),
        239 => ("RST", 1, vec!["5"]),
        226 => ("JPO", 3, vec!["adr"]),
        238 => ("XRI", 2, vec!["D8"]),
        237 => ("-", 1, vec![]),
        220 => ("CC", 3, vec!["adr"]),
        53 => ("DCR", 1, vec!["M"]),
        52 => ("INR", 1, vec!["M"]),
        55 => ("STC", 1, vec![]),
        54 => ("MVI", 2, vec!["M", "D8"]),
        49 => ("LXI", 3, vec!["SP", "D16"]),
        48 => ("SIM", 1, vec![]),
        51 => ("INX", 1, vec!["SP"]),
        50 => ("STA", 3, vec!["adr"]),
        212 => ("CNC", 3, vec!["adr"]),
        232 => ("RPE", 1, vec![]),
        57 => ("DAD", 1, vec!["SP"]),
        56 => ("-", 1, vec![]),
        192 => ("RNZ", 1, vec![]),
        225 => ("POP", 1, vec!["H"]),
        254 => ("CPI", 2, vec!["D8"]),
        136 => ("ADC", 1, vec!["B"]),
        221 => ("-", 1, vec![]),
        137 => ("ADC", 1, vec!["C"]),
        43 => ("DCX", 1, vec!["H"]),
        44 => ("INR", 1, vec!["L"]),
        253 => ("-", 1, vec![]),
        42 => ("LHLD", 3, vec!["adr"]),
        47 => ("CMA", 1, vec![]),
        252 => ("CM", 3, vec!["adr"]),
        45 => ("DCR", 1, vec!["L"]),
        46 => ("MVI", 2, vec!["L", "D8"]),
        92 => ("MOV", 1, vec!["E", "H"]),
        91 => ("MOV", 1, vec!["E", "E"]),
        90 => ("MOV", 1, vec!["E", "D"]),
        186 => ("CMP", 1, vec!["D"]),
        95 => ("MOV", 1, vec!["E", "A"]),
        94 => ("MOV", 1, vec!["E", "M"]),
        93 => ("MOV", 1, vec!["E", "L"]),
        201 => ("RET", 1, vec![]),
        64 => ("MOV", 1, vec!["B", "B"]),
        65 => ("MOV", 1, vec!["B", "C"]),
        66 => ("MOV", 1, vec!["B", "D"]),
        67 => ("MOV", 1, vec!["B", "E"]),
        68 => ("MOV", 1, vec!["B", "H"]),
        69 => ("MOV", 1, vec!["B", "L"]),
        70 => ("MOV", 1, vec!["B", "M"]),
        71 => ("MOV", 1, vec!["B", "A"]),
        72 => ("MOV", 1, vec!["C", "B"]),
        73 => ("MOV", 1, vec!["C", "C"]),
        175 => ("XRA", 1, vec!["A"]),
        174 => ("XRA", 1, vec!["M"]),
        173 => ("XRA", 1, vec!["L"]),
        172 => ("XRA", 1, vec!["H"]),
        171 => ("XRA", 1, vec!["E"]),
        170 => ("XRA", 1, vec!["D"]),
        230 => ("ANI", 2, vec!["D8"]),
        234 => ("JPE", 3, vec!["adr"]),
        74 => ("MOV", 1, vec!["C", "D"]),
        75 => ("MOV", 1, vec!["C", "E"]),
        76 => ("MOV", 1, vec!["C", "H"]),
        77 => ("MOV", 1, vec!["C", "L"]),
        78 => ("MOV", 1, vec!["C", "M"]),
        79 => ("MOV", 1, vec!["C", "A"]),
        83 => ("MOV", 1, vec!["D", "E"]),
        82 => ("MOV", 1, vec!["D", "D"]),
        81 => ("MOV", 1, vec!["D", "C"]),
        80 => ("MOV", 1, vec!["D", "B"]),
        87 => ("MOV", 1, vec!["D", "A"]),
        86 => ("MOV", 1, vec!["D", "M"]),
        85 => ("MOV", 1, vec!["D", "L"]),
        84 => ("MOV", 1, vec!["D", "H"]),
        229 => ("PUSH", 1, vec!["H"]),
        89 => ("MOV", 1, vec!["E", "C"]),
        88 => ("MOV", 1, vec!["E", "B"]),
        244 => ("CP", 3, vec!["adr"]),
        251 => ("EI", 1, vec![]),
        249 => ("SPHL", 1, vec![]),
        246 => ("ORI", 2, vec!["D8"]),
        169 => ("XRA", 1, vec!["C"]),
        168 => ("XRA", 1, vec!["B"]),
        167 => ("ANA", 1, vec!["A"]),
        166 => ("ANA", 1, vec!["M"]),
        165 => ("ANA", 1, vec!["L"]),
        164 => ("ANA", 1, vec!["H"]),
        163 => ("ANA", 1, vec!["E"]),
        162 => ("ANA", 1, vec!["D"]),
        161 => ("ANA", 1, vec!["C"]),
        160 => ("ANA", 1, vec!["B"]),
        245 => ("PUSH", 1, vec!["PSW"]),
        122 => ("MOV", 1, vec!["A", "D"]),
        242 => ("JP", 3, vec!["adr"]),
        124 => ("MOV", 1, vec!["A", "H"]),
        123 => ("MOV", 1, vec!["A", "E"]),
        126 => ("MOV", 1, vec!["A", "M"]),
        125 => ("MOV", 1, vec!["A", "L"]),
        127 => ("MOV", 1, vec!["A", "A"]),
        240 => ("RP", 1, vec![]),
        104 => ("MOV", 1, vec!["L", "B"]),
        105 => ("MOV", 1, vec!["L", "C"]),
        102 => ("MOV", 1, vec!["H", "M"]),
        103 => ("MOV", 1, vec!["H", "A"]),
        100 => ("MOV", 1, vec!["H", "H"]),
        101 => ("MOV", 1, vec!["H", "L"]),
        98 => ("MOV", 1, vec!["H", "D"]),
        99 => ("MOV", 1, vec!["H", "E"]),
        96 => ("MOV", 1, vec!["H", "B"]),
        97 => ("MOV", 1, vec!["H", "C"]),
        153 => ("SBB", 1, vec!["C"]),
        213 => ("PUSH", 1, vec!["D"]),
        206 => ("ACI", 2, vec!["D8"]),
        205 => ("CALL", 3, vec!["adr"]),
        184 => ("CMP", 1, vec!["B"]),
        185 => ("CMP", 1, vec!["C"]),
        202 => ("JZ", 3, vec!["adr"]),
        204 => ("CZ", 3, vec!["adr"]),
        203 => ("-", 1, vec![]),
        178 => ("ORA", 1, vec!["D"]),
        179 => ("ORA", 1, vec!["E"]),
        176 => ("ORA", 1, vec!["B"]),
        177 => ("ORA", 1, vec!["C"]),
        182 => ("ORA", 1, vec!["M"]),
        183 => ("ORA", 1, vec!["A"]),
        180 => ("ORA", 1, vec!["H"]),
        181 => ("ORA", 1, vec!["L"]),
        227 => ("XTHL", 1, vec![]),
        214 => ("SUI", 2, vec!["D8"]),
        111 => ("MOV", 1, vec!["L", "A"]),
        109 => ("MOV", 1, vec!["L", "L"]),
        110 => ("MOV", 1, vec!["L", "M"]),
        107 => ("MOV", 1, vec!["L", "E"]),
        108 => ("MOV", 1, vec!["L", "H"]),
        106 => ("MOV", 1, vec!["L", "D"]),
        121 => ("MOV", 1, vec!["A", "C"]),
        120 => ("MOV", 1, vec!["A", "B"]),
        113 => ("MOV", 1, vec!["M", "C"]),
        112 => ("MOV", 1, vec!["M", "B"]),
        115 => ("MOV", 1, vec!["M", "E"]),
        114 => ("MOV", 1, vec!["M", "D"]),
        117 => ("MOV", 1, vec!["M", "L"]),
        116 => ("MOV", 1, vec!["M", "H"]),
        119 => ("MOV", 1, vec!["M", "A"]),
        118 => ("HLT", 1, vec![]),
        197 => ("PUSH", 1, vec!["B"]),
        196 => ("CNZ", 3, vec!["adr"]),
        199 => ("RST", 1, vec!["0"]),
        198 => ("ADI", 2, vec!["D8"]),
        193 => ("POP", 1, vec!["B"]),
        139 => ("ADC", 1, vec!["E"]),
        195 => ("JMP", 3, vec!["adr"]),
        194 => ("JNZ", 3, vec!["adr"]),
        187 => ("CMP", 1, vec!["E"]),
        188 => ("CMP", 1, vec!["H"]),
        140 => ("ADC", 1, vec!["H"]),
        191 => ("CMP", 1, vec!["A"]),
        200 => ("RZ", 1, vec![]),
        189 => ("CMP", 1, vec!["L"]),
        190 => ("CMP", 1, vec!["M"]),
        241 => ("POP", 1, vec!["PSW"]),
        233 => ("PCHL", 1, vec![]),
        216 => ("RC", 1, vec![]),
        217 => ("-", 1, vec![]),
        247 => ("RST", 1, vec!["6"]),
        243 => ("DI", 1, vec![]),
        208 => ("RNC", 1, vec![]),
        159 => ("SBB", 1, vec!["A"]),
        158 => ("SBB", 1, vec!["M"]),
        157 => ("SBB", 1, vec!["L"]),
        8 => ("-", 1, vec![]),
        9 => ("DAD", 1, vec!["B"]),
        154 => ("SBB", 1, vec!["D"]),
        215 => ("RST", 1, vec!["2"]),
        4 => ("INR", 1, vec!["B"]),
        5 => ("DCR", 1, vec!["B"]),
        6 => ("MVI", 2, vec!["B", "D8"]),
        7 => ("RLC", 1, vec![]),
        0 => ("NOP", 1, vec![]),
        1 => ("LXI", 3, vec!["B", "D16"]),
        2 => ("STAX", 1, vec!["B"]),
        3 => ("INX", 1, vec!["B"]),
        132 => ("ADD", 1, vec!["H"]),
        133 => ("ADD", 1, vec!["L"]),
        134 => ("ADD", 1, vec!["M"]),
        135 => ("ADD", 1, vec!["A"]),
        128 => ("ADD", 1, vec!["B"]),
        129 => ("ADD", 1, vec!["C"]),
        130 => ("ADD", 1, vec!["D"]),
        131 => ("ADD", 1, vec!["E"]),
        31 => ("RAR", 1, vec![]),
        30 => ("MVI", 2, vec!["E", "D8"]),
        29 => ("DCR", 1, vec!["E"]),
        28 => ("INR", 1, vec!["E"]),
        27 => ("DCX", 1, vec!["D"]),
        26 => ("LDAX", 1, vec!["D"]),
        222 => ("SBI", 2, vec!["D8"]),
        223 => ("RST", 1, vec!["3"]),
        209 => ("POP", 1, vec!["D"]),
        210 => ("JNC", 3, vec!["adr"]),
        211 => ("OUT", 2, vec!["D8"]),
        156 => ("SBB", 1, vec!["H"]),
        155 => ("SBB", 1, vec!["E"]),
        141 => ("ADC", 1, vec!["L"]),
        142 => ("ADC", 1, vec!["M"]),
        143 => ("ADC", 1, vec!["A"]),
        224 => ("RPO", 1, vec![]),
        231 => ("RST", 1, vec!["4"]),
        138 => ("ADC", 1, vec!["D"]),
        25 => ("DAD", 1, vec!["D"]),
        24 => ("-", 1, vec![]),
        23 => ("RAL", 1, vec![]),
        22 => ("MVI", 2, vec!["D", "D8"]),
        21 => ("DCR", 1, vec!["D"]),
        20 => ("INR", 1, vec!["D"]),
        19 => ("INX", 1, vec!["D"]),
        18 => ("STAX", 1, vec!["D"]),
        17 => ("LXI", 3, vec!["D", "D16"]),
        16 => ("-", 1, vec![]),
        151 => ("SUB", 1, vec!["A"]),
        150 => ("SUB", 1, vec!["M"]),
        149 => ("SUB", 1, vec!["L"]),
        148 => ("SUB", 1, vec!["H"]),
        147 => ("SUB", 1, vec!["E"]),
        146 => ("SUB", 1, vec!["D"]),
        145 => ("SUB", 1, vec!["C"]),
        144 => ("SUB", 1, vec!["B"]),
        13 => ("DCR", 1, vec!["C"]),
        14 => ("MVI", 2, vec!["C", "D8"]),
        15 => ("RRC", 1, vec![]),
        152 => ("SBB", 1, vec!["B"]),
        10 => ("LDAX", 1, vec!["B"]),
        11 => ("DCX", 1, vec!["B"]),
        12 => ("INR", 1, vec!["C"]),

        _ => ("unknown", 1, vec![])
   }
}

enum Register8080 {
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

trait InstructionSet8080 {
    fn instruction_cpi(&self, data1: u8);
    fn instruction_sub(&self, register1: Register8080);
    fn instruction_jz(&self, address1: u8);
    fn instruction_cpo(&self, address1: u8);
    fn instruction_aci(&self, data1: u8);
    fn instruction_cmc(&self);
    fn instruction_cpe(&self, address1: u8);
    fn instruction_cma(&self);
    fn instruction_ani(&self, data1: u8);
    fn instruction_jm(&self, address1: u8);
    fn instruction_sbi(&self, data1: u8);
    fn instruction_rz(&self);
    fn instruction_lhld(&self, address1: u8);
    fn instruction_ei(&self);
    fn instruction_shld(&self, address1: u8);
    fn instruction_sim(&self);
    fn instruction_jc(&self, address1: u8);
    fn instruction_dad(&self, register1: Register8080);
    fn instruction_jnc(&self, address1: u8);
    fn instruction_lda(&self, address1: u8);
    fn instruction_rp(&self);
    fn instruction_daa(&self);
    fn instruction_jmp(&self, address1: u8);
    fn instruction_di(&self);
    fn instruction_rrc(&self);
    fn instruction_pop(&self, register1: Register8080);
    fn instruction_ret(&self);
    fn instruction_rim(&self);
    fn instruction_rpe(&self);
    fn instruction_dcx(&self, register1: Register8080);
    fn instruction_rc(&self);
    fn instruction_xchg(&self);
    fn instruction_rm(&self);
    fn instruction_cmp(&self, register1: Register8080);
    fn instruction_dcr(&self, register1: Register8080);
    fn instruction_rpo(&self);
    fn instruction_out(&self, data1: u8);
    fn instruction_cnz(&self, address1: u8);
    fn instruction_xri(&self, data1: u8);
    fn instruction_sta(&self, address1: u8);
    fn instruction_cm(&self, address1: u8);
    fn instruction_stc(&self);
    fn instruction_cc(&self, address1: u8);
    fn instruction_jp(&self, address1: u8);
    fn instruction_xra(&self, register1: Register8080);
    fn instruction_push(&self, register1: Register8080);
    fn instruction_add(&self, register1: Register8080);
    fn instruction_cnc(&self, address1: u8);
    fn instruction_ldax(&self, register1: Register8080);
    fn instruction_in(&self, data1: u8);
    fn instruction_cz(&self, address1: u8);
    fn instruction_mvi(&self, register1: Register8080, data2: u8);
    fn instruction_cp(&self, address1: u8);
    fn instruction_xthl(&self);
    fn instruction_stax(&self, register1: Register8080);
    fn instruction_adi(&self, data1: u8);
    fn instruction_sui(&self, data1: u8);
    fn instruction_pchl(&self);
    fn instruction_inx(&self, register1: Register8080);
    fn instruction_ana(&self, register1: Register8080);
    fn instruction_jpo(&self, address1: u8);
    fn instruction_sphl(&self);
    fn instruction_rnc(&self);
    fn instruction_jnz(&self, address1: u8);
    fn instruction_hlt(&self);
    fn instruction_jpe(&self, address1: u8);
    fn instruction_mov(&self, register1: Register8080, register2: Register8080);
    fn instruction_inr(&self, register1: Register8080);
    fn instruction_rar(&self);
    fn instruction_sbb(&self, register1: Register8080);
    fn instruction_rnz(&self);
    fn instruction_call(&self, address1: u8);
    fn instruction_rlc(&self);
    fn instruction_ori(&self, data1: u8);
    fn instruction_rst(&self, data1: u8);
    fn instruction_ora(&self, register1: Register8080);
    fn instruction_nop(&self);
    fn instruction_ral(&self);
    fn instruction_lxi(&self, register1: Register8080, data2: u16);
    fn instruction_adc(&self, register1: Register8080);
}