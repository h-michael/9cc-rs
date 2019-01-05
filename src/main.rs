use quicli::prelude::*;
use structopt::StructOpt;

mod ptr;

#[derive(Debug, StructOpt)]
struct Cli {
    tokens: String,
}
fn main() -> CliResult {
    let args = Cli::from_args();
    let mut ptr = ptr::Ptr::new(args.tokens.as_str());

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", ptr.parse_number());

    while let Some(chr) = ptr.next() {
        match chr {
           '+' => { 
               println!("  add rax, {}", ptr.parse_number());
           },
           '-' => {
               println!("  sub rax, {}", ptr.parse_number());
           },
           _ => panic!("unexpected charactor: {}", ptr.parse_number()),
        }
    }

    println!("  ret");

    Ok(())
}
