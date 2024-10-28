use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

// ELF Header Constants
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const ET_EXEC: u16 = 2;
const EM_AARCH64: u16 = 183;

#[repr(C, packed)]
struct Elf64_Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C, packed)]
struct Elf64_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

struct Linker {
    output_file: File,
    entry_point: u64,
}

impl Linker {
    fn new(output_path: &Path, entry_point: u64) -> io::Result<Self> {
        let output_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;

        Ok(Linker {
            output_file,
            entry_point,
        })
    }

    fn write_elf_header(&mut self) -> io::Result<()> {
        let mut header = Elf64_Ehdr {
            e_ident: [0; 16],
            e_type: ET_EXEC,
            e_machine: EM_AARCH64,
            e_version: 1,
            e_entry: self.entry_point,
            e_phoff: std::mem::size_of::<Elf64_Ehdr>() as u64,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: std::mem::size_of::<Elf64_Ehdr>() as u16,
            e_phentsize: std::mem::size_of::<Elf64_Phdr>() as u16,
            e_phnum: 1,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        };

        // Set ELF identification bytes
        header.e_ident[0] = 0x7f; // ELF magic number
        header.e_ident[1] = b'E';
        header.e_ident[2] = b'L';
        header.e_ident[3] = b'F';
        header.e_ident[4] = ELFCLASS64;
        header.e_ident[5] = ELFDATA2LSB;
        header.e_ident[6] = EV_CURRENT;

        // Write header to file
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &header as *const _ as *const u8,
                std::mem::size_of::<Elf64_Ehdr>(),
            )
        };
        self.output_file.write_all(header_bytes)
    }

    fn write_program_header(&mut self, segment_size: u64) -> io::Result<()> {
        let phdr = Elf64_Phdr {
            p_type: 1,         // PT_LOAD
            p_flags: 7,        // Read + Write + Execute
            p_offset: 0x1000,  // Offset where segment data begins
            p_vaddr: 0x400000, // Virtual address in memory
            p_paddr: 0x400000, // Physical address (same as virtual for most systems)
            p_filesz: segment_size,
            p_memsz: segment_size,
            p_align: 0x1000,
        };

        let phdr_bytes = unsafe {
            std::slice::from_raw_parts(
                &phdr as *const _ as *const u8,
                std::mem::size_of::<Elf64_Phdr>(),
            )
        };
        self.output_file.write_all(phdr_bytes)
    }

    fn write_segment(&mut self, data: &[u8]) -> io::Result<()> {
        // Seek to segment offset
        self.output_file.seek(SeekFrom::Start(0x1000))?;
        self.output_file.write_all(data)
    }

    fn link_objects(&mut self, object_files: &[&Path]) -> io::Result<()> {
        // Write ELF header
        self.write_elf_header()?;

        // For simplicity, we'll just concatenate all object file contents
        let mut combined_data = Vec::new();
        for object_file in object_files {
            let mut file = File::open(object_file)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            combined_data.extend_from_slice(&contents);
        }

        // Write program header
        self.write_program_header(combined_data.len() as u64)?;

        // Write segment data
        self.write_segment(&combined_data)?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Example usage
    let output_path = Path::new("output.elf");
    let entry_point = 0x400000;
    let mut linker = Linker::new(output_path, entry_point)?;

    // Example object files (you would pass your actual object files here)
    let object_files = vec![Path::new("input1.o"), Path::new("input2.o")];

    linker.link_objects(&object_files)?;
    println!("Linking completed successfully!");
    Ok(())
}
