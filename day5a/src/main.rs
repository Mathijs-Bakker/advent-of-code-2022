//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
//----------

use std::collections::hash_map::DefaultHasher;

// example-input.txt has 3 stacks of crates:
const N_STACKS: usize = 3;

fn main() {
    let data = include_str!("../example-input.txt");
    // let data = include_bytes!("../example-input.txt");

    // Split data into two
    // let (crates, moves) = data.split_at("\n\n");
    let (crates, moves) = data.split_at(49);

    let solution = crates.lines().collect::<Vec<&str>>();

    let mut stacks: [Vec<char>; N_STACKS] = Default::default();
    for i in 0..N_STACKS {
        for l in &solution {
            println!("l {l}",);
            // Populate stacks with crate letters:
            stacks[i].push(
                l.chars()
                    // Calculate the rows to fetch:
                    .nth((i * 3) + 1 + i)
                    // Filter only alphabetic characters
                    .filter(|c| c.is_ascii_alphabetic())
                    // If 'None' or non-alphabetical then add a dash.
                    .unwrap_or('-'),
            );
            // Clean up the stacks by popping the dashes.
            if stacks[i].last() == Some(&'-') {
                stacks[i].pop();
            }
        }
        // Reverse crates for easy movement:
        stacks[i].reverse();
    }

    for l in stacks {
        println!("Stack {:?}", l);
    }

    // let directions = moves.lines().collect::<Vec<&str>>();

    // let directions: Vec<(u8, u8, u8)> = Default::default();
    // let mut directions: Vec<&str> = Default::default();
    let mut directions: Vec<&str>; // = Default::default();

    for l in moves.lines().collect::<Vec<&str>>() {
        // println!("dir line {:?}", l);
        directions = l.split_whitespace().collect::<Vec<&str>>();
        // println!("foo {:?}", directions);
    }

    // MOVEMENTS:::
}
