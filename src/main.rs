use aosp_missing_blobs::MissingBlobs;
use clap::{crate_description, crate_name, crate_version, App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
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
    let recursive = matches.is_present("recursive");

    MissingBlobs::new(recursive).run(&paths);
}
