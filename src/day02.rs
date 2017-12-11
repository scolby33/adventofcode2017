use std::io::{self, BufRead};
use std::iter;

pub fn run() {
    assert!(solve1(vec![vec![5, 1, 9, 5],
                        vec![7, 5, 3],
                        vec![2, 4, 6, 8]])==18);

    assert!(solve2(vec![vec![5, 9, 2, 8],
                        vec![9, 4, 7, 3],
                        vec![3, 8, 6, 5]])==9);

    // Get the input
    let stdin = io::stdin();
    let input: Vec<Vec<u32>> = stdin.lock().lines()
        .map(|l| l.unwrap().split_whitespace()
             .map(|w| w.parse().unwrap()).collect())
        .collect();

    println!("Day 2-1: {}", solve1(input.clone()));
    println!("Day 2-2: {}", solve2(input.clone()));

}

fn solve1(vals: Vec<Vec<u32>>)->u32 {
    let mut diffs: Vec<u32> = Vec::new();
    for line in vals.iter() {
        let max = line.iter().max().unwrap();
        let min = line.iter().min().unwrap();
        diffs.push(max - min);
    }
    diffs.iter().sum()
}

fn solve2(mut vals: Vec<Vec<u32>>)->u32 {
    let mut quots: Vec<u32> = Vec::new();
    for line in vals.iter_mut() {
        line.sort_unstable();
        line.reverse();
        for (v1, v2) in line.iter()
            .enumerate()
            .flat_map(|(i, val)| iter::repeat(val).zip(line.iter().skip(i+1))) {
                if v1 % v2 == 0 {
                    quots.push(v1 / v2);
                    break;
                }
            }
    }
    quots.iter().sum()
}
