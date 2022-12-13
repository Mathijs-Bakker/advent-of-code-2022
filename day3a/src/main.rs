fn main() {
    // Read data as bytes
    let data = include_bytes!("../example-input.txt")
        // Create lines by splitting the data by the 'newline' byte escape
        .split(|b| *b == b'\n')
        // Split lines in halve (two compartments)
        .map(|l| l.split_at(l.len() / 2));

    for f in data {
        println!("f {:?}", f);
    }
}
