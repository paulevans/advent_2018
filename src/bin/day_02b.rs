use std::io::{self, BufRead};

#[cfg(test)]
use std::io::Cursor;

/// Read all frequencies from standard in
fn read_box_ids<T: BufRead>(read_handle: &mut T, box_ids: &mut Vec<String>) {
    // Give large capacity for the line. 32 should be plenty.
    // Longest line in data is 26
    let mut input = String::with_capacity(32);

    // do something with line
    while read_handle
        .read_line(&mut input)
        .expect("Failed to read line")
        > 0
    {
        // Empty lines end the program
        if input.len() == 0 || input == "\n" {
            break;
        }

        box_ids.push(input.to_string());

        // Re-using buffer rather than allocating new string each read.
        input.clear();
    }
}

fn process_box_ids(box_ids: &Vec<String>) -> String {
    // So this algorithm stinks O (n^2) because of the nested processing of data
    // Might also do a lot of allocations?
    // ... but the input.txt only has 250 lines so... *shrugs*
    for (lhs_index, lhs_id) in box_ids.iter().enumerate() {
        // Using the for loop to give us a starting point for the next search
        // We don't need to compare anything that has been on the left side again
        for rhs_id in box_ids.iter().skip(lhs_index + 1) {
            if lhs_id
                .chars() // all the characters
                .zip(rhs_id.chars()) // iterates over rhs_id at the same time as lhs_id
                .filter(|(a, b)| a != b) // find the inequal characters
                .count()
                == 1
            {
                // if there is only one inequal character
                return lhs_id
                    .chars() // Get all the characters
                    .zip(rhs_id.chars()) // .. of both left and right
                    .filter(|(a, b)| a == b && a != &'\n') // Only pick ones that are equal
                    .map(|(a, _)| a) // and return them. Map here maintains order.
                    .collect(); // Collect joins the characters up again without the non-matching character
            }
        }
    }

    // So here is something Rust-y
    // Assuming the data is correct this point should never be reached
    // If it is, this macro causes a panic.
    // Without this macro the code will not compile.
    // We could handle via returning an Option Err instead.
    unreachable!()
}

/// Enter data to test via stdin for this day's exercise.
/// Assumes there is a solution, if not control + c
/// :P
fn main() -> io::Result<()> {
    println!("Day 02: https://adventofcode.com/2018/day/2");

    let stdin = io::stdin();

    // Locking once for all reading.
    let mut input_handle = stdin.lock();

    // As the sequence now has to be repeated, parse it once and store
    let mut box_ids = Vec::with_capacity(256);
    read_box_ids(&mut input_handle, &mut box_ids);
    let common = process_box_ids(&box_ids);
    println!("{} common", common);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day_02b_example_01() {
        let mut input_handle =
            Cursor::new("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n\n".as_bytes());
        let mut box_ids = Vec::with_capacity(256);
        read_box_ids(&mut input_handle, &mut box_ids);
        let box_id = process_box_ids(&box_ids);

        assert_eq!(box_ids.len(), 7);
        assert_eq!(box_id, "fgij");
    }
}
