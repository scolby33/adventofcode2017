use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn run() {
    assert!(solve1(vec![0, 2, 7, 0])==(5, vec![2, 4, 1, 2]));
    assert!(solve2(vec![0, 2, 7, 0], vec![2, 4, 1, 2], 5)==4);

    // Get the input
    let stdin = io::stdin();
    let input: Vec<u32> = stdin.lock().lines().next().unwrap().unwrap()
        .split_whitespace().map(|word| word.parse().unwrap()).collect();

    let (steps, final_state) = solve1(input.clone());
    println!("Day 6-1: {}", steps);
    println!("Day 6-2: {}", solve2(input.clone(), final_state, steps));
}

fn solve1(mut memory: Vec<u32>) -> (u32, Vec<u32>) {
    let buckets = memory.len();
    let mut seen_states: HashSet<Vec<u32>> = HashSet::new();
    seen_states.insert(memory.clone());

    let mut steps = 0;
    loop {
        let max = *memory.iter().max().unwrap();
        let mut max_index = memory.iter().position(|x| x == &max).unwrap();
        memory[max_index] = 0;
        for _ in 0..max {
            max_index = (max_index + 1) % buckets;
            memory[max_index] += 1;
        }
        steps += 1;
        if seen_states.contains(&memory) {
            break;
        }
        seen_states.insert(memory.clone());
    }
    (steps, memory)
}

fn solve2(mut memory: Vec<u32>, final_memory: Vec<u32>, total_steps: u32) -> u32 {
    let buckets = memory.len();

    let mut steps = 0;
    loop {
        let max = *memory.iter().max().unwrap();
        let mut max_index = memory.iter().position(|x| x == &max).unwrap();
        memory[max_index] = 0;
        for _ in 0..max {
            max_index = (max_index + 1) % buckets;
            memory[max_index] += 1;
        }
        steps += 1;
        if memory == final_memory {
            break;
        }
    }
    total_steps - steps
}

