mod header;
mod section;
mod segment;

pub use header::*;
pub use section::*;
pub use segment::*;

use crate::Result;
use std::fs::File;
use std::path::Path;

pub struct ElfFile {
    pub header: ElfHeader,
    pub sections: Vec<Section>,
    pub segments: Vec<Segment>,
}

impl ElfFile {
    pub fn new() -> Self {
        Self {
            header: ElfHeader::default(),
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }

    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        self.header.write(&mut file)?;
        
        for segment in &self.segments {
            segment.write(&mut file)?;
        }
        
        for section in &self.sections {
            section.write(&mut file)?;
        }
        
        Ok(())
    }
}