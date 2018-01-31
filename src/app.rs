use clap::{App, Arg, ArgMatches};

pub fn build_cli<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .long("config")
                .help("path to config.toml file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("i3")
                .short("3")
                .long("i3blocks")
                .help("format output for i3blocks pango."),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .help("show detailed output"),
        )
        .arg(
            Arg::with_name("debug")
                .short("D")
                .long("debug")
                .help("run with a local file")
                .long_help(&format!(
                    "run with a local file found in ~/.config/{}/config",
                    env!("CARGO_PKG_NAME")
                )),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("just show the raw json response")
                .conflicts_with_all(&["i3", "long"]),
        )
        .arg(
            Arg::with_name("local")
                .long("local")
                .help("use a local file as test data")
                .conflicts_with_all(&["debug", "historical"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("historical")
                .short("H")
                .long("historical")
                .takes_value(true)
                .help("Make a Time Machine request")
                .long_help(
                    "Make a Time Machine request. Optionally takes a UNIX timestamp. This \
                     conflicts with `debug`, because this program doesn't know beforehand \
                     whether the local file is historical or current.",
                )
                .conflicts_with("debug"),
        )
        .get_matches()
}
