//use std::error::Error;
//use std::collections::HashSet;
//use std::collections::hash_map::DefaultHasher;
//use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
//use std::io::{self, BufRead, Read};
use std::io::{self, Read};

// #[cfg(test)]

const GRAM_RANGE: usize = (1 + GRAM_RANGE_END - GRAM_RANGE_START) as usize;
const GRAM_RANGE_START: u8 = 'a' as u8;
const GRAM_RANGE_END: u8 = 'z' as u8;

#[derive(Default)]
struct WarehouseHasher {
    thrice: u16,
    twice: u16,
    seen: [u32; GRAM_RANGE],
}

// Not sure how to do this with the BuildHasherDefault?
// Is this the equivalent of Default anyway?
/*
impl Default for WarehouseHasher {
    fn default() -> WarehouseHasher {
        WarehouseHasher {
            thrice: 0,
            twice: 0,
            new_line: false,
            seen: [0; GRAM_RANGE],
        }
    }
}
*/

impl Hasher for WarehouseHasher {
    fn write(&mut self, bytes: &[u8]) {
        const N: u8 = '\n' as u8;
        const R: u8 = '\r' as u8;

        // For Debug only to output csv
        for i in GRAM_RANGE_START..GRAM_RANGE_END {
            print!("{},", (i) as char);
        }

        // For Debug only to output csv
        println!("{},twice,thrice,line", GRAM_RANGE_END as char);

        let mut line_string = String::default();

        for byte in bytes {
            match byte {
                &N | &R => {
                    // For Debug only to output csv
                    for count in &self.seen {
                        print!("{},", count);
                    }

                    // Iterate over everything looking for letters occuring twice.
                    for count in &self.seen {
                        if *count == 2 {
                            self.twice += 1;
                            break;
                        }
                    }

                    // Iterate over everything looking for letters occuring twice.
                    for count in &self.seen {
                        if *count == 3 {
                            self.thrice += 1;
                            break;
                        }
                    }

                    // Debug only - current tally and line read in
                    println!("{:05},{:06},{}", self.twice, self.thrice, line_string);

                    // Clear line and array
                    line_string.clear();

                    for i in &mut self.seen[0..GRAM_RANGE] {
                        *i = 0
                    }
                }

                //TODO: Bug? Is range inclusive or exclusive?
                GRAM_RANGE_START...GRAM_RANGE_END => {
                    self.seen[(byte - GRAM_RANGE_START) as usize] += 1;
                    line_string += &(*byte as char).to_string();
                }

                &_ => {}
            }
        }
    }

    fn finish(&self) -> u64 {
        println!("twice {}   thrice {}", self.twice, self.thrice);
        (self.twice * self.thrice) as u64
    }
}

type WarehouseBuildHasher = BuildHasherDefault<WarehouseHasher>;

/// Enter data to test via stdin for this day's exercise.
fn main() -> io::Result<()> {
    println!("Day 02: https://adventofcode.com/2018/day/2");

    let stdin = io::stdin();

    // Locking once for all reading.
    let mut input_handle = stdin.lock();

    let mut input_string = String::default();
    match input_handle.read_to_string(&mut input_string) {
        Ok(_) => {}     //TODO
        Err(_err) => {} // TODO
    }

    let input_bytes = input_string.into_bytes();

    let build_hasher = WarehouseBuildHasher::default();
    let mut hasher = build_hasher.build_hasher();

    hasher.write(&input_bytes);
    println!("{}", hasher.finish());

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day_02a_example_01() {
        // Example taken from in exercise description
        let input_string =
            String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n\n");
        //                abcdef  bababc  abbcde  abcccd  aabcdd  abcdee  ababab
        let input_bytes = input_string.into_bytes();
        let build_hasher = WarehouseBuildHasher::default();
        let mut hasher = build_hasher.build_hasher();

        hasher.write(&input_bytes);
        let hash = hasher.finish();
        assert_eq!(hash, 12);
    }
}
