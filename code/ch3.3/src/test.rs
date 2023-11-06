use super::*;
#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}
#[test]
fn test_0xa9_lda_immediate_tax() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0xAA, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
    assert_eq!(cpu.register_x, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}
#[test]
fn test_0xa9_lda_immediate_tay() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0xA8, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
    assert_eq!(cpu.register_y, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0xAA, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    cpu.load_and_run(vec![0xAA, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}
#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn test_lda_from_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xb5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn test_lda_from_u16() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0x64);

    cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x64);
}
#[test]
fn test_lda_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0x42);
    cpu.mem_write(0x1042, 0x23);
    cpu.load_and_run(vec![0xa9, 0x42, 0xaa, 0xbd, 0x00, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x23);
}
#[test]
fn test_lda_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0x42);
    cpu.mem_write(0x1042, 0x23);
    cpu.load_and_run(vec![0xa9, 0x42, 0xa8, 0xb9, 0x00, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x23);
}
#[test]
fn test_0xa2_ldx_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
    assert_eq!(cpu.register_x, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_0xa2_ldx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}
#[test]
fn test_ldx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa6, 0x10, 0x00]);

    assert_eq!(cpu.register_x, 0x55);
}
#[test]
fn test_ldx_zero_flag_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xb6, 0x10, 0x00]);

    assert_eq!(cpu.register_x, 0x55);
}

#[test]
fn test_ldx_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1234, 0x55);
    cpu.load_and_run(vec![0xae, 0x34, 0x12]);

    assert_eq!(cpu.register_x, 0x55);
}
#[test]
fn test_ldx_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1234, 0x55);
    cpu.load_and_run(vec![0xa9, 0x10, 0xa8, 0xbe, 0x24, 0x12]);

    assert_eq!(cpu.register_x, 0x55);
}
#[test]
fn test_nop() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xea, 0x00]);
    assert_eq!(cpu.program_counter, 0x8002);
}
#[test]
fn test_txa() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x10, 0x8A, 0x00]);
    assert_eq!(cpu.register_a, 0x10);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
    cpu.load_and_run(vec![0xA2, 0x00, 0x8A, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}
#[test]
fn test_tya() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x10, 0xA8, 0x98, 0x00]);
    assert_eq!(cpu.register_a, 0x10);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
    cpu.load_and_run(vec![0xA9, 0x00, 0xA8, 0x98, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}
#[test]
fn test_iny() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x10, 0xA8, 0xC8, 0x00]);
    assert_eq!(cpu.register_y, 0x11);
}
#[test]
fn test_cld() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xd8, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0001_0000, 0);
}
#[test]
fn test_cli() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x58, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_0100, 0);
}
#[test]
fn test_clv() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xb8, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0100_0000, 0);
}
#[test]
fn test_clc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x18, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_0001, 0);
}
#[test]
fn test_sec() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x38, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_0001, 1);
}
#[test]
fn test_sei() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x78, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_0100, 0b0000_0100);
}
#[test]
fn test_sed() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xf8, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_1000, 0b0000_1000);
}
#[test]
fn test_txs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x10, 0x9a, 0x00]);
    assert_eq!(cpu.stack_pointer, 0x10);
    cpu.load_and_run(vec![0xa2, 0x00, 0x9a, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_0010, 2);
}
#[test]
fn test_tsx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x10, 0x9a, 0xba, 0x00]);
    assert_eq!(cpu.register_x, 0x10);
    cpu.load_and_run(vec![0xa2, 0x00, 0x9a, 0xba, 0x00]);
    assert_eq!(cpu.status.bits() & 0b0000_0010, 2);
}
#[test]
fn test_0xa0_ldy_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
    assert_eq!(cpu.register_y, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_0xa0_ldy_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
    assert_eq!(cpu.register_y, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}
#[test]
fn test_ldy_zero_flag() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa4, 0x10, 0x00]);

    assert_eq!(cpu.register_y, 0x55);
}
#[test]
fn test_ldy_zero_flag_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x55);

    cpu.load_and_run(vec![0xa2, 0x01, 0xb4, 0x10, 0x00]);

    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn test_ldy_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1234, 0x55);
    cpu.load_and_run(vec![0xac, 0x34, 0x12]);

    assert_eq!(cpu.register_y, 0x55);
}
#[test]
fn test_ldy_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1234, 0x55);
    cpu.load_and_run(vec![0xa2, 0x10, 0xbc, 0x24, 0x12]);

    assert_eq!(cpu.register_y, 0x55);
}
#[test]
fn test_stx_zeropage() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x23, 0x86, 0x55, 0x00]);

    let test = cpu.mem_read(0x55);
    assert_eq!(test, 0x23);
}
#[test]
fn test_stx_zeropage_y() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x01, 0xA2, 0x23, 0x96, 0x55, 0x00]);

    let test = cpu.mem_read(0x56);
    assert_eq!(test, 0x23);
}
#[test]
fn test_stx_abs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x23, 0x8E, 0x55, 0x10, 0x00]);

    let test = cpu.mem_read(0x1055);
    assert_eq!(test, 0x23);
}
#[test]
fn test_sty_zeropage() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x23, 0x84, 0x55, 0x00]);

    let test = cpu.mem_read(0x55);
    assert_eq!(test, 0x23);
}
#[test]
fn test_sty_zeropage_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x01, 0xA0, 0x23, 0x94, 0x55, 0x00]);

    let test = cpu.mem_read(0x56);
    assert_eq!(test, 0x23);
}
#[test]
fn test_sty_abs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x23, 0x8C, 0x55, 0x10, 0x00]);

    let test = cpu.mem_read(0x1055);
    assert_eq!(test, 0x23);
}
//Stack tests
#[test]
fn test_pha() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x10, 0x48, 0x00]);
    let test = cpu.mem_read(0x1fd);
    assert_eq!(test, 0x10);
}

#[test]
fn test_php() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x38, 0x78, 0xf8, 0x08, 0x00]);
    let mut test = cpu.mem_read(0x1fd);
    test &= 0b00101111;
    assert_eq!(test, cpu.status.bits());
}
#[test]
fn test_pla() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x38, 0x78, 0xf8, 0x08, 0x68, 0x00]);
    cpu.register_a &= 0b00101111;
    assert_eq!(cpu.register_a, cpu.status.bits());
}
#[test]
fn test_plp() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x38, 0x78, 0xf8, 0xa9, 0b100100, 0x48, 0x28, 0x00]);
    assert_eq!(cpu.register_a, cpu.status.bits());
}
//test - ADC
#[test]
fn test_adc_immediate() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 0x10, 0x69, 0x10]);
    assert_eq!(cpu.register_a, 0x20);

    cpu.load_and_run(vec![0xA9, 0x50, 0x69, 0x50]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.load_and_run(vec![0xA9, 0x50, 0x69, 0xD0]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.load_and_run(vec![0xA9, 0xD0, 0x69, 0x90]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.load_and_run(vec![0xA9, 0xD0, 0x69, 0xD0]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x65, 0x10]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x10, 0x50);
    cpu.load_and_run(vec![0xA9, 0x50, 0x65, 0x10]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0x65, 0x10]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0x65, 0x10]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x75, 0x10]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x11, 0x50);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0x75, 0x10]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x11, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0x75, 0x10]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x11, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa2, 0x01, 0x75, 0x10]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x6D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x1034, 0x50);
    cpu.load_and_run(vec![0xA9, 0x50, 0x6D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1034, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0x6D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1034, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0x6D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x7D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x1035, 0x50);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0x7D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0x7D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa2, 0x01, 0x7D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x79, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x1035, 0x50);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa0, 0x01, 0x79, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa0, 0x01, 0x79, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa0, 0x01, 0x79, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_ind_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x61, 0x50]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0x61, 0x50]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0x61, 0x50]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa2, 0x01, 0x61, 0x50]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_adc_ind_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x71, 0x50]);
    assert_eq!(cpu.register_a, 0x60);

    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x50, 0xa0, 0x01, 0x71, 0x50]);
    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa0, 0x01, 0x71, 0x50]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa0, 0x01, 0x71, 0x50]);
    assert_eq!(cpu.register_a, 0x60);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
//test -  SBC
#[test]
fn test_sbc_immediate() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 10, 0xE9, 10]);
    assert_eq!(cpu.register_a, 0x00);

    cpu.load_and_run(vec![0xA9, 0x30, 0xE9, 0x50]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.load_and_run(vec![0xA9, 0x50, 0xE9, 0xD0]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.load_and_run(vec![0xA9, 0xD0, 0xE9, 0x90]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.load_and_run(vec![0xA9, 0xD0, 0xE9, 0xD0]);
    assert_eq!(cpu.register_a, 0);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xE5, 0x10]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x10, 0x50);
    cpu.load_and_run(vec![0xA9, 0x30, 0xE5, 0x10]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xE5, 0x10]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xE5, 0x10]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0xF5, 0x10]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x11, 0x50);
    cpu.load_and_run(vec![0xA9, 0x30, 0xa2, 0x01, 0xF5, 0x10]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x11, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0xF5, 0x10]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x11, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa2, 0x01, 0xF5, 0x10]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xED, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x1034, 0x50);
    cpu.load_and_run(vec![0xA9, 0x30, 0xED, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1034, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xED, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1034, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xED, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0xFD, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x1035, 0x50);
    cpu.load_and_run(vec![0xA9, 0x30, 0xa2, 0x01, 0xFD, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0xFD, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa2, 0x01, 0xFD, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0xF9, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x1035, 0x50);
    cpu.load_and_run(vec![0xA9, 0x30, 0xa0, 0x01, 0xF9, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa0, 0x01, 0xF9, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa0, 0x01, 0xF9, 0x34, 0x10]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_ind_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0xE1, 0x50]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x30, 0xa2, 0x01, 0xE1, 0x50]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa2, 0x01, 0xE1, 0x50]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa2, 0x01, 0xE1, 0x50]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_sbc_ind_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0xF1, 0x50]);
    assert_eq!(cpu.register_a, (0x10 - 0x50) as u8);

    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x30, 0xa0, 0x01, 0xF1, 0x50]);
    assert_eq!(cpu.register_a, (0x30 - 0x50) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0xD0);
    cpu.load_and_run(vec![0xA9, 0x50, 0xa0, 0x01, 0xF1, 0x50]);
    assert_eq!(cpu.register_a, (0x50 - 0xD0) as u8);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(!cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x90);
    cpu.load_and_run(vec![0xA9, 0xD0, 0xa0, 0x01, 0xF1, 0x50]);
    assert_eq!(cpu.register_a, (0xD0 - 0x90) as u8);
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
//test - AND
#[test]
fn test_and_immediate() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 0x10, 0x29, 0x10]);
    assert_eq!(cpu.register_a, (0x10 & 0x10));

    cpu.load_and_run(vec![0xA9, 0x30, 0x29, 0x50]);
    assert_eq!(cpu.register_a, (0x30 & 0x50) as u8);
}
#[test]
fn test_and_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x25, 0x10]);
    assert_eq!(cpu.register_a, (0x10 & 0x50));
}
#[test]
fn test_and_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x35, 0x10]);
    assert_eq!(cpu.register_a, 0x10 & 0x50);
}
#[test]
fn test_and_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x2D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 & 0x50);
}
#[test]
fn test_and_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x3D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 & 0x50);
}
#[test]
fn test_and_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x39, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 & 0x50);
}
#[test]
fn test_and_ind_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x21, 0x50]);
    assert_eq!(cpu.register_a, 0x10 & 0x50);
}
#[test]
fn test_and_ind_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x31, 0x50]);
    assert_eq!(cpu.register_a, 0x10 & 0x50);
}
//test - EOR
#[test]
fn test_eor_immediate() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 0x10, 0x49, 0x10]);
    assert_eq!(cpu.register_a, (0x10 ^ 0x10));

    cpu.load_and_run(vec![0xA9, 0x50, 0x49, 0x50]);
    assert_eq!(cpu.register_a, (0x50 ^ 0x50) as u8);
}
#[test]
fn test_eor_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x45, 0x10]);
    assert_eq!(cpu.register_a, (0x10 ^ 0x50));
}
#[test]
fn test_eor_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x55, 0x10]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x50);
}
#[test]
fn test_eor_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x4D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x50);
}
#[test]
fn test_eor_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x5D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x50);
}
#[test]
fn test_eor_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x59, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x50);
}
#[test]
fn test_eor_ind_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x41, 0x50]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x50);
}
#[test]
fn test_eor_ind_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x51, 0x50]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x50);
}
//test - EOR
#[test]
fn test_ora_immediate() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 0x10, 0x09, 0x10]);
    assert_eq!(cpu.register_a, (0x10 | 0x10));

    cpu.load_and_run(vec![0xA9, 0x50, 0x09, 0x50]);
    assert_eq!(cpu.register_a, (0x50 | 0x50));
}
#[test]
fn test_ora_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x05, 0x10]);
    assert_eq!(cpu.register_a, (0x10 | 0x50));
}
#[test]
fn test_ora_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x15, 0x10]);
    assert_eq!(cpu.register_a, 0x10 | 0x50);
}
#[test]
fn test_ora_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0x0D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 | 0x50);
}
#[test]
fn test_ora_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x1D, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 | 0x50);
}
#[test]
fn test_ora_abs_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x19, 0x34, 0x10]);
    assert_eq!(cpu.register_a, 0x10 | 0x50);
}
#[test]
fn test_ora_ind_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x01, 0x50]);
    assert_eq!(cpu.register_a, 0x10 | 0x50);
}
#[test]
fn test_ora_ind_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xA9, 0x10, 0xa0, 0x01, 0x11, 0x50]);
    assert_eq!(cpu.register_a, 0x10 | 0x50);
}
//test - ASL
#[test]
fn test_asl_accumalator() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 0x10, 0x0A]);
    assert_eq!(cpu.register_a, (0x10 << 1));

    cpu.load_and_run(vec![0xA9, 0xFF, 0x0A]);
    assert_eq!(cpu.register_a, 0b11111110);
    println!("{:b}", cpu.register_a);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_asl_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x50);
    cpu.load_and_run(vec![0x06, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0x50 << 1);
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0x06, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0xFF << 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_asl_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);
    cpu.load_and_run(vec![0xA2, 0x01, 0x16, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0x50 << 1);
    cpu.mem_write(0x11, 0xFF);
    cpu.load_and_run(vec![0xA2, 0x01, 0x16, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0xFF << 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_asl_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0x0E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0x50 << 1);

    cpu.mem_write(0x1034, 0xFF);

    cpu.load_and_run(vec![0x0E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0xFF << 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_asl_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xa2, 0x01, 0x1E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0x50 << 1);

    cpu.mem_write(0x1035, 0xFF);

    cpu.load_and_run(vec![0xa2, 0x01, 0x1E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0xff << 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
//test - lsr
#[test]
fn test_lsr_accumalator() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA9, 0x10, 0x4A]);
    assert_eq!(cpu.register_a, (0x10 >> 1));

    cpu.load_and_run(vec![0xA9, 0xFF, 0x4A]);
    assert_eq!(cpu.register_a, 0b01111111);
    println!("{:b}", cpu.register_a);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_lsr_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x50);
    cpu.load_and_run(vec![0x46, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0x50 >> 1);
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0x46, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0xFF >> 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_lsr_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11, 0x50);
    cpu.load_and_run(vec![0xA2, 0x01, 0x56, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0x50 >> 1);
    cpu.mem_write(0x11, 0xFF);
    cpu.load_and_run(vec![0xA2, 0x01, 0x56, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0xFF >> 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_lsr_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0x4E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0x50 >> 1);

    cpu.mem_write(0x1034, 0xFF);

    cpu.load_and_run(vec![0x4E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0xFF >> 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_lsr_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0xa2, 0x01, 0x5E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0x50 >> 1);

    cpu.mem_write(0x1035, 0xFF);

    cpu.load_and_run(vec![0xa2, 0x01, 0x5E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0xff >> 1);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
//test - rol
#[test]
fn test_rol_accumalator() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0x38, 0xA9, 0x10, 0x2A]);
    assert_eq!(cpu.register_a, (0x10 << 1) | 1);

    cpu.load_and_run(vec![0x18, 0xA9, 0xFF, 0x2A]);
    assert_eq!(cpu.register_a, 0b11111110);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.load_and_run(vec![0x38, 0xA9, 0xFF, 0x2A]);
    assert_eq!(cpu.register_a, 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_rol_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    cpu.load_and_run(vec![0x38, 0x26, 0x10]);
    assert_eq!(cpu.mem_read(0x10), (0x10 << 1) | 1);
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0x18, 0x26, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0b11111110);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0x38, 0x26, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_rol_zero_page_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x11, 0x10);
    cpu.load_and_run(vec![0xa2, 0x01, 0x38, 0x36, 0x10]);
    assert_eq!(cpu.mem_read(0x11), (0x10 << 1) | 1);
    cpu.mem_write(0x11, 0xFF);
    cpu.load_and_run(vec![0xa2, 0x01, 0x18, 0x36, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0b11111110);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.mem_write(0x11, 0xFF);
    cpu.load_and_run(vec![0xa2, 0x01, 0x38, 0x36, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_rol_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0x38, 0x2E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), (0x50 << 1) | 1);

    cpu.mem_write(0x1034, 0xFF);

    cpu.load_and_run(vec![0x18, 0x2E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0b11111110);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1034, 0xFF);

    cpu.load_and_run(vec![0x38, 0x2E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_rol_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0x38, 0xa2, 0x01, 0x3E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), (0x50 << 1) | 1);

    cpu.mem_write(0x1035, 0xFF);

    cpu.load_and_run(vec![0x18, 0xa2, 0x01, 0x3E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0b11111110);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0xFF);

    cpu.load_and_run(vec![0x38, 0xa2, 0x01, 0x3E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
//test - ror
#[test]
fn test_ror_accumalator() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0x38, 0xA9, 0x10, 0x6A]);
    assert_eq!(cpu.register_a, (0x10 >> 1) | 0x80);

    cpu.load_and_run(vec![0x18, 0xA9, 0xFF, 0x6A]);
    assert_eq!(cpu.register_a, 0b01111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.load_and_run(vec![0x38, 0xA9, 0xFF, 0x6A]);
    assert_eq!(cpu.register_a, 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_ror_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    cpu.load_and_run(vec![0x38, 0x66, 0x10]);
    assert_eq!(cpu.mem_read(0x10), (0x10 >> 1) | 0x80);
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0x18, 0x66, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0b01111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0x38, 0x66, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_ror_zero_page_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x11, 0x10);
    cpu.load_and_run(vec![0xa2, 0x01, 0x38, 0x76, 0x10]);
    assert_eq!(cpu.mem_read(0x11), (0x10 >> 1) | 0x80);
    cpu.mem_write(0x11, 0xFF);
    cpu.load_and_run(vec![0xa2, 0x01, 0x18, 0x76, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0b01111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.mem_write(0x11, 0xFF);
    cpu.load_and_run(vec![0xa2, 0x01, 0x38, 0x76, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_ror_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x50);

    cpu.load_and_run(vec![0x38, 0x6E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), (0x50 >> 1) | 0x80);

    cpu.mem_write(0x1034, 0xFF);

    cpu.load_and_run(vec![0x18, 0x6E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0b01111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1034, 0xFF);

    cpu.load_and_run(vec![0x38, 0x6E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
#[test]
fn test_ror_abs_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1035, 0x50);

    cpu.load_and_run(vec![0x38, 0xa2, 0x01, 0x7E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), (0x50 >> 1) | 0x80);

    cpu.mem_write(0x1035, 0xFF);

    cpu.load_and_run(vec![0x18, 0xa2, 0x01, 0x7E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0b01111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x1035, 0xFF);

    cpu.load_and_run(vec![0x38, 0xa2, 0x01, 0x7E, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}
//INC
#[test]
fn test_inc_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    cpu.load_and_run(vec![0xE6, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0x11);

    cpu.mem_write(0x10, 0b01111111);
    cpu.load_and_run(vec![0xE6, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0b10000000);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x10, 0b11111111);
    cpu.load_and_run(vec![0xE6, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_inc_zero_page_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x11, 0x10);
    cpu.load_and_run(vec![0xA2, 0x01, 0xF6, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0x11);

    cpu.mem_write(0x11, 0b01111111);
    cpu.load_and_run(vec![0xA2, 0x01, 0xF6, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0b10000000);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x11, 0b11111111);
    cpu.load_and_run(vec![0xA2, 0x01, 0xF6, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_inc_abs() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1034, 0x10);
    cpu.load_and_run(vec![0xEE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0x11);

    cpu.mem_write(0x1034, 0b01111111);
    cpu.load_and_run(vec![0xEE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0b10000000);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x1034, 0b11111111);
    cpu.load_and_run(vec![0xEE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_inc_abs_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1035, 0x10);
    cpu.load_and_run(vec![0xA2, 0x01, 0xFE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0x11);

    cpu.mem_write(0x1035, 0b01111111);
    cpu.load_and_run(vec![0xA2, 0x01, 0xFE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0b10000000);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x1035, 0b11111111);
    cpu.load_and_run(vec![0xA2, 0x01, 0xFE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
//DEC
#[test]
fn test_dec_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    cpu.load_and_run(vec![0xC6, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0x10 - 1);

    cpu.mem_write(0x10, 0);
    cpu.load_and_run(vec![0xC6, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x10, 1);
    cpu.load_and_run(vec![0xC6, 0x10]);
    assert_eq!(cpu.mem_read(0x10), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_dec_zero_page_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x11, 0x10);
    cpu.load_and_run(vec![0xA2, 0x01, 0xD6, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0x10 - 1);

    cpu.mem_write(0x11, 0);
    cpu.load_and_run(vec![0xA2, 0x01, 0xD6, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x11, 1);
    cpu.load_and_run(vec![0xA2, 0x01, 0xD6, 0x10]);
    assert_eq!(cpu.mem_read(0x11), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_dec_abs() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1034, 0x10);
    cpu.load_and_run(vec![0xCE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0x10 - 1);

    cpu.mem_write(0x1034, 0);
    cpu.load_and_run(vec![0xCE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x1034, 1);
    cpu.load_and_run(vec![0xCE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1034), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_dec_abs_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1035, 0x10);
    cpu.load_and_run(vec![0xA2, 0x01, 0xDE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0x10 - 1);

    cpu.mem_write(0x1035, 0);
    cpu.load_and_run(vec![0xA2, 0x01, 0xDE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0b11111111);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x1035, 1);
    cpu.load_and_run(vec![0xA2, 0x01, 0xDE, 0x34, 0x10]);
    assert_eq!(cpu.mem_read(0x1035), 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_dex() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA2, 1, 0xCA]);
    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));

    cpu.load_and_run(vec![0xA2, 0, 0xCA]);
    assert_eq!(cpu.register_x, 0xFF);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_dey() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xA0, 1, 0x88]);
    assert_eq!(cpu.register_y, 0);
    assert!(cpu.status.contains(CpuFlags::ZERO));

    cpu.load_and_run(vec![0xA0, 0, 0x88]);
    assert_eq!(cpu.register_y, 0xFF);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
//CMP
#[test]
fn test_cmp_immediate() {
    let mut cpu = CPU::new();

    //equal
    cpu.load_and_run(vec![0xA9, 0x10, 0xC9, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA9, 0x11, 0xC9, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA9, 0x10, 0xC9, 0x11]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cmp_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    //equal
    cpu.load_and_run(vec![0xA9, 0x10, 0xC5, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA9, 0x11, 0xC5, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA9, 0x09, 0xC5, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cmp_zero_page_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x11, 0x10);
    //equal
    cpu.load_and_run(vec![0xA9, 0x10, 0xA2, 0x01, 0xD5, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA9, 0x11, 0xA2, 0x01, 0xD5, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA9, 0x09, 0xA2, 0x01, 0xD5, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cmp_abs() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1034, 0x10);
    //equal
    cpu.load_and_run(vec![0xA9, 0x10, 0xCD, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA9, 0x11, 0xCD, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA9, 0x09, 0xCD, 0x34, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cmp_abs_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1035, 0x10);
    //equal
    cpu.load_and_run(vec![0xA2, 0x01, 0xA9, 0x10, 0xDD, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA2, 0x01, 0xA9, 0x11, 0xDD, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA2, 0x01, 0xA9, 0x09, 0xDD, 0x34, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
// let mut cpu = CPU::new();
// cpu.mem_write(0x50, 0x01);
// cpu.mem_write(0x51, 0x35);
// cpu.mem_write(0x52, 0x10);
// cpu.mem_write(0x1035, 0x50);
//
// cpu.load_and_run(vec![0xA9, 0x10, 0xa2, 0x01, 0x01, 0x50]);
// assert_eq!(cpu.register_a, 0x10 | 0x50);
#[test]
fn test_cmp_ind_x() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x50, 0x01);
    cpu.mem_write(0x51, 0x35);
    cpu.mem_write(0x52, 0x10);
    cpu.mem_write(0x1035, 0x10);
    //equal
    cpu.load_and_run(vec![0xA2, 0x01, 0xA9, 0x10, 0xC1, 0x50]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA2, 0x01, 0xA9, 0x11, 0xC1, 0x50]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA2, 0x01, 0xA9, 0x09, 0xC1, 0x50]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cmp_ind_y() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x50, 0x34);
    cpu.mem_write(0x51, 0x10);
    cpu.mem_write(0x1035, 0x10);
    //equal
    cpu.load_and_run(vec![0xA0, 0x01, 0xA9, 0x10, 0xD1, 0x50]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA0, 0x01, 0xA9, 0x11, 0xD1, 0x50]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA0, 0x01, 0xA9, 0x09, 0xD1, 0x50]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
//CPY
#[test]
fn test_cpy_immediate() {
    let mut cpu = CPU::new();

    //equal
    cpu.load_and_run(vec![0xA0, 0x10, 0xC0, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA0, 0x11, 0xC0, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA0, 0x10, 0xC0, 0x11]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cpy_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    //equal
    cpu.load_and_run(vec![0xA0, 0x10, 0xC4, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA0, 0x11, 0xC4, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA0, 0x09, 0xC4, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cpy_abs() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1034, 0x10);
    //equal
    cpu.load_and_run(vec![0xA0, 0x10, 0xCC, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA0, 0x11, 0xCC, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA0, 0x09, 0xCC, 0x34, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
//CPX
#[test]
fn test_cpx_immediate() {
    let mut cpu = CPU::new();

    //equal
    cpu.load_and_run(vec![0xA2, 0x10, 0xE0, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA2, 0x11, 0xE0, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA2, 0x10, 0xE0, 0x11]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cpx_zero_page() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x10, 0x10);
    //equal
    cpu.load_and_run(vec![0xA2, 0x10, 0xE4, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA2, 0x11, 0xE4, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA2, 0x09, 0xE4, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
#[test]
fn test_cpx_abs() {
    let mut cpu = CPU::new();

    cpu.mem_write(0x1034, 0x10);
    //equal
    cpu.load_and_run(vec![0xA2, 0x10, 0xEC, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //greater
    cpu.load_and_run(vec![0xA2, 0x11, 0xEC, 0x34, 0x10]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    //lesser
    cpu.load_and_run(vec![0xA2, 0x09, 0xEC, 0x34, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}
//JMP
#[test]
fn test_jmp_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1034, 0x00);

    cpu.load_and_run(vec![0x4C, 0x34, 0x10]);
    assert_eq!(cpu.program_counter, 0x1034 + 1);
}
#[test]
fn test_jmp_ind() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11FF, 0x35);
    cpu.mem_write(0x1100, 0x10);

    cpu.load_and_run(vec![0x6C, 0xFF, 0x11]);
    assert_eq!(cpu.program_counter, 0x1035 + 1);
    cpu.mem_write(0x1150, 0x40);
    cpu.mem_write(0x1151, 0x10);
    cpu.load_and_run(vec![0x6C, 0x50, 0x11]);
    assert_eq!(cpu.program_counter, 0x1040 + 1);
}
//JSR - RTS
#[test]
fn test_jsr_rts() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11FF, 0xE8);
    cpu.mem_write(0x11FF + 1, 0x60);

    cpu.load_and_run(vec![0xA2, 3, 0x20, 0xFF, 0x11, 0xCA]);
    assert_eq!(cpu.register_x, 3);
}
//RTI
#[test]
fn test_rti() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x11FF, 0x58);
    cpu.mem_write(0x11FF + 1, 0x18);
    cpu.mem_write(0x11FF + 2, 0x60);
    cpu.load_and_run(vec![0x38, 0x78, 0x08, 0x20, 0xFF, 0x11, 0x40]);
    assert_eq!(cpu.status.bits(), 0b00100101);
}
//BNE
#[test]
fn test_bne() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xD0, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 6 + 1);
}
//BVS
#[test]
fn test_bvs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x50, 0x69, 0x50, 0x70, 4]);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert_eq!(cpu.program_counter, 0x8000 + 10 + 1);
}
//BPL
#[test]
fn test_bpl() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x10, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 6 + 1);
}
//BVC
#[test]
fn test_bvc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x50, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 6 + 1);
}
//BMI
#[test]
fn test_bmi() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0xFF, 0x30, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 8 + 1);
}
//Beq
#[test]
fn test_beq() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0, 0xF0, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 8 + 1);
}
//BCS
#[test]
fn test_bcs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x38, 0xB0, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 7 + 1);
}
//BCC
#[test]
fn test_bcc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x90, 4]);
    assert_eq!(cpu.program_counter, 0x8000 + 6 + 1);
}
//BIT
#[test]
fn test_bit_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x51, 0b01111111);
    cpu.load_and_run(vec![0xA9, 0, 0x24, 0x51]);
    assert!(cpu.status.contains(CpuFlags::ZERO));

    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));

    cpu.mem_write(0x51, 0b11111111);
    cpu.load_and_run(vec![0xA9, 0xFF, 0x24, 0x51]);
    assert!(!cpu.status.contains(CpuFlags::ZERO));

    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
}
#[test]
fn test_bit_abs() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1051, 0b01111111);
    cpu.load_and_run(vec![0xA9, 0, 0x2C, 0x51, 0x10]);
    assert!(cpu.status.contains(CpuFlags::ZERO));

    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));

    cpu.mem_write(0x1051, 0b11111111);
    cpu.load_and_run(vec![0xA9, 0xFF, 0x2C, 0x51, 0x10]);
    assert!(!cpu.status.contains(CpuFlags::ZERO));

    assert!(cpu.status.contains(CpuFlags::NEGATIV));
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
}
