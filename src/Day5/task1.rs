use regex::Regex;

use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut crates_lines = vec![];

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        } else {
            crates_lines.push(line);
        }
    }

    crates_lines.reverse();

    let (line, rest) = crates_lines.split_at(1);
    let crates_lines = rest.to_vec();

    let crates_names = line[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().expect("Not a number"))
        .collect::<Vec<usize>>();

    let mut crates = HashMap::<usize, Vec<char>>::new();
    for name in crates_names.iter() {
        crates.insert(*name, vec![]);
    }

    for line in crates_lines {
        let mut names_it = crates_names.iter();
        let ch_vec: Vec<char> = line.chars().collect();
        for chunk in ch_vec.chunks(4) {
            if chunk[1] != ' ' {
                let stack = crates
                    .get_mut(names_it.next().expect("Expected name"))
                    .expect("Expected stack");
                stack.push(chunk[1]);
            } else {
                names_it.next();
            }
        }
    }

    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let parse_arg = |arg: &str| arg.parse::<usize>().expect("Argument should be a number");

    for line in io::stdin().lock().lines() {
        let line = line.expect("There should be a line");

        let cap = move_re
            .captures(line.as_str())
            .expect("There should be a match");
        let how_many = parse_arg(&cap[1]);
        let from = parse_arg(&cap[2]);
        let to = parse_arg(&cap[3]);

        let from_crate = crates.get_mut(&from).expect("There should be a crate");

        let mut to_move = vec![];
        for _ in 0..how_many {
            to_move.push(from_crate.pop().expect("There should be a char"));
        }

        let to_crates = crates.get_mut(&to).expect("There should be a crate");
        for el in to_move {
            to_crates.push(el);
        }
    }

    let tops = crates
        .iter()
        .sorted()
        .map(|(_, v)| v.last().unwrap())
        .collect::<Vec<&char>>();
    for top in tops.iter() {
        print!("{}", top);
    }
    println!();
}
