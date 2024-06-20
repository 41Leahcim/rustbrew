use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    time::{Duration, SystemTime},
};

use clap::{Arg, Command};
use serde::Deserialize;

const CORE_FORMULAS_FILE: &str = "core_formulas.json";

#[derive(Debug)]
#[allow(dead_code)]
struct Args {
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
}

#[derive(Debug, Deserialize)]
struct Formula {
    name: String,
    build_dependencies: Vec<String>,
    dependencies: Vec<String>,
    test_dependencies: Vec<String>,
    recommended_dependencies: Vec<String>,
    opional_dependencies: Option<Vec<String>>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

pub fn is_file_old(file_path: &str) -> bool {
    let Ok(file) = File::open(file_path) else {
        return true;
    };
    let Ok(metadata) = file.metadata() else {
        return true;
    };
    let seven_days_ago = SystemTime::now() - Duration::from_secs(7 * 24 * 3600);
    metadata
        .modified()
        .map_or(true, |last_modification| last_modification < seven_days_ago)
}

fn get_core_formulas(file_name: &str) {
    let response = ureq::get("https://formulae.brew.sh/api/formula.json")
        .call()
        .expect("Can not reach API endpoint");
    let mut output = BufWriter::new(File::create(file_name).expect("Error creating file"));
    io::copy(&mut response.into_reader(), &mut output).expect("Error writing to a file");
    eprintln!("Successfully written JSON data into {file_name}");
}

fn get_formulas_from_file(
    file_name: &str,
    lang_name: &str,
) -> Result<impl Iterator<Item = String>, Error> {
    let reader = BufReader::new(File::open(file_name)?);
    let formulas: Vec<Formula> = serde_json::from_reader(reader)?;
    let lang_name = lang_name.to_owned();
    let lang_at = lang_name.to_owned() + "@";
    Ok(formulas
        .into_iter()
        .filter(move |formula| {
            formula
                .build_dependencies
                .iter()
                .chain(&formula.dependencies)
                .chain(&formula.test_dependencies)
                .chain(&formula.recommended_dependencies)
                .chain(formula.opional_dependencies.iter().flatten())
                .any(|dep| *dep == lang_name || dep.starts_with(&lang_at))
        })
        .map(|formula| formula.name))
}

pub fn get_package_count(file_name: &str, lang: &str) {
    assert!(
        lang.len() <= 30,
        "The language is more than 30 characters long! which is weird! : language={lang}\n"
    );

    if is_file_old(file_name) {
        get_core_formulas(file_name);
    }
    let formulas_list =
        get_formulas_from_file(file_name, lang).expect("Error getting formulas list");

    let pkg_count = formulas_list.count();
    println!("{pkg_count}");
}

fn main() {
    let args = Args::parse();
    get_package_count(CORE_FORMULAS_FILE, &args.language);
}
