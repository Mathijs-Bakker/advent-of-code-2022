fn main() {
    let solution = include_str!("../../data.txt")
        .lines()
        .map(|l| {
            (
                (l.as_bytes()[0] - b'A') as i16,
                (l.as_bytes()[2] - b'X') as i16,
            )
        })
        .map(|(a, b)| 1 + b + 3 * ((1 + b - a).rem_euclid(3)))
        .sum::<i16>();

    println!("score {solution:?}");
}
