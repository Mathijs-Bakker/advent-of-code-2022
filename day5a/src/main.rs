#![warn(clippy::all, clippy::pedantic)]

// const N_STACKS: usize = 3; // example-input.txt
const N_STACKS: usize = 9;

fn main() {
    // let data = include_str!("../example-input.txt");
    let data = include_str!("../input.txt");

    // Split data into crates and movement instructions:
    // let (crates, moves) = data.split_at(49); // example-input.txt
    let (crates, moves) = data.split_at(325);

    let crates = crates.lines().collect::<Vec<&str>>();

    let mut stacks: [Vec<char>; N_STACKS] = Default::default();
    (0..N_STACKS).for_each(|i| {
        for l in &crates {
            // Populate stacks with crate id's:
            stacks[i].push(
                l.chars()
                    // Calculate total rows to fetch:
                    .nth((i * 3) + 1 + i)
                    // Filter by alphabetic characters:
                    .filter(char::is_ascii_alphabetic)
                    // If 'None' or non-alphabetical then add a unique placeholder.
                    .unwrap_or('-'),
            );
            // Clean up the stacks by removing the unique placeholders.
            if stacks[i].last() == Some(&'-') {
                stacks[i].pop();
            }
        }
        // Reverse crates for sane movement:
        stacks[i].reverse();
    });

    // Get the crane movement directions:
    let mut directions: Vec<Vec<&str>> = Vec::default();

    for l in moves.lines().collect::<Vec<&str>>() {
        let temp = l.split_whitespace().collect::<Vec<&str>>();
        directions.push(temp);
    }

    for d in directions {
        let n_crates: usize = d[1].parse().unwrap();
        let origin: usize = d[3].parse::<usize>().unwrap() - 1;
        let destination: usize = d[5].parse::<usize>().unwrap() - 1;

        (0..n_crates).for_each(|i| {
            let tmp = stacks.clone();
            let v = tmp[origin].last().unwrap();
            stacks[origin].pop();
            stacks[destination].push(*v);
        });
    }

    let mut solution: String = String::new();
    for s in &stacks {
        solution.push(*s.last().unwrap_or(&' '));
    }
    println!("Solution: {:?}", solution);
}
