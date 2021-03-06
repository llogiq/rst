
use std::ffi::OsString;

use super::ls;
use super::init;
use super::tutorial;
use super::types::*;

pub fn get_matches<'a, I, T>(args: I) -> ClapResult<ArgMatches<'a>>
    where I: IntoIterator<Item=T>, T: Into<OsString> {
    // [#SPC-ui-cmdline-cmd-help]
    App::new("rst")
        .version(env!("CARGO_PKG_VERSION"))
        .about("the requirements tracking tool made for developers. Call `rst init -t` for \
                a tutorial")
        .author("https://github.com/vitiral/rst")
        .settings(&[AS::SubcommandRequiredElseHelp, AS::VersionlessSubcommands,
                    AS::DeriveDisplayOrder, AS::ColoredHelp])
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("sets the level of verbosity, use multiple (up to 3) to increase")
             .global(true))
        .arg(Arg::with_name("quiet")
             .short("q")
             .long("quiet")
             .help("if set no output will be printed")
             .global(true))
        .subcommand(tutorial::get_subcommand())
        .subcommand(init::get_subcommand())
        .subcommand(ls::get_subcommand())
        .get_matches_from_safe(args)
}
