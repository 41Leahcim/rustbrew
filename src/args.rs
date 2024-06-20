use clap::{Arg, Command, CommandFactory, FromArgMatches, Parser};

/// Parses the arguments passed by the user
#[derive(Debug)]
#[allow(dead_code)]
pub struct Args {
    language: String,
    build_dep: bool,
}

impl CommandFactory for Args {
    fn command() -> Command {
        // Create the command with all required info and arguments
        Command::new("rustbrew")
        .about("Count all programs written/built in X language or Y build system or Z library distributed via Homebrew.")
        .long_about("Count all programs written/built in X language or Y build system or Z library distributed via Homebrew. Get all build dependencies of all packages in Homebrew Core formulae")
        .args([
            Arg::new("lang")
                .short('l')
                .help("get count of all packages which have this language/build-system/library as a dependency (required)"),
            Arg::new("build-dep")
                .short('b')
                .help("show building dependencies for all packages in Homebrew Core")
        ])
    }

    fn command_for_update() -> Command {
        // Just calls the normal `command` function
        Self::command()
    }
}

impl FromArgMatches for Args {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        // Take the language from the lang argument or default to Rust (Go in the original version).
        let language = matches.get_one::<String>("lang").map_or_else(||{
            eprintln!("No language nor build system nor library is specified. Counting packages built in Rust (by default):");
            "rust".to_owned()
        }, String::to_owned);

        // Take the build-dep argument or default to false
        let build_dep = matches
            .get_one::<bool>("build-dep")
            .map_or_else(|| false, bool::to_owned);

        // Return the result
        Ok(Self {
            language,
            build_dep,
        })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // Take and store the new language argument
        if let Some(language) = matches.get_one::<String>("lang") {
            language.clone_into(&mut self.language);
        }

        // Take and store the new build dependency argument
        if let Some(build_dep) = matches.get_one::<bool>("build-dep").copied() {
            self.build_dep = build_dep;
        }
        Ok(())
    }
}

impl Parser for Args {
    fn parse() -> Self {
        // Generate the command
        let mut root_cmd = Self::command();

        // Take the arguments
        root_cmd.build();

        // Get the argument matches
        let matches = root_cmd.get_matches();

        // Parse the arguments
        Self::from_arg_matches(&matches).expect("Failed to parse arguments")
    }
}

impl Args {
    // Return the requested language
    pub fn language(&self) -> &str {
        &self.language
    }
}
