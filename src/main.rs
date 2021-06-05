use aosp_missing_blobs::{run, Config};
use clap::{App, Arg};

fn main() {
    let matches = App::new("aosp-missing-blobs")
        .version("0.4.0")
        .author("Josh Choo <dev@joshuous.com>")
        .about("An AOSP tool to generate a list of required missing blobs.")
        .arg(
            Arg::with_name("PATHS")
                .help("Paths to blobs")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Read blobs in each path recursively"),
        )
        .get_matches();

    let paths = matches.values_of("PATHS").unwrap().collect::<Vec<_>>();

    run(
        &paths,
        Config {
            recursive: matches.is_present("recursive"),
        },
    );
}
