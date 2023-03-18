use std::env;

pub mod interpreter;
pub mod utils;

fn main() {
    let mut args = env::args();
    // dbg!(&args);

    if args.len() == 1 {
        println!("\nERROR: Expected filename.\nCorrect usage: `pain [filename]`");
    } else {
        interpreter::interpret(args.nth(1).unwrap().as_str());
    }
}
