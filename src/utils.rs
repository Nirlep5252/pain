use std::process;

pub fn error(err: String) {
    println!("{}", err);
    process::exit(-1);
}
