use advent_of_code::template::commands::{all, download, read, scaffold, solve, time};
use args::{parse, AppArguments};

mod args {
    use std::process;

    use advent_of_code::template::Day;

    pub enum AppArguments {
        Download {
            day: Day,
        },
        Read {
            day: Day,
        },
        Scaffold {
            day: Day,
        },
        Solve {
            day: Day,
            release: bool,
            time: bool,
            submit: Option<u8>,
        },
        All {
            release: bool,
            time: bool,
        },
        Time {
            all: bool,
            no_readme: bool,
            day: Option<Day>,
        },
    }

    pub fn parse() -> Result<AppArguments, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => AppArguments::All {
                release: args.contains("--release"),
                time: args.contains("--time"),
            },
            Some("time") => {
                let all = args.contains("--all");
                let no_readme = args.contains("--no-readme");

                AppArguments::Time {
                    all,
                    no_readme,
                    day: args.opt_free_from_str()?,
                }
            }
            Some("download") => AppArguments::Download {
                day: args.free_from_str()?,
            },
            Some("read") => AppArguments::Read {
                day: args.free_from_str()?,
            },
            Some("scaffold") => AppArguments::Scaffold {
                day: args.free_from_str()?,
            },
            Some("solve") => AppArguments::Solve {
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                time: args.contains("--time"),
            },
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArguments::All { release, time } => all::handle(release, time),
            AppArguments::Time {
                day,
                all,
                no_readme,
            } => time::handle(day, all, !no_readme),
            AppArguments::Download { day } => download::handle(day),
            AppArguments::Read { day } => read::handle(day),
            AppArguments::Scaffold { day } => scaffold::handle(day),
            AppArguments::Solve {
                day,
                release,
                time,
                submit,
            } => solve::handle(day, release, time, submit),
        },
    };
}
