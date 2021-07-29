//! Instructions related to processor interrupts and traps.

use enum_primitive::FromPrimitive;
use faucon_asm::{Instruction, Operand};

use super::{Cpu, CpuFlag, Trap, PC};

/// Returns from an interrupt handler.
pub fn iret(cpu: &mut Cpu, _: &Instruction) -> usize {
    // Restore return address from the stack.
    cpu.registers[PC] = cpu.stack_pop();

    // Restore the interrupt state.
    cpu.registers
        .set_flag(CpuFlag::IE0, cpu.registers.get_flag(CpuFlag::IS0));
    cpu.registers
        .set_flag(CpuFlag::IE1, cpu.registers.get_flag(CpuFlag::IS1));
    cpu.registers
        .set_flag(CpuFlag::IE2, cpu.registers.get_flag(CpuFlag::IS2));

    // Signal regular PC increment to the CPU.
    cpu.increment_pc = true;

    1
}

/// Triggers a software trap.
pub fn trap(cpu: &mut Cpu, insn: &Instruction) -> usize {
    // Extract the instruction operands (trap value).
    let trap = insn.operands()[0];

    // Trigger the software trap.
    if let Operand::UnsignedImmediate(imm) = trap {
        cpu.trigger_trap(Trap::from_u32(imm).unwrap());
    } else {
        unreachable!();
    }

    // Signal irregular PC modification to the CPU.
    cpu.increment_pc = false;

    1
}
