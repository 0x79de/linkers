use crate::Result;
use std::collections::HashMap;
use log::{debug, warn};

#[derive(Debug, Clone)]
pub struct SymbolResolution {
    pub symbol_name: String,
    pub section_index: usize,
    pub value: u64,
    pub size: u64,
    pub is_defined: bool,
}

pub struct SymbolResolver {
    symbol_map: HashMap<String, SymbolResolution>,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self {
            symbol_map: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, name: String, resolution: SymbolResolution) -> Result<()> {
        if let Some(existing) = self.symbol_map.get(&name) {
            if existing.is_defined && resolution.is_defined {
                warn!("Symbol {} multiply defined", name);
                return Ok(());
            }
        }
        
        debug!("Adding symbol: {} at value: {:#x}", name, resolution.value);
        self.symbol_map.insert(name, resolution);
        Ok(())
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<&SymbolResolution> {
        self.symbol_map.get(name)
    }

    pub fn get_all_symbols(&self) -> impl Iterator<Item = (&String, &SymbolResolution)> {
        self.symbol_map.iter()
    }
}