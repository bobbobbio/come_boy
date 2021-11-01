// Copyright 2021 Remi Bernotavicius

use super::{Disassembler, RGBDSInstructionPrinterFactory, ROMAccessor};
use crate::emulator_common::disassembler::SimpleMemoryAccessor;
use std::fmt::Write;
use std::str;

#[cfg(test)]
fn do_disassembler_test(test_rom: &[u8], expected_str: &str) {
    let mut output = vec![];
    let ma = ROMAccessor::new(test_rom);
    let mut disassembler = Disassembler::new(&ma, RGBDSInstructionPrinterFactory, &mut output);
    disassembler
        .disassemble(0u16..test_rom.len() as u16, true)
        .unwrap();
    assert_eq!(str::from_utf8(&output).unwrap(), expected_str);
}

#[cfg(test)]
fn do_simple_disassembler_test(test_rom: &[u8], expected_str: &str) {
    let mut output = vec![];
    {
        let mut ma = SimpleMemoryAccessor::new();
        ma.memory[0..test_rom.len()].clone_from_slice(test_rom);
        let mut disassembler = Disassembler::new(&ma, RGBDSInstructionPrinterFactory, &mut output);
        disassembler
            .disassemble(0u16..test_rom.len() as u16, true)
            .unwrap();
    }
    assert_eq!(str::from_utf8(&output).unwrap(), expected_str);
}

#[test]
fn disassembler_rgbds_test() {
    do_simple_disassembler_test(
        &[
            0x00, 0x01, 0x00, 0x0, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00, 0x07, 0x08, 0x10, 0x2, 0x09,
            0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x7d, 0x0f, 0x10, 0x11, 0x31, 0x0, 0x12, 0x13, 0x14,
            0x15, 0x16, 0x00, 0x17, 0x18, 0x04, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x7d, 0x1f,
            0x20, 0xa2, 0x21, 0xa5, 0x4, 0x22, 0x23, 0x24, 0x25, 0x26, 0xa8, 0x27, 0x28, 0x40,
            0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x00, 0x2f, 0x30, 0x7d, 0x31, 0x01, 0x2, 0x32,
            0x33, 0x34, 0x35, 0x36, 0x7d, 0x37, 0x38, 0x3c, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e,
            0xa1, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b,
            0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
            0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
            0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
            0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83,
            0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91,
            0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
            0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad,
            0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb,
            0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0x00, 0x8, 0xc3, 0xd6, 0x6, 0xc4, 0x73, 0xf,
            0xc5, 0xc6, 0x7c, 0xc7, 0xc8, 0xc9, 0xca, 0xed, 0x6, 0xcb, 0x07, 0xcc, 0x71, 0xe, 0xcd,
            0x64, 0x3, 0xce, 0x07, 0xcf, 0xd0, 0xd1, 0xd2, 0xef, 0x7, 0xd3, 0xd4, 0x07, 0xd, 0xd5,
            0xd6, 0x71, 0xd7, 0xd8, 0xd9, 0xda, 0x7a, 0xd, 0xdb, 0xdc, 0xba, 0x9, 0xdd, 0xde, 0xfe,
            0xdf, 0xe0, 0x7e, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0x77, 0xe7, 0xe8, 0xe8, 0xe9,
            0xea, 0x71, 0xd, 0xeb, 0xec, 0xed, 0xee, 0x71, 0xef, 0xf0, 0x77, 0xf1, 0xf2, 0xf3,
            0xf4, 0xf5, 0xf6, 0x7e, 0xf7, 0xf8, 0x20, 0xf9, 0xfa, 0x6e, 0x2, 0xfb, 0xfc, 0xfd,
            0xfe, 0x7c, 0xff,
        ],
        "\
         0000000 00       nop\n\
         0000001 01 00 00 ld   bc,$0000\n\
         0000004 02       ld   [bc],a\n\
         0000005 03       inc  bc\n\
         0000006 04       inc  b\n\
         0000007 05       dec  b\n\
         0000008 06 00    ld   b,$00\n\
         000000a 07       rlca\n\
         000000b 08 10 02 ld   [$0210],sp\n\
         000000e 09       add  hl,bc\n\
         000000f 0a       ld   a,[bc]\n\
         0000010 0b       dec  bc\n\
         0000011 0c       inc  c\n\
         0000012 0d       dec  c\n\
         0000013 0e 7d    ld   c,$7D\n\
         0000015 0f       rrca\n\
         0000016 10       -\n\
         0000017 11 31 00 ld   de,$0031\n\
         000001a 12       ld   [de],a\n\
         000001b 13       inc  de\n\
         000001c 14       inc  d\n\
         000001d 15       dec  d\n\
         000001e 16 00    ld   d,$00\n\
         0000020 17       rla\n\
         0000021 18 04    jr   $0027\n\
         0000023 19       add  hl,de\n\
         0000024 1a       ld   a,[de]\n\
         0000025 1b       dec  de\n\
         0000026 1c       inc  e\n\
         0000027 1d       dec  e\n\
         0000028 1e 7d    ld   e,$7D\n\
         000002a 1f       rra\n\
         000002b 20 a2    jr   nz,$FFCF\n\
         000002d 21 a5 04 ld   hl,$04A5\n\
         0000030 22       ldi  [hl],a\n\
         0000031 23       inc  hl\n\
         0000032 24       inc  h\n\
         0000033 25       dec  h\n\
         0000034 26 a8    ld   h,$A8\n\
         0000036 27       daa\n\
         0000037 28 40    jr   z,$0079\n\
         0000039 29       add  hl,hl\n\
         000003a 2a       ldi  a,[hl]\n\
         000003b 2b       dec  hl\n\
         000003c 2c       inc  l\n\
         000003d 2d       dec  l\n\
         000003e 2e 00    ld   l,$00\n\
         0000040 2f       cpl\n\
         0000041 30 7d    jr   nc,$00C0\n\
         0000043 31 01 02 ld   sp,$0201\n\
         0000046 32       ldd  [hl],a\n\
         0000047 33       inc  sp\n\
         0000048 34       inc  [hl]\n\
         0000049 35       dec  [hl]\n\
         000004a 36 7d    ld   [hl],$7D\n\
         000004c 37       scf\n\
         000004d 38 3c    jr   c,$008B\n\
         000004f 39       add  hl,sp\n\
         0000050 3a       ldd  a,[hl]\n\
         0000051 3b       dec  sp\n\
         0000052 3c       inc  a\n\
         0000053 3d       dec  a\n\
         0000054 3e a1    ld   a,$A1\n\
         0000056 3f       ccf\n\
         0000057 40       ld   b,b\n\
         0000058 41       ld   b,c\n\
         0000059 42       ld   b,d\n\
         000005a 43       ld   b,e\n\
         000005b 44       ld   b,h\n\
         000005c 45       ld   b,l\n\
         000005d 46       ld   b,[hl]\n\
         000005e 47       ld   b,a\n\
         000005f 48       ld   c,b\n\
         0000060 49       ld   c,c\n\
         0000061 4a       ld   c,d\n\
         0000062 4b       ld   c,e\n\
         0000063 4c       ld   c,h\n\
         0000064 4d       ld   c,l\n\
         0000065 4e       ld   c,[hl]\n\
         0000066 4f       ld   c,a\n\
         0000067 50       ld   d,b\n\
         0000068 51       ld   d,c\n\
         0000069 52       ld   d,d\n\
         000006a 53       ld   d,e\n\
         000006b 54       ld   d,h\n\
         000006c 55       ld   d,l\n\
         000006d 56       ld   d,[hl]\n\
         000006e 57       ld   d,a\n\
         000006f 58       ld   e,b\n\
         0000070 59       ld   e,c\n\
         0000071 5a       ld   e,d\n\
         0000072 5b       ld   e,e\n\
         0000073 5c       ld   e,h\n\
         0000074 5d       ld   e,l\n\
         0000075 5e       ld   e,[hl]\n\
         0000076 5f       ld   e,a\n\
         0000077 60       ld   h,b\n\
         0000078 61       ld   h,c\n\
         0000079 62       ld   h,d\n\
         000007a 63       ld   h,e\n\
         000007b 64       ld   h,h\n\
         000007c 65       ld   h,l\n\
         000007d 66       ld   h,[hl]\n\
         000007e 67       ld   h,a\n\
         000007f 68       ld   l,b\n\
         0000080 69       ld   l,c\n\
         0000081 6a       ld   l,d\n\
         0000082 6b       ld   l,e\n\
         0000083 6c       ld   l,h\n\
         0000084 6d       ld   l,l\n\
         0000085 6e       ld   l,[hl]\n\
         0000086 6f       ld   l,a\n\
         0000087 70       ld   [hl],b\n\
         0000088 71       ld   [hl],c\n\
         0000089 72       ld   [hl],d\n\
         000008a 73       ld   [hl],e\n\
         000008b 74       ld   [hl],h\n\
         000008c 75       ld   [hl],l\n\
         000008d 76       halt\n\
         000008e 77       ld   [hl],a\n\
         000008f 78       ld   a,b\n\
         0000090 79       ld   a,c\n\
         0000091 7a       ld   a,d\n\
         0000092 7b       ld   a,e\n\
         0000093 7c       ld   a,h\n\
         0000094 7d       ld   a,l\n\
         0000095 7e       ld   a,[hl]\n\
         0000096 7f       ld   a,a\n\
         0000097 80       add  b\n\
         0000098 81       add  c\n\
         0000099 82       add  d\n\
         000009a 83       add  e\n\
         000009b 84       add  h\n\
         000009c 85       add  l\n\
         000009d 86       add  [hl]\n\
         000009e 87       add  a\n\
         000009f 88       adc  b\n\
         00000a0 89       adc  c\n\
         00000a1 8a       adc  d\n\
         00000a2 8b       adc  e\n\
         00000a3 8c       adc  h\n\
         00000a4 8d       adc  l\n\
         00000a5 8e       adc  [hl]\n\
         00000a6 8f       adc  a\n\
         00000a7 90       sub  b\n\
         00000a8 91       sub  c\n\
         00000a9 92       sub  d\n\
         00000aa 93       sub  e\n\
         00000ab 94       sub  h\n\
         00000ac 95       sub  l\n\
         00000ad 96       sub  [hl]\n\
         00000ae 97       sub  a\n\
         00000af 98       sbc  b\n\
         00000b0 99       sbc  c\n\
         00000b1 9a       sbc  d\n\
         00000b2 9b       sbc  e\n\
         00000b3 9c       sbc  h\n\
         00000b4 9d       sbc  l\n\
         00000b5 9e       sbc  [hl]\n\
         00000b6 9f       sbc  a\n\
         00000b7 a0       and  b\n\
         00000b8 a1       and  c\n\
         00000b9 a2       and  d\n\
         00000ba a3       and  e\n\
         00000bb a4       and  h\n\
         00000bc a5       and  l\n\
         00000bd a6       and  [hl]\n\
         00000be a7       and  a\n\
         00000bf a8       xor  b\n\
         00000c0 a9       xor  c\n\
         00000c1 aa       xor  d\n\
         00000c2 ab       xor  e\n\
         00000c3 ac       xor  h\n\
         00000c4 ad       xor  l\n\
         00000c5 ae       xor  [hl]\n\
         00000c6 af       xor  a\n\
         00000c7 b0       or   b\n\
         00000c8 b1       or   c\n\
         00000c9 b2       or   d\n\
         00000ca b3       or   e\n\
         00000cb b4       or   h\n\
         00000cc b5       or   l\n\
         00000cd b6       or   [hl]\n\
         00000ce b7       or   a\n\
         00000cf b8       cp   b\n\
         00000d0 b9       cp   c\n\
         00000d1 ba       cp   d\n\
         00000d2 bb       cp   e\n\
         00000d3 bc       cp   h\n\
         00000d4 bd       cp   l\n\
         00000d5 be       cp   [hl]\n\
         00000d6 bf       cp   a\n\
         00000d7 c0       ret  nz\n\
         00000d8 c1       pop  bc\n\
         00000d9 c2 00 08 jp   nz,$0800\n\
         00000dc c3 d6 06 jp   $06D6\n\
         00000df c4 73 0f call nz,$0F73\n\
         00000e2 c5       push bc\n\
         00000e3 c6 7c    add  a,$7C\n\
         00000e5 c7       rst  $00\n\
         00000e6 c8       ret  z\n\
         00000e7 c9       ret\n\
         00000e8 ca ed 06 jp   z,$06ED\n\
         00000eb cb 07    rl   a\n\
         00000ed cc 71 0e call z,$0E71\n\
         00000f0 cd 64 03 call $0364\n\
         00000f3 ce 07    adc  a,$07\n\
         00000f5 cf       rst  $08\n\
         00000f6 d0       ret  nc\n\
         00000f7 d1       pop  de\n\
         00000f8 d2 ef 07 jp   nc,$07EF\n\
         00000fb d3       -\n\
         00000fc d4 07 0d call nc,$0D07\n\
         00000ff d5       push de\n\
         0000100 d6 71    sub  a,$71\n\
         0000102 d7       rst  $10\n\
         0000103 d8       ret  c\n\
         0000104 d9       reti\n\
         0000105 da 7a 0d jp   c,$0D7A\n\
         0000108 db       -\n\
         0000109 dc ba 09 call c,$09BA\n\
         000010c dd       -\n\
         000010d de fe    sbc  a,$FE\n\
         000010f df       rst  $18\n\
         0000110 e0 7e    ldh  [$FF7E],a\n\
         0000112 e1       pop  hl\n\
         0000113 e2       ld   [$FF00+c],a\n\
         0000114 e3       -\n\
         0000115 e4       -\n\
         0000116 e5       push hl\n\
         0000117 e6 77    and  a,$77\n\
         0000119 e7       rst  $20\n\
         000011a e8 e8    add  sp,$E8\n\
         000011c e9       jp   [hl]\n\
         000011d ea 71 0d ld   [$0D71],a\n\
         0000120 eb       -\n\
         0000121 ec       -\n\
         0000122 ed       -\n\
         0000123 ee 71    xor  a,$71\n\
         0000125 ef       rst  $28\n\
         0000126 f0 77    ldh  a,[$FF77]\n\
         0000128 f1       pop  af\n\
         0000129 f2       ld   a,[$FF00+c]\n\
         000012a f3       di\n\
         000012b f4       -\n\
         000012c f5       push af\n\
         000012d f6 7e    or   a,$7E\n\
         000012f f7       rst  $30\n\
         0000130 f8 20    ld   hl,[sp+$20]\n\
         0000132 f9       ld   sp,hl\n\
         0000133 fa 6e 02 ld   a,[$026E]\n\
         0000136 fb       ei\n\
         0000137 fc       -\n\
         0000138 fd       -\n\
         0000139 fe 7c    cp   a,$7C\n\
         000013b ff       rst  $38\n\
         ",
    );
}

#[test]
fn disassembler_rgbds_prints_not_implemented_instructions_correctly() {
    do_disassembler_test(
        &[0xd3, 0xe3, 0xe4, 0xf4],
        "\
         0000000 d3       -\n\
         0000001 e3       -\n\
         0000002 e4       -\n\
         0000003 f4       -\n\
         ",
    );
}

#[test]
fn disassembler_rgbds_prints_db_correctly() {
    let mut d = vec![];
    let mut s = String::new();
    for a in 0..0x0104 {
        d.push(0x0);
        write!(s, "{:07x} 00       nop\n", a).unwrap();
    }

    d.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    writeln!(
        s,
        "0000104          \
         db   $01,$02,$03,$04,$05,$06,$07,$08,$09,$0A,$0B,$0C,$0D,$0E,$0F,$10"
    )
    .unwrap();

    d.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    writeln!(
        s,
        "0000114          \
         db   $01,$02,$03,$04,$05,$06,$07,$08,$09,$0A,$0B,$0C,$0D,$0E,$0F,$10"
    )
    .unwrap();

    d.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    writeln!(
        s,
        "0000124          \
         db   $01,$02,$03,$04,$05,$06,$07,$08,$09,$0A,$0B,$0C,$0D,$0E,$0F,$10"
    )
    .unwrap();

    d.extend_from_slice("HELLOTHERE99999".as_bytes());
    writeln!(s, "0000134          db   \"HELLOTHERE99999\"").unwrap();

    do_disassembler_test(&d, &s);
}
