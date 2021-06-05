use std::env;

use aosp_missing_blobs::run;
use clap::App;

fn main() {
    App::new("aosp-missing-blobs")
        .version("0.3.0")
        .author("Josh Choo <dev@joshuous.com>")
        .about("An AOSP tool to generate a list of required missing blobs.")
        .get_matches();

    let args: Vec<String> = env::args().collect();
    let paths = &args[1..];

    run(paths);
}
