fn main() {
    let solution = include_bytes!("../../input.txt")
        // Iterate over windows with the size of 4 elements:
        .windows(4)
        // Search for an element in a winddow, returning its index.
        .position(|b| 
            // Tests if any element of the iterator matches a predicate.
            // If there's no double entry, short circuit when whe have a hit:
            !(0..3)
                .any(|i| (i + 1..4).any(|j| b[i] == b[j]))
        )
        .unwrap()
        // Add 4 to get the marker:
        + 4;

    println!("Solution: {solution:?}");
}
