// Day 1: find pair of numbers that add up to 2020 and multiply those two numbers
// Return the multiplied value (2sum)

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

    let (a, b) = pair.unwrap();
    dbg!(a * b);


    // Day 1 part 2 (3Sum)
    let triplet = find_triplet(
        include_str!("input.txt")
            .split('\n')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect()
    );

    dbg!(triplet);
    let (a, b, c) = triplet.unwrap();
    dbg!(a * b * c);
 
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

fn find_triplet(mut s: Vec<i64>) -> Option<(i64, i64, i64)> {
    s.sort();  // O(n log n)
    let len_list = s.len();

    for i in 0..(s.len() - 2) {  // O(n-2)
        if i == 0 || (i > 0 && s[i] != s[i-1]) {
            let mut low = i + 1;
            let mut high = len_list - 1;
            let sum_pair = 2020 - s[i];

            while low < high {
                if s[low] + s[high] == sum_pair {
                    dbg!("hello");
                    return Some((s[i], s[low], s[high]));
                } else if s[low] + s[high] > sum_pair {
                    high -= 1;
                } else {
                    low += 1;
                }
            }
        }
    }
    None
}
