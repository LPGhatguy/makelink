use std::path::PathBuf;

use symlink::symlink_auto;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Options {
    /// This is where the link will point to.
    source: PathBuf,

    /// This is where the link will be created.
    dest: PathBuf,
}

fn main() {
    let options = Options::from_args();

    if let Err(err) = symlink_auto(&options.source, &options.dest) {
        eprintln!("could not create symlink: {}", err);
        std::process::exit(1);
    }
}
