use std::io::{self, BufRead, Write};

#[cfg(test)]
use std::io::BufReader;

#[cfg(any(unix))]
const EOF_CHARACTER_COUNT: usize = 1;

#[cfg(any(windows))]
const EOF_CHARACTER_COUNT: usize = 2;

/// Solution to day 1 part 1
pub fn day_01a<T: BufRead>(mut handle: T, start_frequency: i32) -> i32 {
    // Reusable lock to write to stdout via writelin - perf gains over println macro
    let stdout = io::stdout();
    let mut output_handle = stdout.lock();

    // Allow user to stipulate start frequency rather than assuming zero.AsMut
    let mut current_frequency = start_frequency;

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
                let previous_frequency = current_frequency;
                current_frequency += frequency_delta;

                writeln!(
                    output_handle,
                    "  - Current frequency {}, change of {:+}; resulting frequency {}.",
                    previous_frequency, frequency_delta, current_frequency
                );
            }
            // Parsing failed, report error but continue processing stream
            Err(e) => {
                writeln!(output_handle, "ERROR: Input not integer. {}", e);
            }
        }

        // Re-using buffer rather than allocating new string each read.
        input.clear();
    }

    current_frequency
}

/// Enter data to test via stdin for this day's exercise.
fn main() -> io::Result<()> {
    println!("Day 01: https://adventofcode.com/2018/day/1");

    let stdin = io::stdin();

    // Locking once for all reading.
    let input_handle = stdin.lock();

    day_01a(input_handle, 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day_01_example_01() {
        // +1, +1, +1 results in 3
        let input = BufReader::new("1\n1\n1\n\n".as_bytes());
        let frequency = day_01a(input, 0);
        assert_eq!(frequency, 3);
    }

    #[test]
    fn test_day_01_example_02() {
        // +1, +1, -2 results in 0
        let input = BufReader::new("1\n1\n-2\n\n".as_bytes());
        let frequency = day_01a(input, 0);
        assert_eq!(frequency, 0);
    }

    #[test]
    fn test_day_01_example_03() {
        // -1, -2, -3 results in -6
        let input = BufReader::new("-1\n-2\n-3\n\n".as_bytes());
        let frequency = day_01a(input, 0);
        assert_eq!(frequency, -6);
    }
}
