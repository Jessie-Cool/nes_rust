#[cfg(test)]
mod test {
    use crate::cpu::cpu::Cpu;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = Cpu::new();
        cpu.load_and_run_program(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1111_1111 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_and_run_program(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010);
    }

    #[test]
    fn test_0xa9_lda_subsequent_load_of_zero() {
        let mut cpu = Cpu::new();
        cpu.load_and_run_program(vec![0xa9, 0x05, 0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010);
    }

    #[test]
    fn test_0xa9_lda_subsequent_load_of_non_zero() {
        let mut cpu = Cpu::new();
        cpu.load_and_run_program(vec![0xa9, 0x00, 0xa9, 0x0a, 0x00]);
        assert_eq!(cpu.register_a, 0x0a);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1111_1111 == 0);
    }

    #[test]
    fn test_0xaa_move_a_to_x() {
        let mut cpu = Cpu::new();

        cpu.load_and_run_program(vec![0xa9, 0x10, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0x10);
    }

    #[test]
    fn test_0xaa_move_a_to_x_zero() {
        let mut cpu = Cpu::new();

        cpu.load_and_run_program(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status == 0b0000_0010);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = Cpu::new();
        cpu.load_and_run_program(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_x = 0xff;
        cpu.load_and_run_program(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory_zero_page_addressing() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run_program(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_lda_from_memory_immediate_addressing() {
        let mut cpu = Cpu::new();
        cpu.load_and_run_program(vec![0xa9, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x10);
    }

    #[test]
    fn test_lda_from_memory_absolute_addressing() {
        let mut cpu = Cpu::new();
        cpu.mem_write_u16(0x10, 0x5501);
        cpu.load_and_run_program(vec![0xad, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x01);
    }
}
