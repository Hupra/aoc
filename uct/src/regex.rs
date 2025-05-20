use std::collections::HashMap;
use regex::Regex;

use crate::{ args::Args, units::{ self, Unit } };

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn generate_output_regex(
    input: &str,
    to_unit: &str,
    output_unit: &Unit,
    all_units: &HashMap<&str, units::Unit>,
    args: &Args
) -> String {
    // Build regular expression from unit names

    // can be joined on | to make simple regex!
    let mut unit_names: Vec<&str> = all_units.keys().into_iter().copied().collect();
    unit_names.sort_by(|&a, &b| b.cmp(&a));
    let r_unit = format!("({})", unit_names.join("|"));

    let r_number = format!("(\\d+(?:\\{}\\d+)?)", args.decimal_separator);
    // let r_unit = "([KMGTPE](?:bit|i?B?)|b(?:it)?|B)";
    let r = format!("{}{}", r_number, r_unit);
    let re = Regex::new(&r).unwrap();

    // The classic swap the input process
    let mut i: usize = 0;
    let mut output = String::with_capacity(input.len());

    for cap in re.captures_iter(&input) {
        if
            let (Some(match_res), Some(match_num), Some(match_str)) = (
                cap.get(0),
                cap.get(1),
                cap.get(2),
            )
        {
            let num: f64 = match_num
                .as_str()
                .parse()
                .unwrap_or_else(|e| {
                    eprintln!("Unable to parse number: {}", e);
                    std::process::exit(1);
                });

            let unit: &Unit = all_units.get(match_str.as_str()).unwrap_or_else(|| {
                eprintln!("Unable to parse unit");
                std::process::exit(1);
            });

            let conversion_factor = unit.multiplier / output_unit.multiplier;
            let new_number = num * conversion_factor;
            let formatted_number = format!("{:.d$}", new_number, d = args.decimals);
            let formatted_number = formatted_number.trim_end_matches('0').trim_end_matches('.');

            if args.color {
                output.push_str(RED);
                output.push_str(&input[i..match_res.start()]);
                output.push_str(&formatted_number);
                output.push_str(GREEN);
                if args.suffix {
                    output.push_str(&to_unit);
                }
                output.push_str(RESET);
            } else {
                output.push_str(&input[i..match_res.start()]);
                output.push_str(&formatted_number);
                if args.suffix {
                    output.push_str(&to_unit);
                }
            }

            i = match_res.end();
        } else {
            eprintln!("Regex capture invalid");
            std::process::exit(1);
        }
    }

    // Add the rest of the input
    output.push_str(&input[i..]);
    output
}
