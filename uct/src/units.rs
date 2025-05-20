// src/units.rs

use std::collections::HashMap;

const BYTE: Unit = Unit::new_const("Byte", UnitCategory::SIByte, 0);
const KILOBYTE: Unit = Unit::new_const("Kilobyte", UnitCategory::SIByte, 3);
const MEGABYTE: Unit = Unit::new_const("Megabyte", UnitCategory::SIByte, 6);
const GIGABYTE: Unit = Unit::new_const("Gigabyte", UnitCategory::SIByte, 9);
const TERABYTE: Unit = Unit::new_const("Terabyte", UnitCategory::SIByte, 12);
const PETABYTE: Unit = Unit::new_const("Petabyte", UnitCategory::SIByte, 15);
const EXABYTE: Unit = Unit::new_const("Exabyte", UnitCategory::SIByte, 18);

const KIBIBYTE: Unit = Unit::new_const("Kibibyte", UnitCategory::BinaryByte, 10);
const MEBIBYTE: Unit = Unit::new_const("Mebibyte", UnitCategory::BinaryByte, 20);
const GIBIBYTE: Unit = Unit::new_const("Gibibyte", UnitCategory::BinaryByte, 30);
const TEBIBYTE: Unit = Unit::new_const("Tebibyte", UnitCategory::BinaryByte, 40);
const PEBIBYTE: Unit = Unit::new_const("Pebibyte", UnitCategory::BinaryByte, 50);
const EXBIBYTE: Unit = Unit::new_const("Exbibyte", UnitCategory::BinaryByte, 60);

const BIT: Unit = Unit::new_const("bit", UnitCategory::DataRate, 0);
const KILOBIT: Unit = Unit::new_const("Kilobit", UnitCategory::DataRate, 3);
const MEGABIT: Unit = Unit::new_const("Megabit", UnitCategory::DataRate, 6);
const GIGABIT: Unit = Unit::new_const("Gigabit", UnitCategory::DataRate, 9);
const TERABIT: Unit = Unit::new_const("Terabit", UnitCategory::DataRate, 12);
const PETABIT: Unit = Unit::new_const("Petabit", UnitCategory::DataRate, 15);
const EXABIT: Unit = Unit::new_const("Exabit", UnitCategory::DataRate, 18);

/// Represents the category of a unit.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnitCategory {
    SIByte, // For SI Byte units (e.g., KB, MB)
    BinaryByte, // For Binary Byte units (e.g., KiB, MiB)
    DataRate, // For Data Rate units (e.g., Kb, Mb)
}

/// Represents a unit of measurement.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Unit {
    pub name: &'static str,
    pub category: UnitCategory,
    pub multiplier: f64, // Multiplier to bits
}

impl Unit {
    /// Creates a new `Unit` with a dynamically calculated multiplier.
    ///
    /// - `name`: The unit's full name (e.g., "Kilobit").
    /// - `category`: The category of the unit.
    /// - `power`: The exponent associated with the unit's prefix.
    ///
    /// The multiplier is calculated as follows:
    /// - **SIByte**: 10^power * 8 (to convert bytes to bits)
    /// - **BinaryByte**: 2^power * 8 (to convert bytes to bits)
    /// - **DataRate**: 10^power (already in bits)
    pub fn new(name: &'static str, category: UnitCategory, power: u32) -> Unit {
        let multiplier = match category {
            UnitCategory::SIByte => (10_f64).powi(power as i32) * 8.0, // SI Byte units: 10^power * 8
            UnitCategory::BinaryByte => (2_f64).powi(power as i32) * 8.0, // Binary Byte units: 2^power * 8
            UnitCategory::DataRate => (10_f64).powi(power as i32), // Data Rate units: 10^power
        };
        Unit {
            name,
            category,
            multiplier,
        }
    }

    pub const fn new_const(name: &'static str, category: UnitCategory, power: u32) -> Unit {
        let multiplier = match category {
            UnitCategory::SIByte => ((10_u64).pow(power) as f64) * 8.0, // SI Byte units: 10^power * 8
            UnitCategory::BinaryByte => ((2_u64).pow(power) as f64) * 8.0, // Binary Byte units: 2^power * 8
            UnitCategory::DataRate => (10_u64).pow(power) as f64, // Data Rate units: 10^power
        };
        Unit {
            name,
            category,
            multiplier,
        }
    }
}

/// Returns a HashMap containing all SI Byte units.
pub fn get_si_byte_units() -> HashMap<&'static str, Unit> {
    [
        ("B", BYTE),
        ("KB", KILOBYTE),
        ("MB", MEGABYTE),
        ("GB", GIGABYTE),
        ("TB", TERABYTE),
        ("PB", PETABYTE),
        ("EB", EXABYTE),
        ("K", KILOBYTE),
        ("M", MEGABYTE),
        ("G", GIGABYTE),
        ("T", TERABYTE),
        ("P", PETABYTE),
        ("E", EXABYTE),
    ]
        .iter()
        .cloned()
        .collect()
}

/// Returns a HashMap containing all Binary Byte units.
pub fn get_binary_byte_units() -> HashMap<&'static str, Unit> {
    [
        ("KiB", KIBIBYTE),
        ("MiB", MEBIBYTE),
        ("GiB", GIBIBYTE),
        ("TiB", TEBIBYTE),
        ("PiB", PEBIBYTE),
        ("EiB", EXBIBYTE),
        ("Ki", KIBIBYTE),
        ("Mi", MEBIBYTE),
        ("Gi", GIBIBYTE),
        ("Ti", TEBIBYTE),
        ("Pi", PEBIBYTE),
        ("Ei", EXBIBYTE),
    ]
        .iter()
        .cloned()
        .collect()
}

/// Returns a HashMap containing all Data Rate units.
pub fn get_data_rate_units() -> HashMap<&'static str, Unit> {
    [
        ("b", BIT),
        ("bit", BIT),
        ("Kb", KILOBIT),
        ("Mb", MEGABIT),
        ("Gb", GIGABIT),
        ("Tb", TERABIT),
        ("Pb", PETABIT),
        ("Eb", EXABIT),
        ("Kbit", KILOBIT),
        ("Mbit", MEGABIT),
        ("Gbit", GIGABIT),
        ("Tbit", TERABIT),
        ("Pbit", PETABIT),
        ("Ebit", EXABIT),
    ]
        .iter()
        .cloned()
        .collect()
}

/// Returns a combined HashMap containing all SI Byte, Binary Byte, and Data Rate units.
pub fn get_all_units() -> HashMap<&'static str, Unit> {
    let mut all_units = HashMap::new();

    // Insert SI Byte Units
    all_units.extend(get_si_byte_units());

    // Insert Binary Byte Units
    all_units.extend(get_binary_byte_units());

    // Insert Data Rate Units
    all_units.extend(get_data_rate_units());

    all_units
}

pub fn get_unit(symbol: &str) -> Option<&Unit> {
    let mut chars = symbol.chars();

    match chars.next() {
        Some('b') => Some(&BIT),
        Some('B') => Some(&BYTE),
        Some('K') => {
            match chars.next() {
                Some('i') => Some(&KIBIBYTE),
                Some('b') => Some(&KILOBIT),
                _ => Some(&KILOBYTE),
            }
        }
        Some('M') => {
            match chars.next() {
                Some('i') => Some(&MEBIBYTE),
                Some('b') => Some(&MEGABIT),
                _ => Some(&MEGABYTE),
            }
        }
        Some('G') => {
            match chars.next() {
                Some('i') => Some(&GIBIBYTE),
                Some('b') => Some(&GIGABIT),
                _ => Some(&GIGABYTE),
            }
        }
        Some('T') => {
            match chars.next() {
                Some('i') => Some(&TEBIBYTE),
                Some('b') => Some(&TERABIT),
                _ => Some(&TERABYTE),
            }
        }
        Some('P') => {
            match chars.next() {
                Some('i') => Some(&PEBIBYTE),
                Some('b') => Some(&PETABIT),
                _ => Some(&PETABYTE),
            }
        }
        Some('E') => {
            match chars.next() {
                Some('i') => Some(&EXBIBYTE),
                Some('b') => Some(&EXABIT),
                _ => Some(&EXABYTE),
            }
        }
        _ => None,
    }
}
