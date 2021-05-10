use std::{env, process};

use aosp_missing_blobs::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = &args[1..];

    if let Err(e) = run(&paths) {
        println!("An error occurred: {}", e);
        process::exit(1)
    }
}
