fn main() {
    let solution = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (range1, range2) = l.split_once(',').unwrap();
            let ((a, b), (c, d)) = (
                range1.split_once('-').unwrap(),
                range2.split_once('-').unwrap(),
            );
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| a <= d && c <= b)
        .count();

    println!("Solution {solution}");
}
