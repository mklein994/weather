use chrono::{Local, Timelike};
use clap::{App, Arg, Shell, SubCommand};

lazy_static! {
    // This is the same as $(date -Is), that is, "%Y-%m-%dT%H:%M:%S%:z".
    // Nanoseconds are stripped away because the Dark Sky API doesn't accept them.
    static ref NOW: String = Local::now().with_nanosecond(0).unwrap().to_rfc3339();
}

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .long("config")
                .help("path to config.toml file")
                .takes_value(true),
        )
        .arg(Arg::with_name("latitude").env("WEATHER_LAT"))
        .arg(Arg::with_name("longitude").env("WEATHER_LON"))
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
                .long_help(concat!(
                    "run with a local file found in ~/.config/",
                    crate_name!(),
                    "/config.toml"
                ))
                .conflicts_with("historical"),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("just show the raw json response")
                .conflicts_with_all(&["i3", "long", "local"]),
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
                .default_value(&NOW)
                .help("Make a Time Machine request")
                .long_help(
                    "Make a Time Machine request. Optionally takes a UNIX timestamp. This \
                     conflicts with `debug`, because this program doesn't know beforehand \
                     whether the local file is historical or current. [default: <now>]",
                )
                .hide_default_value(true),
        )
        .subcommand(
            SubCommand::with_name("completions").arg(
                Arg::with_name("shell")
                    .required(true)
                    .possible_values(&Shell::variants()),
            ),
        )
}
