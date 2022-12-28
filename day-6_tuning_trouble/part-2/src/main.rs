fn main() {
    let solution = include_bytes!("../../input.txt")
        // Iterate over windows with the size of 14 elements:
        .windows(14)
        // Search for an element in a window, returning its index.
        .position(|b| 
            // Tests if any element of the iterator matches a predicate.
            // If there's no double entry, short circuit when whe have a hit:
            !(0..13)
                .any(|i| (i + 1..14).any(|j| b[i] == b[j]))
        )
        .unwrap()
        // Add 14 to get the marker:
        + 14;

    println!("Solution {solution:?}");
}
