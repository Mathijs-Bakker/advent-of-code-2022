// Count fully contained ranges:
// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7 -> Fully contained
// 6-6,4-6 -> Fully contained
// 2-6,4-8
// Result = 2;

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
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count();

    println!("Solution {solution}");
}
