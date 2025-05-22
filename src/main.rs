use aosp_missing_blobs::MissingBlobs;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("PATHS")
                .help("Paths to blobs")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Read blobs in each path recursively")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let paths = matches.get_many::<String>("PATHS").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    let recursive = matches.get_flag("recursive");

    MissingBlobs::builder()
        .recursive(recursive)
        .build()
        .run(&paths);
}
