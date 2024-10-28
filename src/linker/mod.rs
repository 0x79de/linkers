mod resolver;
mod relocation;
mod symbol;

use crate::elf::{ElfFile, Section, Segment};
use crate::Result;
use std::path::Path;
use goblin::Object;
use std::fs::File;
use std::io::Read;

pub struct Linker {
    output: ElfFile,
    entry_point: u64,
    symbols: symbol::SymbolTable,
}

impl Linker {
    pub fn new(output_path: &Path, entry_point: u64) -> Result<Self> {
        Ok(Self {
            output: ElfFile::new(),
            entry_point,
            symbols: symbol::SymbolTable::new(),
        })
    }

    pub fn link_objects(&mut self, input_files: &[impl AsRef<Path>]) -> Result<()> {
        // First pass: collect symbols
        for input_file in input_files {
            self.collect_symbols(input_file.as_ref())?;
        }

        // Second pass: perform linking
        for input_file in input_files {
            self.process_object_file(input_file.as_ref())?;
        }

        // Write the final output
        self.output.header.e_entry = self.entry_point;
        self.output.write_to_file(output_path)?;

        Ok(())
    }

    fn collect_symbols(&mut self, input_path: &Path) -> Result<()> {
        let mut buffer = Vec::new();
        File::open(input_path)?.read_to_end(&mut buffer)?;

        match Object::parse(&buffer)? {
            Object::Elf(elf) => {
                self.symbols.add_file_symbols(&elf)?;
                Ok(())
            }
            _ => Err(anyhow::anyhow!("Desteklenmeyen dosya formatı"))  // Error handling eklendi
        }
    }
}