fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>();

    let mut elves_score = vec![];
    let mut score = 0;

    for n in input {
        if n == 0 {
            elves_score.push(score);
            score = 0;
        } else {
            score += n;
        }
    }

    println!("Result {:?}", elves_score.iter().max().unwrap());
}
