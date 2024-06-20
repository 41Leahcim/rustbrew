use clap::{Arg, Command};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Args {
    language: String,
    build_dep: bool,
}

impl Args {
    pub fn parse() -> Self {
        let mut root_cmd = Command::new("rustbrew")
        .about("Count all programs written/built in X language or Y build system or Z library distributed via Homebrew.")
        .long_about("Count all programs written/built in X language or Y build system or Z library distributed via Homebrew. Get all build dependencies of all packages in Homebrew Core formulae")
        .args([
            Arg::new("lang")
                .short('l')
                .help("get count of all packages which have this language/build-system/library as a dependency (required)"),
            Arg::new("build-dep")
                .short('b')
                .help("show building dependencies for all packages in Homebrew Core")
        ]);

        root_cmd.build();
        let matches = root_cmd.get_matches();
        let language = matches.get_one::<String>("lang").map_or_else(||{
        eprintln!("No language nor build system nor library is specified. Counting packages built in Rust (by default):");
        "rust".to_owned()
    }, String::to_owned);
        let build_dep = matches
            .get_one::<bool>("build-dep")
            .map_or_else(|| false, bool::to_owned);
        Self {
            language,
            build_dep,
        }
    }

    pub fn language(&self) -> &str {
        &self.language
    }
}
