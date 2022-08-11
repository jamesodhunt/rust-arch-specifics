use anyhow::Result;
use std::env;
use std::process::exit;

mod arch;

fn run_checks(value: &str) -> Result<()> {
    println!("INFO: main: run_checks: value: {:?}", value);

    arch::check(value)
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];

    if args.len() < 2 {
        println!("ERROR: {}: specify string", program_name);
        exit(1);
    }

    let value = &args[1];

    run_checks(value)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        exit(1);
    }
}
