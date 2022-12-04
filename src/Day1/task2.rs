use std::io;
use std::io::BufRead;

fn main() {
    let mut values = Vec::new();

    let mut current_max = 0;
    let mut current_sum = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            values.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().expect("Failed to parse line");
        }
        current_max = current_max.max(current_sum);
    }
    values.push(current_sum);

    values.sort();
    values.reverse();

    let mut current_sum = 0;
    for i in 0..3 {
        current_sum += values[i];
    }

    println!("{}", current_sum);
}
