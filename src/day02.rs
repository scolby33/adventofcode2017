use std::io::{self, BufRead};

pub fn run() {
    assert!(solve(vec![vec![5, 1, 9, 5],
                       vec![7, 5, 3],
                       vec![2, 4, 6, 8]])==18);

    // Get the input
    let stdin = io::stdin();
    let input: Vec<Vec<u32>> = stdin.lock().lines()
        .map(|l| l.unwrap().split_whitespace()
             .map(|w| w.parse().unwrap()).collect())
        .collect();

    println!("Day 2-1: {}", solve(input));

}

fn solve(vals: Vec<Vec<u32>>)->u32 {
    let mut diffs = Vec::new();
    for line in vals.iter() {
        let max = line.iter().max().unwrap();
        let min = line.iter().min().unwrap();
        diffs.push(max - min);
    }
    diffs.iter().sum()
}
