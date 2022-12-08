use std::io::BufRead;

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut visible = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let current_tree = lines[i][j];
            let mut is_visible_left = true;
            let mut is_visible_right = true;
            let mut is_visible_top = true;
            let mut is_visible_bottom = true;

            for elem in lines.iter().take(i) {
                let tree = elem[j];
                if current_tree <= tree {
                    is_visible_top = false;
                    break;
                }
            }

            for elem in lines.iter().skip(i + 1) {
                let tree = elem[j];
                if current_tree <= tree {
                    is_visible_bottom = false;
                    break;
                }
            }

            for x in 0..j {
                let tree = lines[i][x];
                if current_tree <= tree {
                    is_visible_left = false;
                    break;
                }
            }

            for x in j + 1..lines[i].len() {
                let tree = lines[i][x];
                if current_tree <= tree {
                    is_visible_right = false;
                    break;
                }
            }

            if j == 0
                || j + 1 == lines[i].len()
                || i == 0
                || i + 1 == lines.len()
                || is_visible_left
                || is_visible_right
                || is_visible_top
                || is_visible_bottom
            {
                println!("{} {}", i, j);
                visible += 1;
            }
        }
    }

    println!("{}", visible);
}
