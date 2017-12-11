use std::io::{self, BufRead};

pub fn run(){
    assert!(solve(&vec![1, 1, 2, 2], 1)==3);
    assert!(solve(&vec![1, 1, 1, 1], 1)==4);
    assert!(solve(&vec![1, 2, 3, 4], 1)==0);
    assert!(solve(&vec![9, 1, 2, 1, 2, 1, 2, 9], 1)==9);

    assert!(solve(&vec![1, 2, 1, 2], 2)==6);
    assert!(solve(&vec![1, 2, 2, 1], 2)==0);
    assert!(solve(&vec![1, 2, 3, 4, 2, 5], 3)==4);
    assert!(solve(&vec![1, 2, 3, 1, 2, 3], 3)==12);
    assert!(solve(&vec![1, 2, 1, 3, 1, 4, 1, 5], 4)==4);

    // Get the input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let vals: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    println!("Day 1-1: {}", solve(&vals, 1));
    println!("Day 1-2: {}", solve(&vals, vals.len() / 2));
}

fn solve(input: &Vec<u32>, shift: usize)->u32 {
    input.iter()
        .zip(input.iter().cycle().skip(shift))
        .filter_map(|(a, b)| if a==b { Some(a) } else { None })
        .sum()
}
