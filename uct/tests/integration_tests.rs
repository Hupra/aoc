// tests/integration_tests.rs

use assert_cmd::Command;
use predicates::prelude::*;
use assert_fs::prelude::*;

const TEST_BITS: [&str; 7] = ["b", "Eb", "Gb", "Kb", "Mb", "Pb", "Tb"];
const TEST_BITS_LONG: [&str; 7] = ["bit", "Ebit", "Gbit", "Kbit", "Mbit", "Pbit", "Tbit"];

const TEST_SI_BYTES: [&str; 7] = ["B", "E", "G", "K", "M", "P", "T"];
const TEST_SI_BYTES_LONG: [&str; 7] = ["B", "EB", "GB", "KB", "MB", "PB", "TB"];

const TEST_BI_BYTES: [&str; 6] = ["Ei", "Gi", "Ki", "Mi", "Pi", "Ti"];
const TEST_BI_BYTES_LONG: [&str; 6] = ["EiB", "GiB", "KiB", "MiB", "PiB", "TiB"];

#[test]
fn test_bits_to_bytes() {
    let input_data = "80b";
    Command::cargo_bin("uct")
        .unwrap()
        .args(&["B"])
        .write_stdin(input_data)
        .assert()
        .success()
        .stdout(predicate::eq("10B"));
}

#[test]
fn test_bits_to_bytes_with_input() {
    Command::cargo_bin("uct")
        .unwrap()
        .args(&["-i", "80b", "B"])
        .assert()
        .success()
        .stdout(predicate::eq("10B"));
}

#[test]
fn test_bits_to_bytes_with_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("input.txt");
    file.write_str("80b").unwrap();

    Command::cargo_bin("uct")
        .unwrap()
        .args(&["--file", file.path().to_str().unwrap(), "B"])
        .assert()
        .success()
        .stdout(predicate::eq("10B"));

    temp.close().unwrap();
}

#[test]
fn test_help() {
    let input_data = "123b";
    Command::cargo_bin("uct")
        .unwrap()
        .args(&["B", "--help"])
        .write_stdin(input_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("-h, --help"));
}

#[test]
fn test_bits_to_binary_byte_units() {
    let input_data = "8796093022208b";
    Command::cargo_bin("uct")
        .unwrap()
        .args(&["b"])
        .write_stdin(input_data)
        .assert()
        .success()
        .stdout(predicate::eq(input_data));

    Command::cargo_bin("uct")
        .unwrap()
        .args(&["MiB"])
        .write_stdin(input_data)
        .assert()
        .success()
        .stdout(predicate::eq("1048576MiB"));

    Command::cargo_bin("uct")
        .unwrap()
        .args(&["GiB"])
        .write_stdin(input_data)
        .assert()
        .success()
        .stdout(predicate::eq("1024GiB"));

    Command::cargo_bin("uct")
        .unwrap()
        .args(&["Ti"])
        .write_stdin(input_data)
        .assert()
        .success()
        .stdout(predicate::eq("1Ti"));
}

#[test]
fn test_same_units_is_same_bits() {
    let input_data: i64 = 1_000_000_000_000_000_000;
    for i in 0..TEST_BITS.len() {
        let input_1 = format!("{}{}", input_data, TEST_BITS[i]);
        let input_2 = format!("{}{}", input_data, TEST_BITS_LONG[i]);
        let inputs = [input_1, input_2];

        let res = inputs.map(|input| {
            String::from_utf8(
                Command::cargo_bin("uct")
                    .unwrap()
                    .args(&["b"])
                    .write_stdin(input)
                    .output()
                    .ok()
                    .unwrap()
                    .stdout.to_vec()
            ).unwrap()
        });
        assert_eq!(res[0], res[1]);
    }
}

#[test]
fn test_same_units_is_same_si_bytes() {
    let input_data: i64 = 1_000_000_000_000_000_000;
    for i in 0..TEST_SI_BYTES.len() {
        let input_1 = format!("{}{}", input_data, TEST_SI_BYTES[i]);
        let input_2 = format!("{}{}", input_data, TEST_SI_BYTES_LONG[i]);
        let inputs = [input_1, input_2];

        let res = inputs.map(|input| {
            String::from_utf8(
                Command::cargo_bin("uct")
                    .unwrap()
                    .args(&["b"])
                    .write_stdin(input)
                    .output()
                    .ok()
                    .unwrap()
                    .stdout.to_vec()
            ).unwrap()
        });
        assert_eq!(res[0], res[1]);
    }
}

#[test]
fn test_same_units_is_same_bi_bytes() {
    let input_data: i64 = 1_000_000_000_000_000_000;
    for i in 0..TEST_BI_BYTES.len() {
        let input_1 = format!("{}{}", input_data, TEST_BI_BYTES[i]);
        let input_2 = format!("{}{}", input_data, TEST_BI_BYTES_LONG[i]);
        let inputs = [input_1, input_2];

        let res = inputs.map(|input| {
            String::from_utf8(
                Command::cargo_bin("uct")
                    .unwrap()
                    .args(&["b"])
                    .write_stdin(input)
                    .output()
                    .ok()
                    .unwrap()
                    .stdout.to_vec()
            ).unwrap()
        });
        assert_eq!(res[0], res[1]);
    }
}

#[test]
fn test_all_units_unique() {
    let combined_units = TEST_BITS.iter()
        .chain(TEST_SI_BYTES.iter())
        .chain(TEST_BI_BYTES.iter())
        .collect::<Vec<_>>();

    let input_data = 1_000_000_000;
    for i in 0..combined_units.len() {
        let input_1 = format!("{}{}", input_data, combined_units[i]);
        for j in 0..combined_units.len() {
            if i == j {
                continue;
            }
            let input_2 = format!("{}{}", input_data, combined_units[j]);
            let inputs = [input_1.clone(), input_2];

            let res = inputs.map(|input| {
                String::from_utf8(
                    Command::cargo_bin("uct")
                        .unwrap()
                        .args(&["b"])
                        .write_stdin(input)
                        .output()
                        .ok()
                        .unwrap()
                        .stdout.to_vec()
                ).unwrap()
            });
            assert_ne!(res[0], res[1]);
        }
    }
}
