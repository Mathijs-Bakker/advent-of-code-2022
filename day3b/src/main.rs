fn main() {
    // Read data as bytes
    let solution: _ = include_bytes!("../input.txt")
        // Create lines by splitting the data by the 'newline' byte escape:
        .split(|b| *b == b'\n')
        // Store them in a vector:
        .collect::<Vec<_>>()
        // Create a chunk (group) of each three lines:
        .chunks(3)
        .map(|range| {
            // Iterate over each first range of a chunk:
            range[0]
                .iter()
                // Find occurrences of each value as in the first range of the chunk.
                .find(|b| range[1].contains(b) && range[2].contains(b))
                // Last value of the iteration is 'None', we return a zero value.
                .unwrap_or(&0)
        })
        .map(|b| {
            if *b >= b'a' {
                // Read the first part of day one for enlightment.
                i16::from(b - b'a') + 1
            } else if b >= &b'A' {
                i16::from(b - b'A') + 27
            } else {
                // Last value that we returned is '0',
                // which is a safe value to sum up.
                0
            }
        })
        .sum::<i16>();

    println!("Solution {solution}");
}
