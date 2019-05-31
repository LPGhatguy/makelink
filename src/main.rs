use symlink::symlink_auto;
use clap::{App, Arg};

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("makelink")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))

        .arg(Arg::with_name("LINK")
            .help("Where the symlink should be created")
            .takes_value(true)
            .required(true)
            .index(1))

        .arg(Arg::with_name("TARGET")
            .help("Where the symlink should point")
            .takes_value(true)
            .required(true)
            .index(2))

        .get_matches();

    let link_name = matches.value_of("LINK").unwrap();
    let target = matches.value_of("TARGET").unwrap();

    symlink_auto(target, link_name)?;
    Ok(())
}
