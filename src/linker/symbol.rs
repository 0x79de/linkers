use std::collections::HashMap;
use crate::{LinkerError, Result};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub section_index: usize,
    pub binding: SymbolBinding,
    pub typ: SymbolType,
    pub visibility: SymbolVisibility,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    NoType,
    Object,
    Func,
    Section,
    File,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolVisibility {
    Default,
    Internal,
    Hidden,
    Protected,
}

pub struct SymbolTable {
    symbols: Vec<Symbol>,
    symbol_map: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            symbol_map: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<usize> {
        if let Some(&existing_idx) = self.symbol_map.get(&symbol.name) {
            let existing = &self.symbols[existing_idx];
            
            // Handle symbol precedence rules
            match (existing.binding, symbol.binding) {
                (SymbolBinding::Global, SymbolBinding::Global) => {
                    return Err(LinkerError::SymbolResolution(
                        format!("Multiple definition of symbol: {}", symbol.name)
                    ));
                }
                (SymbolBinding::Weak, _) => {
                    // Replace weak symbol with new symbol
                    self.symbols[existing_idx] = symbol;
                    return Ok(existing_idx);
                }
                (_, SymbolBinding::Weak) => {
                    // Keep existing symbol
                    return Ok(existing_idx);
                }
                _ => {}
            }
        }

        let index = self.symbols.len();
        self.symbol_map.insert(symbol.name.clone(), index);
        self.symbols.push(symbol);
        Ok(index)
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_map.get(name).map(|&idx| &self.symbols[idx])
    }

    pub fn get_symbol_by_index(&self, index: usize) -> Option<&Symbol> {
        self.symbols.get(index)
    }

    pub fn resolve_undefined_symbols(&self) -> Result<()> {
        for symbol in &self.symbols {
            if symbol.binding == SymbolBinding::Global && 
               symbol.typ == SymbolType::NoType &&
               symbol.section_index == 0 {
                return Err(LinkerError::SymbolResolution(
                    format!("Undefined symbol: {}", symbol.name)
                ));
            }
        }
        Ok(())
    }
}