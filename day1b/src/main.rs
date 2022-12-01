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

    elves_score.sort_unstable();
    elves_score.reverse();

    let mut count: u8 = 1;
    let mut res = 0;
    for score in elves_score {
        if count <= 3 {
            res += score;
            count += 1;
        } else {
            break;
        }
    }

    println!("Result {res}");
}
