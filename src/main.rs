use std::env;
use utils::error;

pub mod interpreter;
pub mod utils;

fn main() {
    let mut args = env::args();

    if args.len() == 1 {
        error("\nERROR: Expected filename.\nCorrect usage: `pain [filename]`".to_string());
    } else {
        let file = args.nth(1).unwrap();
        if !file.ends_with(".pain") {
            error("\nThe file should end with `.pain`.".to_string());
        }
        interpreter::interpret(file.as_str());
    }
}
