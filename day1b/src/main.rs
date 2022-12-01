fn main() {
    let mut calories = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    calories.sort_unstable();

    println!("{}", calories.into_iter().rev().take(3).sum::<u32>());
}
