use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn run() {
   assert!(is_valid1(&vec!["aa", "bb", "cc", "dd", "ee"]));
   assert!(!is_valid1(&vec!["aa", "bb", "cc", "dd", "aa"]));
   assert!(is_valid1(&vec!["aa", "bb", "cc", "dd", "aaa"]));

   assert!(is_valid2(&vec!["abcde", "fghji"]));
   assert!(!is_valid2(&vec!["abcde", "xyz", "ecdab"]));
   assert!(is_valid2(&vec!["a", "ab", "abc", "abd", "abf", "abj"]));
   assert!(is_valid2(&vec!["iiii", "oiii", "ooii", "oooi", "oooo"]));
   assert!(!is_valid2(&vec!["oiii", "ioii", "iioi", "iiio"]));

    // Get the input
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let input: Vec<&str> = input.iter().map(|l| l.as_str()).collect();
    let passphrases: Vec<Vec<&str>> = input.iter().map(|l| l.split_whitespace().collect()).collect();

    println!("Day 4-1: {}", solve1(&passphrases));
    println!("Day 4-2: {}", solve2(&passphrases));
}

fn solve1(passphrases: &Vec<Vec<&str>>) -> u32 {
    passphrases.iter().map(|passphrase| is_valid1(passphrase)).fold(0, |acc, validity| acc + validity as u32)
}

fn solve2(passphrases: &Vec<Vec<&str>>) -> u32 {
    passphrases.iter().map(|passphrase| is_valid2(passphrase)).fold(0, |acc, validity| acc + validity as u32)
}

fn is_valid1(passphrase: &Vec<&str>) -> bool {
    let words: HashSet<&str> = passphrase.iter().cloned().collect();
    passphrase.len() == words.len()
}

fn is_valid2(passphrase: &Vec<&str>) -> bool {
    let mut split: Vec<Vec<char>> = passphrase.iter().cloned().map(|word| word.chars().collect()).collect();
    split.iter_mut().map(|word| word.sort()).count();
    let uniques: HashSet<String> = split.iter().map(|word| word.into_iter().collect()).collect();
    passphrase.len() == uniques.len()
}
