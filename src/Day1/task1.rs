
use std::io;
use std::io::BufRead;

fn main() {
    let mut current_max = 0;
    let mut current_sum = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().expect("Failed to parse line");
        }
        current_max = current_max.max(current_sum);
    }

    println!("{}", current_max);
}
