use std::collections::HashMap;

use crate::units::{ self, Unit, get_unit };

pub fn generate_output_noregex(
    input: &str,
    to_unit: &str,
    output_unit: &Unit,
    all_units: &HashMap<&str, units::Unit>,
    decimal_separator: char,
    decimals: usize,
    color: bool
) -> String {
    // Build regular expression from unit names
    let mut unit_names: Vec<&str> = all_units.keys().into_iter().copied().collect();
    unit_names.sort();

    let mut number_start: usize = 0;
    let mut search_for_number = true;

    let mut numbers: Vec<(usize, usize)> = Vec::new();

    for (i, c) in input.char_indices().rev() {
        match (search_for_number, c.is_ascii_digit()) {
            (true, true) => {
                number_start = i + 1;
                search_for_number = false;
            }
            (false, false) => {
                numbers.push((i + 1, number_start));
                search_for_number = true;
            }
            _ => {}
        }
    }

    // add number starting at 0 if exist
    if !search_for_number {
        numbers.push((0, number_start));
    }

    let mut i: usize = 0;
    let mut output = String::with_capacity(input.len());

    for (num_s, num_e) in numbers.into_iter().rev() {
        let num = input[num_s..num_e].parse::<f64>().unwrap_or_else(|e| {
            eprintln!("Unable to parse number: {}", e);
            std::process::exit(1);
        });

        let search_end = std::cmp::min(num_e + 2, input.len());
        let from_unit: Option<&Unit> = get_unit(&input[num_e..search_end]);

        if let Some(unit) = from_unit {
            let multi = unit.multiplier / output_unit.multiplier;
            let bits = num * multi;
            let formatted_bits = format!("{:.decimals$}", bits)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();

            output.push_str(&input[i..num_s]);
            output.push_str(&formatted_bits);
            output.push_str(&to_unit);
            i = search_end;
        }
    }
    // Add the rest of the input
    output.push_str(&input[i..]);
    output
}
