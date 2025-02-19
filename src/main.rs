use clap::{AppSettings, Arg, SubCommand};
use std::error::Error;
use std::ffi::OsStr;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let current_dir = env::current_dir()?;
    let args = get_args(current_dir.as_os_str());

    disable_color_output(&args);

    match args.subcommand() {
        ("", None) => {
            let total_warnings = dotenv_linter::check(&args, &current_dir)?;

            if total_warnings == 0 {
                process::exit(0);
            }
        }
        ("fix", Some(fix_args)) => {
            dotenv_linter::fix(&fix_args, &current_dir)?;
            process::exit(0);
        }
        ("list", Some(_)) => {
            dotenv_linter::available_check_names()
                .iter()
                .for_each(|name| println!("{}", name));

            process::exit(0);
        }
        ("compare", Some(compare_args)) => {
            disable_color_output(&compare_args);

            let warnings = dotenv_linter::compare(&compare_args, &current_dir)?;
            if warnings.is_empty() {
                process::exit(0);
            }
        }
        _ => {
            eprintln!("unknown command");
        }
    }

    process::exit(1);
}

fn get_args(current_dir: &OsStr) -> clap::ArgMatches {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .args(common_args(current_dir).as_ref())
        .subcommand(
            SubCommand::with_name("list")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("l")
                .usage("dotenv-linter list")
                .about("Shows list of available checks"),
        )
        .subcommand(
            SubCommand::with_name("fix")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("f")
                .args(common_args(current_dir).as_ref())
                .arg(
                    Arg::with_name("no-backup")
                        .long("no-backup")
                        .help("Prevents backing up .env files"),
                )
                .usage("dotenv-linter fix [FLAGS] [OPTIONS] <input>...")
                .about("Automatically fixes warnings"),
        )
        .subcommand(
            SubCommand::with_name("compare")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("c")
                .args(&vec![
                    Arg::with_name("input")
                        .help("Files to compare")
                        .multiple(true)
                        .min_values(2)
                        .required(true),
                    no_color_flag(),
                    quiet_flag(),
                ])
                .about("Compares if files have the same keys")
                .usage("dotenv-linter compare <files>..."),
        )
        .get_matches()
}

fn disable_color_output(args: &clap::ArgMatches) {
    if args.is_present("no-color") {
        colored::control::set_override(false);
    }
}

fn quiet_flag() -> clap::Arg<'static, 'static> {
    Arg::with_name("quiet")
        .short("q")
        .long("quiet")
        .help("Doesn't display additional information")
}

fn no_color_flag() -> clap::Arg<'static, 'static> {
    Arg::with_name("no-color")
        .long("no-color")
        .help("Turns off the colored output")
}

fn common_args(current_dir: &OsStr) -> Vec<Arg> {
    vec![
        Arg::with_name("input")
            .help("files or paths")
            .index(1)
            .default_value_os(current_dir)
            .required(true)
            .multiple(true),
        Arg::with_name("exclude")
            .short("e")
            .long("exclude")
            .value_name("FILE_NAME")
            .help("Excludes files from check")
            .multiple(true)
            .takes_value(true),
        Arg::with_name("skip")
            .short("s")
            .long("skip")
            .value_name("CHECK_NAME")
            .help("Skips checks")
            .multiple(true)
            .takes_value(true),
        Arg::with_name("recursive")
            .short("r")
            .long("recursive")
            .help("Recursively searches and checks .env files"),
        no_color_flag(),
        quiet_flag(),
    ]
}
