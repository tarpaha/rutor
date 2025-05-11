use clap::{Arg, Command};

pub fn parse_args(default_number_of_torrents: usize) -> usize {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("number")
                .help(format!("Number of torrents to show (default: {})", default_number_of_torrents))
                .value_parser(clap::value_parser!(usize))
        )
        .get_matches();
    matches
        .get_one::<usize>("number")
        .map(|n| *n)
        .unwrap_or(default_number_of_torrents)
}