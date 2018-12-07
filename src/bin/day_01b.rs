//use std::error::Error;
use std::collections::HashSet;
use std::fmt;
use std::io::{self, BufRead, Write};

#[cfg(test)]
use std::io::{Cursor, SeekFrom};

#[cfg(any(unix))]
const EOF_CHARACTER_COUNT: usize = 1;

#[cfg(any(windows))]
const EOF_CHARACTER_COUNT: usize = 2;

#[derive(Debug)]
pub struct FrequencyNotRepeated;

/// Not found error state for when frequency is not repeated in an iteration
impl fmt::Display for FrequencyNotRepeated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frequency not repeated in sequence")
    }
}

/// Read all frequencies from standard in
fn read_frequency_deltas<T: BufRead>(handle: &mut T, frequency_deltas: &mut Vec<i32>) {
    // Reusable lock to write to stdout via writelin - perf gains over println macro
    let stdout = io::stdout();
    let mut output_handle = stdout.lock();

    // Give large capacity for the line. 32 should be plenty.
    let mut input = String::with_capacity(32);

    // do something with line
    while handle.read_line(&mut input).expect("Failed to read line") > 0 {
        // Empty lines end the program
        if input.len() == 0 || input == "\n" {
            break;
        }

        // Read line includes line endings - just parse the content
        // Using slice that this shouldn't create an allocation ... ?
        match input[0..input.len() - EOF_CHARACTER_COUNT].parse::<i32>() {
            // Parsing succeded - process frequency
            Ok(frequency_delta) => {
                frequency_deltas.push(frequency_delta);
            }
            // Parsing failed, report error but continue processing stream
            Err(e) => {
                writeln!(output_handle, "ERROR: Input not integer. {}", e).unwrap();
            }
        }

        // Re-using buffer rather than allocating new string each read.
        input.clear();
    }
}

/// Keep applying deltas until a repeat frequency is found
fn process_frequency_deltas(
    frequency_deltas: &Vec<i32>,
    seen_frequencies: &mut HashSet<i32>,
) -> i32 {
    let mut current_frequency: i32 = 0;

    loop {
        for delta in frequency_deltas {
            current_frequency = current_frequency + delta;
            if seen_frequencies.contains(&current_frequency) {
                return current_frequency;
            }

            seen_frequencies.insert(current_frequency);
        }
    }
}

/// Original solution to day 1 part 2
/// ... too hacky though.
/// Would require reading file over and over again as designed.
/// Okay for small values, but meh the actual input took way too long because
/// * Reading files
/// * Uses vector instead of map - and never removed dupes
pub fn day_01b_old<T: BufRead>(
    handle: &mut T,
    frequency_values: &mut Vec<i32>,
) -> Result<i32, FrequencyNotRepeated> {
    // Reusable lock to write to stdout via writelin - perf gains over println macro
    let stdout = io::stdout();
    let mut output_handle = stdout.lock();

    // Allow user to stipulate start frequency rather than assuming zero.AsMut
    let mut current_frequency: i32 = *frequency_values.last().unwrap();

    // Give large capacity for the line. 32 should be plenty.
    let mut input = String::with_capacity(32);

    // do something with line
    while handle.read_line(&mut input).expect("Failed to read line") > 0 {
        // Empty lines end the program
        if input.len() == 0 || input == "\n" {
            break;
        }

        // Read line includes line endings - just parse the content
        // Using slice that this shouldn't create an allocation ... ?
        match input[0..input.len() - EOF_CHARACTER_COUNT].parse::<i32>() {
            // Parsing succeded - process frequency
            Ok(frequency_delta) => {
                current_frequency += frequency_delta;

                writeln!(
                    output_handle,
                    "  - Current frequency {}, change of {:+}; resulting frequency {}.",
                    frequency_values[frequency_values.len() - 1],
                    frequency_delta,
                    current_frequency
                ).unwrap();

                if frequency_values.contains(&current_frequency) {
                    return Ok(current_frequency);
                }

                frequency_values.push(current_frequency);
            }
            // Parsing failed, report error but continue processing stream
            Err(e) => {
                writeln!(output_handle, "ERROR: Input not integer. {}", e).unwrap();
            }
        }

        // Re-using buffer rather than allocating new string each read.
        input.clear();
    }

    Err(FrequencyNotRepeated {})
}

/// Enter data to test via stdin for this day's exercise.
/// Assumes there is a solution, if not control + c
/// :P
fn main() -> io::Result<()> {
    println!("Day 01: https://adventofcode.com/2018/day/1");

    let stdin = io::stdin();

    // Locking once for all reading.
    let mut input_handle = stdin.lock();

    // As the sequence now has to be repeated, parse it once and store
    let mut frequency_deltas = Vec::with_capacity(1038);
    read_frequency_deltas(&mut input_handle, &mut frequency_deltas);

    // HashSet looks like the best choice. Do not need any mapping after all.
    // The capacity I got from running the data through already.
    // May as well give it a headstart rather it having too little then
    // having the penality for resizing.
    // 146478 * 4 bytes (for 32 bit ints) * 2 is still just over 1 megabyte
    let mut seen_frequencies = HashSet::with_capacity(146478 * 2);

    // This says 0 has been seen before
    seen_frequencies.insert(0);

    // Look for that repeated frequency
    let repeated_frequency = process_frequency_deltas(&frequency_deltas, &mut seen_frequencies);

    // Not following the output standard as strictly as part 1
    // ... but as that's not really part of the test... meh
    println!("{} has already been seen", repeated_frequency);
    println!("Total unique frequencies found {}", seen_frequencies.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::io::Seek;

    #[test]
    fn test_day_01b_example_01() {
        // +1, -1 first reaches 0 twice.
        let mut input = Cursor::new("1\n-1\n\n".as_bytes());
        let mut seen_frequencies = vec![0];

        let frequency = day_01b_old(&mut input, &mut seen_frequencies);
        assert_eq!(frequency.ok(), Some(0));
    }

    #[test]
    fn test_day_01b_example_02() {
        // +3, +3, +4, -2, -4 first reaches 10 twice.
        // It requires the list to be read twice
        let mut input = Cursor::new("3\n3\n4\n-2\n-4\n\n".as_bytes());
        let mut seen_frequencies = vec![0];

        day_01b_old(&mut input, &mut seen_frequencies).err();
        assert_eq!(input.seek(SeekFrom::Start(0)).ok(), Some(0));

        let frequency = day_01b_old(&mut input, &mut seen_frequencies);
        assert_eq!(frequency.ok(), Some(10));
    }

    #[test]
    fn test_day_01b_example_03() {
        // -6, +3, +8, +5, -6 first reaches 5 twice.
        // It requires the list to be read three times
        let mut input = Cursor::new("-6\n3\n8\n5\n-6\n\n".as_bytes());
        let mut seen_frequencies = vec![0];

        day_01b_old(&mut input, &mut seen_frequencies).err();
        assert_eq!(input.seek(SeekFrom::Start(0)).ok(), Some(0));

        day_01b_old(&mut input, &mut seen_frequencies).err();
        assert_eq!(input.seek(SeekFrom::Start(0)).ok(), Some(0));

        let frequency = day_01b_old(&mut input, &mut seen_frequencies);
        assert_eq!(frequency.ok(), Some(5));
    }

    #[test]
    fn test_day_01b_example_04() {
        // +7, +7, -2, -7, -4 first reaches 14 twice.
        let mut input = Cursor::new("7\n7\n-2\n-7\n-4\n\n".as_bytes());
        let mut seen_frequencies = vec![0];

        let mut frequency = day_01b_old(&mut input, &mut seen_frequencies);
        let mut n = 0;

        while frequency.is_err() {
            assert_eq!(input.seek(SeekFrom::Start(0)).ok(), Some(0));
            frequency = day_01b_old(&mut input, &mut seen_frequencies);
            n = n + 1;
        }

        assert_eq!(n, 2);
        assert_eq!(frequency.ok(), Some(14));
    }
}
