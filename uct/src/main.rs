use clap::Parser;

mod args;
mod units;
mod input;
mod regex;
mod noregex;

use args::Args;
use units::get_all_units;
use input::{ read_input, InputSource };
use regex::generate_output_regex;

fn main() {
    let args = Args::parse();

    // Determine input source
    let input_source = if let Some(file_path) = args.file.clone() {
        InputSource::File(file_path)
    } else if let Some(input_string) = args.input.clone() {
        InputSource::Direct(input_string)
    } else {
        InputSource::Stdin
    };

    // Read input
    let input = match read_input(input_source) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
            std::process::exit(1);
        }
    };

    // Get valid unit types
    let all_units: std::collections::HashMap<&str, units::Unit> = get_all_units();

    // Set output unit
    let to_unit = args.to_unit.clone();
    let output_unit = all_units.get(to_unit.as_str()).unwrap_or_else(|| {
        eprintln!("Internal Error: Cannot find unit '{}'.", to_unit);
        std::process::exit(1);
    });

    let output = generate_output_regex(&input, &to_unit, &output_unit, &all_units, &args);

    print!("{}", output);
}
