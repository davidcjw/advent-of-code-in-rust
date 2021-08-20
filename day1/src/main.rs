// Day 1: find pair of numbers that add up to 2020 and multiply those two numbers
// Return the multiplied value

use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    // read from file and display first 40 bytes
    // opening the file at runtime means it can fail
    // so we need to ensure that the input.txt file is always next to the executable.
    // let s = std::fs::read_to_string("./src/input.txt")?;

    // with .split(), we don't get back an array, but instead we get:
    // (Split<&str>) that implements Iterator<Item = &str>
    let pair = find_pair(
        include_str!("input.txt")
            .split('\n')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect()
    );

    dbg!(pair);
    let (a, b) = pair.unwrap();
    dbg!(a * b);

    Ok(())
}

fn find_pair(s: Vec<i64>) -> Option<(i64, i64)> {
    let mut complements = HashSet::new();

    for num in s.iter() {
        let complement = 2020 - num;

        if complements.contains(&complement) {
            return Some((*num, complement));
        } else {
            complements.insert(num);
        }
    };
    None
}
