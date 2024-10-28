use std::io;
use thiserror::Error;

pub use elf;
pub use linker;

#[derive(Error, Debug)]
pub enum LinkerError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid ELF format: {0}")]
    InvalidElf(String),

    #[error("Symbol resolution error: {0}")]
    SymbolResolution(String),

    #[error("Relocation error: {0}")]
    Relocation(String),

    #[error("Section error: {0}")]
    Section(String),
}

pub type Result<T> = std::result::Result<T, LinkerError>;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
