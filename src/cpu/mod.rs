//! Falcon microprocessor abstractions.

use faucon_asm::{disassembler, Instruction};

use crate::memory::{LookupError, Memory, PageFlag};
use instructions::process_instruction;
pub use registers::*;

mod instructions;
mod registers;

/// Representation of the Falcon processor.
pub struct Cpu {
    /// The Falcon CPU registers.
    registers: CpuRegisters,
    /// The Falcon SRAM for code and data.
    memory: Memory,
}

impl Cpu {
    /// Retrieves the next instruction by looking up the TLB corresponding to the given
    /// virtual address.
    fn fetch_insn(&self, address: u32) -> faucon_asm::Result<Instruction> {
        // Look up the TLB to get the physical code page.
        let (page, tlb) = match self.memory.tlb.lookup(address) {
            Ok((page, tlb)) => (page, tlb),
            Err(LookupError::NoPageHits) => todo!("Generate trap"),
            Err(LookupError::MultiplePageHits) => todo!("Generate trap"),
        };
        let page_offset = ((address >> 8) & 0xFF) as usize;

        // If the page is marked usable, complete the access using the physical page.
        if tlb.get_flag(PageFlag::Usable) {
            let mut code = &self.memory.code[page as usize][page_offset..];
            return Ok(disassembler::read_instruction(&mut code)?);
        } else if tlb.get_flag(PageFlag::Busy) {
            // The page is marked busy, the access must be completed when possible.
            todo!("Wait for the page to be marked as usable");
        } else if tlb.get_flag(PageFlag::Secret) {
            // As secret is the only flag that is being set, complete the
            // access in Authenticated Mode.
            todo!("^");
        }

        // If the page wouldn't have any flags at all, the method
        // would already opt out at TLB lookup.
        unreachable!();
    }

    /// Executes the next instruction at the address held by the PC register.
    pub fn step(&self) {
        match self.fetch_insn(self.registers.get_pc()) {
            Ok(insn) => process_instruction(insn),
            Err(faucon_asm::Error::InvalidInstruction(_)) => todo!("Generate trap"),
            _ => todo!("Handle these errors in a sane way"),
        }
    }
}
