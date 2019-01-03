use quicli::prelude::*;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Cli {
    arg: String,
}
fn main() -> CliResult {
    let args = Cli::from_args();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", args.arg);
    println!("  ret");

    Ok(())
}
