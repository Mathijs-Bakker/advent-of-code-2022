fn main() {
    let result = include_str!("../../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();

    println!("Puzzle result {result}");
}
