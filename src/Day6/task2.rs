
use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let len = 14;
    let mut best = line.len();
    for k in 0..len {
        let mut i = 0;
        for chunk in &line.chars().skip(k).chunks(len) {
            let mut chars = chunk.collect::<Vec<_>>();
            chars.sort();
            chars.dedup();

            let id = (i + 1) * len + k;
            if chars.len() == len {
                best = best.min(id);
            }

            i += 1;
        }
    }

    println!("{best}")
}
