use std::{env, process};
use vmtranslator::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Program format: <input asm path> <output hack path>");
        process::exit(1);
    });

    if let Err(e) = vmtranslator::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
