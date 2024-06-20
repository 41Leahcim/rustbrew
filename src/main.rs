#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::single_call_fn,
    clippy::print_stderr,
    clippy::implicit_return,
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::question_mark_used,
    clippy::shadow_reuse,
    clippy::print_stdout
)]

use args::Args;
use core::time::Duration;
use error::Error;
use formula::Formula;
use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    time::SystemTime,
};

mod args;
mod error;
mod formula;

const CORE_FORMULAS_FILE: &str = "core_formulas.json";

fn is_file_old(file_path: &str) -> bool {
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

/// # Panics
/// If the formula file couldn't be downloaded from <https://formulae.brew.sh/api/formula.json>
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
    let lang_at = format!("{lang_name}@");
    Ok(formulas
        .into_iter()
        .filter(move |formula| {
            formula
                .build_dependencies()
                .iter()
                .chain(formula.dependencies())
                .chain(formula.test_dependencies())
                .chain(formula.recommended_dependencies())
                .chain(
                    formula
                        .optional_dependencies()
                        .iter()
                        .flat_map(|deps| deps.iter()),
                )
                .any(|dep| *dep == lang_name || dep.starts_with(&lang_at))
        })
        .map(Formula::take_name))
}

/// # Panics
/// Will panic if the length of the name of the programming language is at least 30 characters long or the formulas file couldn't be read.
fn get_package_count(file_name: &str, lang: &str) {
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
    get_package_count(CORE_FORMULAS_FILE, args.language());
}
