extern crate any2png;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        return;
    }
    any2png::main(&args);
}
