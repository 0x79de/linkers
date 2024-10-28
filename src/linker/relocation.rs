use crate::{LinkerError, Result};
use goblin::elf64::reloc::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Relocation {
    pub offset: u64,
    pub symbol_index: usize,
    pub typ: u32,
    pub addend: i64,
}

pub struct RelocationProcessor {
    relocations: Vec<Relocation>,
    symbol_addresses: HashMap<usize, u64>,
}

impl RelocationProcessor {
    pub fn new() -> Self {
        Self {
            relocations: Vec::new(),
            symbol_addresses: HashMap::new(),
        }
    }

    pub fn add_relocation(&mut self, reloc: Relocation) {
        self.relocations.push(reloc);
    }

    pub fn set_symbol_address(&mut self, symbol_index: usize, address: u64) {
        self.symbol_addresses.insert(symbol_index, address);
    }

    pub fn process_relocations(&self, section_data: &mut [u8]) -> Result<()> {
        for reloc in &self.relocations {
            let symbol_addr = self.symbol_addresses.get(&reloc.symbol_index)
                .ok_or_else(|| LinkerError::Relocation(
                    format!("Symbol not found for relocation at offset {:#x}", reloc.offset)
                ))?;

            match reloc.typ {
                R_AARCH64_ABS64 => self.handle_abs64(section_data, reloc, *symbol_addr)?,
                R_AARCH64_CALL26 => self.handle_call26(section_data, reloc, *symbol_addr)?,
                R_AARCH64_ADR_PREL_PG_HI21 => self.handle_adr_prel(section_data, reloc, *symbol_addr)?,
                _ => return Err(LinkerError::Relocation(
                    format!("Unsupported relocation type: {}", reloc.typ)
                )),
            }
        }
        Ok(())
    }

    fn handle_abs64(&self, data: &mut [u8], reloc: &Relocation, symbol_addr: u64) -> Result<()> {
        if reloc.offset as usize + 8 > data.len() {
            return Err(LinkerError::Relocation("Invalid relocation offset".to_string()));
        }

        let value = symbol_addr.wrapping_add(reloc.addend as u64);
        data[reloc.offset as usize..reloc.offset as usize + 8]
            .copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn handle_call26(&self, data: &mut [u8], reloc: &Relocation, symbol_addr: u64) -> Result<()> {
        if reloc.offset as usize + 4 > data.len() {
            return Err(LinkerError::Relocation("Invalid relocation offset".to_string()));
        }

        let pc = reloc.offset & !0x3;
        let target = symbol_addr.wrapping_add(reloc.addend as u64);
        let offset = (target.wrapping_sub(pc)) >> 2;

        if offset > 0x3ffffff {
            return Err(LinkerError::Relocation("Branch target too far".to_string()));
        }

        let insn_offset = reloc.offset as usize;
        let mut insn = u32::from_le_bytes([
            data[insn_offset],
            data[insn_offset + 1],
            data[insn_offset + 2],
            data[insn_offset + 3],
        ]);

        insn = (insn & !0x3ffffff) | (offset as u32 & 0x3ffffff);
        
        data[insn_offset..insn_offset + 4].copy_from_slice(&insn.to_le_bytes());
        Ok(())
    }

    fn handle_adr_prel(&self, data: &mut [u8], reloc: &Relocation, symbol_addr: u64) -> Result<()> {
        // Implementation for ADR_PREL_PG_HI21 relocation
        // This is a simplified version
        let pc = reloc.offset & !0xfff;
        let target = symbol_addr.wrapping_add(reloc.addend as u64);
        let offset = (target.wrapping_sub(pc)) >> 12;

        if offset > 0x1fffff {
            return Err(LinkerError::Relocation("Target address too far".to_string()));
        }

        let insn_offset = reloc.offset as usize;
        let mut insn = u32::from_le_bytes([
            data[insn_offset],
            data[insn_offset + 1],
            data[insn_offset + 2],
            data[insn_offset + 3],
        ]);

        insn = (insn & !0x1fffff) | (offset as u32 & 0x1fffff);
        
        data[insn_offset..insn_offset + 4].copy_from_slice(&insn.to_le_bytes());
        Ok(())
    }
}