## Rust ELF Linker
A simple ELF linker written in Rust, designed for educational purposes and embedded system development. This linker supports QEMU testing with semihosting capabilities.

### Features

- 64-bit ELF format support
- Program header generation
- Loadable segment support
- AArch64 architecture support
- Basic object file linking
- QEMU testing compatibility
- Semihosting support

### Installation

1. Clone the repo
```bash
git clone https://github.com/0x79de/linkers.git
cd linkers
```
2. Build the project
```bash
cargo build --release
```

### Usage

Basic usage example: 
```rust
use rust_linker::Linker;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let output_path = Path::new("output.elf");
    let entry_point = 0x400000;
    let mut linker = Linker::new(output_path, entry_point)?;

    let object_files = vec![
        Path::new("input1.o"),
        Path::new("input2.o"),
    ];

    linker.link_objects(&object_files)?;
    println!("Linking completed successfully!");
    Ok(())
}
```
### Testing with QEMU

```bash
cargo build

# Run with QEMU (AArch64)
qemu-system-aarch64 -M virt -cpu cortex-a53 -kernel output.elf -semihosting
```

### Current Features to Implement

- [x] Symbol resolution
- [] Relocation processing
- [] Section merging
- [] Debug information support

### Future Enhancements

- [] Support for multiple architectures
- [] Dynamic linking support
- [] Custom section handling
- [] Optimization options
- [] Better error handling and reporting
- [] Extended DWARF debug support
- [] Cross-platform compatibility
- [] Performance improvements


### resources:
- [ELF Format Documentation](https://refspecs.linuxfoundation.org/elf/elf.pdf)
- [Ian Lance Taylor linkers blog posts](https://www.airs.com/blog/archives/38)
