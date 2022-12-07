use super::cpu::*;

fn update_zero_and_carry_flags(cpu_instance: &mut Cpu, result: u8) {
    //handle zero flag
    if result == 0 {
        cpu_instance.status = cpu_instance.status | 0b0000_0010;
    } else {
        cpu_instance.status = cpu_instance.status & 0b1111_1101;
    }

    //handle carry
    if result & 0b1000_0000 != 0 {
        cpu_instance.status = cpu_instance.status | 0b1000_0000;
    } else {
        cpu_instance.status = cpu_instance.status & 0b0111_1111;
    }
}

fn set_carry_flag(cpu_instance: &mut Cpu) {
    cpu_instance.status = cpu_instance.status | 0b0000_0001;
}

pub fn tax(cpu_instance: &mut Cpu) {
    println!("{}", cpu_instance.register_a);
    cpu_instance.register_x = cpu_instance.register_a;

    update_zero_and_carry_flags(cpu_instance, cpu_instance.register_x);
}

pub fn lda(cpu_instance: &mut Cpu, mode: &AddressingMode) {
    let addr = cpu_instance.get_operand_address(mode);
    let value = cpu_instance.mem_read(addr);

    cpu_instance.register_a = value;
    update_zero_and_carry_flags(cpu_instance, cpu_instance.register_a);
}

pub fn sta(cpu_instance: &mut Cpu, mode: &AddressingMode) {
    let addr = cpu_instance.get_operand_address(mode);
    cpu_instance.mem_write(addr, cpu_instance.register_a);
}

pub fn inx(cpu_instance: &mut Cpu) {
    if cpu_instance.register_x == 0xff {
        cpu_instance.register_x = 0x00;
    } else {
        cpu_instance.register_x += 0x01;
    }
    update_zero_and_carry_flags(cpu_instance, cpu_instance.register_x);
}
