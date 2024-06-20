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
    clippy::print_stdout,
    clippy::missing_trait_methods,
    clippy::use_debug
)]

use args::Args;
use clap::Parser as _;
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

/// Checks whether the file should be downloaded again.
/// This is the case, if the file doesn't exist, it's metadata can't be read, or it is more than a weak old.
fn is_file_old(file_path: &str) -> bool {
    // Open the file for read access
    let Ok(file) = File::open(file_path) else {
        return true;
    };

    // Read the metadata of the file
    let Ok(metadata) = file.metadata() else {
        return true;
    };

    // Calculate the SystemTime of a week ago.
    let seven_days_ago = SystemTime::now() - Duration::from_secs(7 * 24 * 3600);

    // Return whether the file was modified in the last week
    metadata
        .modified()
        .map_or(true, |last_modification| last_modification < seven_days_ago)
}

/// Downloads the current formulas from brew.
///
/// # Panics
/// If the formula file couldn't be downloaded from <https://formulae.brew.sh/api/formula.json>
fn get_core_formulas(file_name: &str) {
    // Download the formula's
    let response = ureq::get("https://formulae.brew.sh/api/formula.json")
        .call()
        .expect("Can not reach API endpoint");

    // Open or create a file to store the the formulas
    let mut output = BufWriter::new(File::create(file_name).expect("Error creating file"));

    // Write the formula's to the file
    io::copy(&mut response.into_reader(), &mut output).expect("Error writing to a file");

    // Print a message to the screen to tell the user, the data has been downloaded successfully.
    println!("Successfully written JSON data into {file_name}");
}

/// Returns an iterator over formula's using `lang_name` as dependency.
///
/// # Errors
/// If the file couldn't be read or converted to a Vec of formulas.
fn get_formulas_from_file(
    file_name: &str,
    lang_name: &str,
) -> Result<impl Iterator<Item = String>, Error> {
    // Open the file
    let reader = BufReader::new(File::open(file_name)?);

    // Convert the content to JSON
    let formulas: Vec<Formula> = serde_json::from_reader(reader)?;

    // Turn the lang_name into an owned String
    let lang_name = lang_name.to_owned();

    // Add a '@' to the lang_name for specific versions
    let lang_at = format!("{lang_name}@");

    // Return an iterator returning the names of formula's with `lang_name` as dependency.
    Ok(formulas
        // Turn the vector of formula's into an iterator
        .into_iter()
        // Only keep the formula's with `lang_name` as dependency
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
        // Only return their name
        .map(Formula::take_name))
}

/// Updates the formula file if needed, and prints the number of formulas using `lang` as a dependency.
///
/// # Panics
/// Will panic if the length of the name of the programming language is at least 30 characters long or the formulas file couldn't be read.
fn get_package_count(file_name: &str, lang: &str) {
    // Make sure the name of the language is at most 29 characters long.
    assert!(
        lang.len() <= 30,
        "The language is more than 30 characters long! which is weird! : language={lang}\n"
    );

    // Redownload the formula file if needed
    if is_file_old(file_name) {
        get_core_formulas(file_name);
    }

    // Create an iterator over the formulas using `lang` as a dependency
    let formulas_list =
        get_formulas_from_file(file_name, lang).expect("Error getting formulas list");

    // Count the formula's returned by the iterator
    let pkg_count = formulas_list.count();

    // Display the final count
    println!("{pkg_count}");
}

fn get_all_build_dep(file_name: &str) -> Result<(), Error> {
    // Open the file
    let reader = BufReader::new(File::open(file_name)?);

    // Read the formulas
    let formulas: Vec<Formula> = serde_json::from_reader(reader)?;

    // Add the unique build dependencies
    let build_deps = formulas.iter().flat_map(Formula::build_dependencies).fold(
        Vec::new(),
        |mut dependencies, dependency| {
            if !dependencies.contains(dependency) {
                dependencies.push(dependency.to_owned());
            }
            dependencies
        },
    );

    // Print the number and names of build dependencies
    println!(
        "All Build Dependencies Count: {}\n{build_deps:?}",
        build_deps.len()
    );
    Ok(())
}

fn main() {
    // Parse arguments passed by the user
    let args = Args::parse();

    // Download and display the packages using the passed language as a dependency
    get_package_count(CORE_FORMULAS_FILE, args.language());

    if args.build_dep() {
        get_all_build_dep(CORE_FORMULAS_FILE).expect("Failed to read build dependencies");
    }
}
