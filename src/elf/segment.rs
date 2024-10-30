use std::io::{self, Write};
use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Debug, Clone)]
pub struct Segment {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
    pub data: Vec<u8>,
}

impl Segment {
    pub fn new() -> Self {
        Self {
            p_type: 1,  // PT_LOAD
            p_flags: 7,  // PF_R | PF_W | PF_X
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0x1000,
            data: Vec::new(),
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(self.p_type)?;
        writer.write_u32::<LittleEndian>(self.p_flags)?;
        writer.write_u64::<LittleEndian>(self.p_offset)?;
        writer.write_u64::<LittleEndian>(self.p_vaddr)?;
        writer.write_u64::<LittleEndian>(self.p_paddr)?;
        writer.write_u64::<LittleEndian>(self.p_filesz)?;
        writer.write_u64::<LittleEndian>(self.p_memsz)?;
        writer.write_u64::<LittleEndian>(self.p_align)?;
        writer.write_all(&self.data)?;
        Ok(())
    }
}