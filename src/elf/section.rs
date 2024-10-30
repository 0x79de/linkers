use std::io::{self, Write};
use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
    pub data: Vec<u8>,
}

impl Section {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
            data: Vec::new(),
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(0)?; // sh_name (string table offset)
        writer.write_u32::<LittleEndian>(self.sh_type)?;
        writer.write_u64::<LittleEndian>(self.sh_flags)?;
        writer.write_u64::<LittleEndian>(self.sh_addr)?;
        writer.write_u64::<LittleEndian>(self.sh_offset)?;
        writer.write_u64::<LittleEndian>(self.sh_size)?;
        writer.write_u32::<LittleEndian>(self.sh_link)?;
        writer.write_u32::<LittleEndian>(self.sh_info)?;
        writer.write_u64::<LittleEndian>(self.sh_addralign)?;
        writer.write_u64::<LittleEndian>(self.sh_entsize)?;
        writer.write_all(&self.data)?;
        Ok(())
    }
}