fn main() {
    // Read data as bytes
    let solution: i16 = include_bytes!("../../input.txt")
        // Create lines by splitting the data by the 'newline' byte escape:
        .split(|b| *b == b'\n')
        // Split lines in half (the two rugsack compartments) => ([u8, u8, ... ],[u8, u8, ... ]):
        .map(|l| l.split_at(l.len() / 2))
        // Iterate over the two compartments:
        .map(|(a, b)| {
            // Iterate over the second one:
            b.iter()
                // Find double entries or short-circuit to false to advance:
                .find(|b| a.contains(b))
                // Calculate the current item priority value:
                .map(|b| {
                    // To illustrate:
                    // b'a' == 97, b'c' == 99
                    // When 'b' is equal to b'c',
                    // we calculate it like:
                    // b'c' - b'a' + 1.
                    // Which results in the value of 3.
                    if *b >= b'a' {
                        // calculate and return priority value
                        (b - b'a') as i16 + 1
                    } else {
                        // calculate and return priority value
                        (b - b'A') as i16 + 27
                    }
                })
                // Last next() value is always None, we unwrap it to a zero value:
                .unwrap_or(0)
        })
        // Sum op the returned priority values:
        .sum();

    println!("Solution {:?}", solution);
}
