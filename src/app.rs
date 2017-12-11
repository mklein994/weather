use clap::{App, Arg, ArgMatches};

pub fn build_cli<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
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
        .get_matches()
}
