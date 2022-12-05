use std::io;
use std::io::BufRead;

fn main() {
    let mut sum = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let (first, second) = line.split_once(',').expect("Invalid input");

        fn split(s: &str) -> (usize, usize) {
            let (left, right) = s.split_once('-').expect("Invalid input");
            let parse = |e: &str| e.parse::<usize>().expect("Invalid input");
            (parse(left), parse(right))
        }

        let (a, b) = (split(first), split(second));

        fn overlap((l, r): (usize, usize), (a, b): (usize, usize)) -> bool {
            l <= a && a <= r || l <= b && b <= r
        }

        if overlap(a, b) || overlap(b, a) {
            sum += 1;
        }
    }

    println!("{}", sum);
}
