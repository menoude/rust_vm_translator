// pub mod code_writer;
// pub mod constants;
// pub mod parser;
// pub mod utils;

use std::process;
use vm_translator::translate;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let nb_args = args.len() - 1;
        println!(
            "{} argument{} specified. Usage: 'cargo run file'",
            nb_args,
            if nb_args > 1 { "s" } else { "" }
        );
        process::exit(1);
    }
    translate(args[1].to_owned()).unwrap_or_else(|e| {
        println!("An error occured: {}", e);
        process::exit(1);
    });
}
