use std::io::{self, Write};
use byteorder::{LittleEndian, WriteBytesExt};

#[repr(C)]
pub struct ElfHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Default for ElfHeader {
    fn default() -> Self {
        let mut header = Self {
            e_ident: [0; 16],
            e_type: 2,  // ET_EXEC
            e_machine: 183,  // EM_AARCH64
            e_version: 1,
            e_entry: 0x400000,
            e_phoff: 64,  // Program header offset
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 64,
            e_phentsize: 56,
            e_phnum: 1,
            e_shentsize: 64,
            e_shnum: 0,
            e_shstrndx: 0,
        };

        // Set ELF magic numbers
        header.e_ident[0] = 0x7f;
        header.e_ident[1] = b'E';
        header.e_ident[2] = b'L';
        header.e_ident[3] = b'F';
        header.e_ident[4] = 2;  // ELFCLASS64
        header.e_ident[5] = 1;  // ELFDATA2LSB
        header.e_ident[6] = 1;  // EV_CURRENT

        header
    }
}

impl ElfHeader {
    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.e_ident)?;
        writer.write_u16::<LittleEndian>(self.e_type)?;
        writer.write_u16::<LittleEndian>(self.e_machine)?;
        writer.write_u32::<LittleEndian>(self.e_version)?;
        writer.write_u64::<LittleEndian>(self.e_entry)?;
        writer.write_u64::<LittleEndian>(self.e_phoff)?;
        writer.write_u64::<LittleEndian>(self.e_shoff)?;
        writer.write_u32::<LittleEndian>(self.e_flags)?;
        writer.write_u16::<LittleEndian>(self.e_ehsize)?;
        writer.write_u16::<LittleEndian>(self.e_phentsize)?;
        writer.write_u16::<LittleEndian>(self.e_phnum)?;
        writer.write_u16::<LittleEndian>(self.e_shentsize)?;
        writer.write_u16::<LittleEndian>(self.e_shnum)?;
        writer.write_u16::<LittleEndian>(self.e_shstrndx)?;
        Ok(())
    }
}