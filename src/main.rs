use std::env;

use aosp_missing_blobs::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = &args[1..];

    run(&paths).expect("An error occurred.");
}
