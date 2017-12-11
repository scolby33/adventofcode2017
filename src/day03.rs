use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn run() {
    assert!(get_spiral_coords(11)==(2, 0));
    assert!(get_spiral_coords(10)==(2, -1));
    assert!(get_spiral_coords(1)==(0, 0));
    assert!(get_spiral_coords(17)==(-2, 2));

    assert!(solve1(1)==0);
    assert!(solve1(12)==3);
    assert!(solve1(23)==2);
    assert!(solve1(1024)==31);

    assert!(solve2(1)==2);
    assert!(solve2(2)==4);
    assert!(solve2(747)==806);

    // Get the input
    let stdin = io::stdin();
    let input: u32 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();

    println!("Day 3-1: {}", solve1(input));
    println!("Day 3-2: {}", solve2(input));
}

fn solve1(square: u32) -> u32 {
   let (x, y) = get_spiral_coords(square);
   x.abs() as u32 + y.abs() as u32
}

fn solve2(square: u32) -> u32 {
    let mut storage: HashMap<(i32, i32), u32> = HashMap::new();
    storage.insert((0, 0), 1);

    let mut i = 2;
    loop {
        let mut acc: u32 = 0;
        let (x, y) = get_spiral_coords(i);
        acc += storage.get(&(x+1, y)).unwrap_or(&0);
        acc += storage.get(&(x+1, y+1)).unwrap_or(&0);
        acc += storage.get(&(x, y+1)).unwrap_or(&0);
        acc += storage.get(&(x-1, y+1)).unwrap_or(&0);
        acc += storage.get(&(x-1, y)).unwrap_or(&0);
        acc += storage.get(&(x-1, y-1)).unwrap_or(&0);
        acc += storage.get(&(x, y-1)).unwrap_or(&0);
        acc += storage.get(&(x+1, y-1)).unwrap_or(&0);
        
        storage.insert((x, y), acc);

        if acc > square {
            return acc;
        }
        i += 1;
    }
}

fn get_spiral_coords(n: u32) -> (i32, i32) {
    let directions = vec!["R", "U", "L", "D"];
    let mut directions = directions.iter().cycle();
    let nums = count().skip(1).map(|num| num/2);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut i: u32 = 1;

    for num in nums {
        if i >= n { break; }
        let direction = directions.next().unwrap();
        for _ in 0..num {
            if i >= n { break; }
            i+=1;
            match direction {
                &"R" => x += 1,
                &"U" => y += 1,
                &"L" => x -= 1,
                &"D" => y -= 1,
                _ => (),
            }
        }
    }

    (x, y)
}

struct Count {
    val: u32
}

impl Iterator for Count {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        self.val += 1;
        Some(self.val)
    }
}

fn count() -> Count {
    Count { val: 0 }
}
