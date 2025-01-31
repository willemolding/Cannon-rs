use crate::cli::CargoSubcommand;
use clap::Parser;
use io::Write;
use std::ffi::OsString;
use std::{env, io, process};

mod cli;

fn main() {
    let result: Result<i32, _> = cargo_cannon();
    process::exit(match result {
        Ok(code) => code,
        Err(err) => {
            let _ = writeln!(io::stderr(), "{}", err);
            1
        }
    });
}

fn cargo_binary() -> OsString {
    env::var_os("CARGO").unwrap_or_else(|| "cargo".to_owned().into())
}

fn cargo_cannon() -> Result<i32, anyhow::Error> {
    let CargoSubcommand::Cannon(args) = CargoSubcommand::parse();

    // Cross compile the MIPS32 using our docker image

    // Patch the resulting elf and convert to a Cannon json file

    Ok(0)
}
