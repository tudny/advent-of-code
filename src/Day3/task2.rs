extern crate core;

use std::collections::HashSet;
use std::io::BufRead;

fn str_to_char_set(a: String) -> HashSet<char> {
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

    let mut i = 0;
    let mut first = None;
    let mut second = None;
    for line in std::io::stdin().lock().lines() {
        match i {
            0 => {
                first = line.ok();
            },
            1 => {
                second = line.ok();
            },
            2 => {
                let a = first.clone().unwrap();
                let a_set = str_to_char_set(a);
                let b = second.clone().unwrap();
                let b_set = str_to_char_set(b);
                let c_set = str_to_char_set(line.ok().unwrap());

                let inter = a_set.intersection(&b_set);
                let inter: HashSet<_> = inter.into_iter().map(|x| x.clone()).collect();
                let mut interr = c_set.intersection(&inter);

                let el = interr.next();
                sum += value_of(el.unwrap());
            },
            _ => unreachable!("")
        }

        i = (i + 1) % 3;
    }

    println!("{sum}")
}
