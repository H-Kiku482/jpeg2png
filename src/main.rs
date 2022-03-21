extern crate jpeg2png;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        return;
    }
    jpeg2png::main(&args);
}
