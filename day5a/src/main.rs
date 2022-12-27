#![warn(clippy::all, clippy::pedantic)]
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
//----------

// example-input.txt has 3 stacks of crates:
// const N_STACKS: usize = 3;
const N_STACKS: usize = 9;

fn main() {
    let data = include_str!("../input.txt");
    // let data = include_str!("../example-input.txt");

    // Split data into two
    // let (crates, moves) = data.split_at("\n\n");
    // let (crates, moves) = data.split_at(49); // DEMO
    let (crates, moves) = data.split_at(325);

    let crates = crates.lines().collect::<Vec<&str>>();

    let mut stacks: [Vec<char>; N_STACKS] = Default::default();
    for i in 0..N_STACKS {
        for l in &crates {
            println!("line: {l}",);
            // Populate stacks with crate id's:
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

    println!(" ---- STACKS ---- ");
    for l in &stacks {
        println!("Stack {:?}", l);
    }

    // let directions = moves.lines().collect::<Vec<&str>>();

    // let directions: Vec<(u8, u8, u8)> = Default::default();
    // let mut directions: Vec<&str> = Default::default();
    let mut directions: Vec<Vec<&str>> = Default::default();

    for l in moves.lines().collect::<Vec<&str>>() {
        // println!("dir line {:?}", l);
        // directions = l.split_whitespace().collect::<Vec<&str>>();
        let temp = l.split_whitespace().collect::<Vec<&str>>();
        directions.push(temp);
        // println!("foo {:?}", directions);
    }

    // for d in &directions {
    //     println!("direction {:?}", d);
    // }

    // MOVEMENTS:::
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3

    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2
    //-------

    println!(" ---- MOVEMENTS ---- ");

    let mut solution: String = "".to_owned();
    for d in directions {
        let n_crates: usize = d[1].parse().unwrap();
        let origin: usize = d[3].parse::<usize>().unwrap() - 1;
        let destination: usize = d[5].parse::<usize>().unwrap() - 1;

        println!("d {:?}", d);
        println!(
            "Move number of crates: {:?} from origin: {:?} to dest: {:?}",
            n_crates, origin, destination
        );

        // ncrates: 1, origin: 2, dest: 1
        // from stacks[1] (origin)
        // ncrates = 1 (one iteration)

        // for i in 0..stacks[origin - 1].len() - n_crates {
        for i in 0..n_crates {
            println!("----- ");
            // println!("stacks[origin].len(): {:?}", stacks[origin].len());
            // println!("crate nr {:?}", i);

            let temp = stacks.clone();
            let v = temp[origin].last().unwrap();
            stacks[origin].pop();
            stacks[destination].push(*v);
        }

        for l in &stacks {
            println!("Stack {:?}", l);
        }
    }
    for s in &stacks {
        solution.push(*s.last().unwrap_or(&' '));
    }
    println!("Solution: {:?}", solution);
}
