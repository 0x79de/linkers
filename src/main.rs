use clap::Parser;
use rust_linker::{Linker, Result};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Input object files
    #[clap(name = "INPUT", required = true)]
    input_files: Vec<PathBuf>,

    /// Output file
    #[clap(short, long, default_value = "output.elf")]
    output: PathBuf,

    /// Entry point address
    #[clap(short, long, default_value = "0x400000")]
    entry: u64,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    if args.verbose {
        println!("Rust Linker v{}", rust_linker::VERSION);
        println!("Input files: {:?}", args.input_files);
        println!("Output file: {:?}", args.output);
        println!("Entry point: 0x{:x}", args.entry);
    }

    let mut linker = Linker::new(&args.output, args.entry)?;
    linker.link_objects(&args.input_files)?;

    println!("Linking completed successfully!");
    Ok(())
}