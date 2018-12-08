use std::io::{self, BufRead};

#[cfg(test)]
use std::io::Cursor;

#[derive(Debug, PartialEq)]
struct Claims {
    ids: Vec<String>,
    bounding_boxes: Vec<[i32; 4]>,
}

/// Read all frequencies from standard in
fn read_claims<T: BufRead>(read_handle: &mut T, claims: &mut Claims) {
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

        // Lexical scope for iterating over string
        {
            let mut input_iter = input.split_whitespace();

            // Unwrap "panics" on error - which seems appropriate for invalid data in this little app
            // ? is shorthand for returning an error. Function would probably need this at the end though
            //  -> Result<(), Box<dyn Error>>
            // which reads something like Ok, or any kind of error
            let claim_id = input_iter.next().unwrap();
            claims.ids.push(String::from(claim_id));

            let at = input_iter.next().unwrap();
            assert_eq!(at, "@");

            let raw_pos = input_iter.next().unwrap();
            let mut raw_pos_iter = raw_pos.split(|c| c == ',' || c == ':');
            let left = raw_pos_iter.next().unwrap().parse::<i32>().unwrap();
            let top = raw_pos_iter.next().unwrap().parse::<i32>().unwrap();

            let raw_size = input_iter.next().unwrap();
            let mut raw_size_iter = raw_size.split('x');
            let width = raw_size_iter.next().unwrap().parse::<i32>().unwrap();
            let height = raw_size_iter.next().unwrap().parse::<i32>().unwrap();

            claims.bounding_boxes.push([left, top, width, height]);
        }

        // Re-using buffer rather than allocating new string each read.
        input.clear();
    }
}

fn find_contested_claim_square_inches(claims: &Claims) -> (usize, String) {
    use std::collections::HashMap;

    // Index is packed version of box x & y - which keeps list of ids at that spot.
    let mut squares: HashMap<u64, Vec<&String>> = HashMap::default();

    // Figure out what claims each square inch
    for (lhs_index, lhs_box) in claims.bounding_boxes.iter().enumerate() {
        for y in lhs_box[1]..lhs_box[1] + lhs_box[3] {
            for x in lhs_box[0]..lhs_box[0] + lhs_box[2] {
                let id = (x as u64) << 32 | y as u64;
                let counter = squares.entry(id).or_default();
                counter.push(&claims.ids[lhs_index]);
            }
        }
    }

    let contested_count = squares.values().filter(|count| count.len() > 1).count();
    let mut uncontested_ids = Vec::new();

    // Now all the claims have been put in see what was unclaimed
    for (lhs_index, lhs_box) in claims.bounding_boxes.iter().enumerate() {
        let mut is_contested = false;

        // Loop through bounds claimed
        for y in lhs_box[1]..lhs_box[1] + lhs_box[3] {
            for x in lhs_box[0]..lhs_box[0] + lhs_box[2] {
                let id = (x as u64) << 32 | y as u64;

                // Once flag is turned on it remains on until next claim is examined
                let counter = squares.entry(id).or_default();

                // The entire claim is contested if one square inch is contested
                is_contested = is_contested | (counter.len() > 1);

                // I suppose we could rewrite as a while loop checking this.
                if is_contested {
                    break;
                }
            }

            // ...or if Rust had a goto ... :P
            if is_contested {
                break;
            }
        }

        if !is_contested {
            // Only one claim found in entire bounds
            uncontested_ids.push(claims.ids[lhs_index].clone());
        }
    }

    // Should be unnecessary.
    uncontested_ids.dedup();

    // But you know iteration, debugging. Hence why this assert is here.
    assert_eq!(uncontested_ids.len(), 1);
    let uncontested_id = uncontested_ids[0].clone();

    return (contested_count, uncontested_id);
}

/// Enter data to test via stdin for this day's exercise.
/// Assumes there is a solution, if not control + c
/// :P
fn main() -> io::Result<()> {
    println!("Day 03: https://adventofcode.com/2018/day/3");

    let stdin = io::stdin();

    // Locking once for all reading.
    let mut input_handle = stdin.lock();

    // As the sequence now has to be repeated, parse it once and store
    let mut claims = Claims {
        ids: Vec::default(),
        bounding_boxes: Vec::default(),
    };

    read_claims(&mut input_handle, &mut claims);
    let result = find_contested_claim_square_inches(&claims);
    println!(
        "{} contested square inches. Uncontested: {}",
        result.0, result.1
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day_03a_example_01() {
        let mut input_handle =
            Cursor::new("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n\n".as_bytes());

        let mut claims = Claims {
            ids: Vec::default(),
            bounding_boxes: Vec::default(),
        };

        read_claims(&mut input_handle, &mut claims);
        let result = find_contested_claim_square_inches(&claims);

        assert_eq!(claims.ids.len(), 3);
        assert_eq!(claims.bounding_boxes.len(), 3);
        assert_eq!(result.0, 4);
    }
}
