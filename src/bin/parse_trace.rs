use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let handle = stdin.lock();

    let mut instruction_counts = HashMap::new();
    for line in handle.lines() {
        let line = line.unwrap();
        if !line.starts_with('#') {
            eprintln!("Skipping non-trace line: {line}");
            continue;
        }

        let parts = line.split('|').skip(1).collect::<Vec<_>>();
        if parts.is_empty() {
            eprintln!("Skipping empty line: {line}");
            continue;
        }

        let instruction = parts[0].trim().split(' ').next().unwrap();
        let count = instruction_counts
            .entry(instruction.to_string())
            .or_insert(0);
        *count += 1;
    }

    // Convert the HashMap to a vector of (instruction, count) pairs
    let mut pairs: Vec<_> = instruction_counts.into_iter().collect();

    // Sort the vector by the count in descending order
    pairs.sort_by(|a, b| b.1.cmp(&a.1));

    // Print the sorted pairs
    let total_count: u64 = pairs.iter().map(|(_, count)| *count).sum();
    println!("Total: {total_count}");
    for (instruction, count) in pairs {
        println!(
            "{instruction:16}: {count:9}, {:6.3}%",
            100.0 * count as f32 / total_count as f32
        );
    }
}
