extern crate core;

use std::collections::HashSet;
use std::io::BufRead;

fn str_to_char_set(a: &str) -> HashSet<char> {
    HashSet::<char>::from_iter(a.chars().into_iter())
}

fn value_of(x: &char) -> usize {
    let mut range = ('a'..='z').into_iter().collect::<Vec<char>>();
    let mut upper_range = ('A'..='Z').into_iter().collect::<Vec<char>>();
    range.append(&mut upper_range);

    1 + range.iter().position(|a| a == x).expect("Eeee")
}

fn main() {
    let mut sum = 0;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let (a, b) = line.split_at(line.len() / 2);

        let a_set = str_to_char_set(a);
        let b_set = str_to_char_set(b);

        let mut intersection = a_set.intersection(&b_set);
        let element = intersection.next();
        let next = intersection.next();

        match (element, next) {
            (Some(x), None) => {
                sum += value_of(x);
            }
            _ => {
                panic!("Invalid state")
            }
        }
    }

    println!("{sum}")
}
