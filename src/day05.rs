use std::io::{self, BufRead};

pub fn run() {
    assert!(solve1(vec![0, 3, 0, 1, -3])==5);
    assert!(solve2(vec![0, 3, 0, 1, -3])==10);

    // Get the input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines().map(|line| line.unwrap().parse().unwrap()).collect();

    println!("Day 5-1: {}", solve1(input.clone()));
    println!("Day 5-2: {}", solve2(input.clone()));
}

fn solve1(mut instructions: Vec<i32>) -> u32 {
    let len = instructions.len();
    let mut i = 0;
    let mut next: usize;
    let mut total_steps = 0;

    loop {
        if i >= len {
            break;
        }

        next = (i as i32 + instructions[i]) as usize;
        instructions[i] += 1;
        i = next;
        total_steps += 1;
    }
    
    total_steps
}

fn solve2(mut instructions: Vec<i32>) -> u32 {
    let len = instructions.len();
    let mut i = 0;
    let mut next: usize;
    let mut total_steps = 0;

    loop {
        if i >= len {
            break;
        }

        next = (i as i32 + instructions[i]) as usize;
        if instructions[i] >= 3 {
            instructions[i] -= 1;
        } else {
            instructions[i] += 1;
        }
        i = next;
        total_steps += 1;
    }
    
    total_steps
}

