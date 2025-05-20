use clap::{ ArgAction, Parser };

use crate::units::{ get_all_units, get_binary_byte_units, get_data_rate_units, get_si_byte_units };

/// Unit Conversion Tool
#[derive(Parser, Debug)]
#[command(name = "uct")]
#[command(
    about = "A Unit Conversion Tool for converting between various units.",
    long_about = "\
A Unit Conversion Tool for converting between various units.

\x1b[4m\x1b[1mExamples:\x1b[0m
  Pipe input from echo:
  $ echo '1024Gi' | uct Ti

  Use with a file:
  $ uct MiB -f input.txt

  Use with an explicit input string:
  $ uct GB -i '100GiB'"
)]
pub struct Args {
    /// Target unit (e.g., KB, MiB) - **Positional Argument**
    #[arg(
        index = 1,
        value_parser = get_all_units().keys().collect::<Vec<_>>(),
        hide_possible_values = true,
        help = format_help_message(),
        default_value_t = String::from("GB")
    )]
    pub to_unit: String,

    /// Take input from file instead of stdin
    #[arg(short, long)]
    pub file: Option<String>,

    /// Take input from explicit string instead of stdin
    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long, default_value_t = 3)]
    pub decimals: usize,

    #[arg(
        short = 's',
        long,
        aliases = ["ds"],
        default_value_t = String::from("."),
        value_parser = [".", ","]
    )]
    pub decimal_separator: String,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub color: bool,

    #[arg(short = 'n', long = "no-suffix", action = ArgAction::SetFalse)]
    pub suffix: bool,
}

fn format_help_message() -> String {
    let mut si_bytes = get_si_byte_units().keys().copied().collect::<Vec<_>>();
    let mut bi_bytes = get_binary_byte_units().keys().copied().collect::<Vec<_>>();
    let mut si_bits = get_data_rate_units().keys().copied().collect::<Vec<_>>();

    let sorter = |a: &&str, b: &&str| {
        let len_cmp = a.len().cmp(&b.len());
        if len_cmp == std::cmp::Ordering::Equal {
            a.cmp(b)
        } else {
            len_cmp
        }
    };
    si_bytes.sort_by(sorter);
    bi_bytes.sort_by(sorter);
    si_bits.sort_by(sorter);

    format!(
        "Target unit for conversion. Possible values include:\n\
         \x1b[1mSI Bytes\x1b[0m: [{}]\n\
         \x1b[1mBI Bytes\x1b[0m: [{}]\n\
         \x1b[1mSI Bits\x1b[0m:  [{}]",
        si_bytes.join(", "),
        bi_bytes.join(", "),
        si_bits.join(", ")
    )
}
