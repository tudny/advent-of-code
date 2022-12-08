use std::collections::HashMap;
use std::io::BufRead;

enum Node {
    Dir(Vec<String>),
    File(usize),
}

fn traverse(
    node: &str,
    nodes: &HashMap<String, Node>,
    sizes: &mut HashMap<String, usize>,
) -> usize {
    let size = match nodes.get(node).unwrap() {
        Node::Dir(children) => children
            .iter()
            .map(|c| traverse(&c[..], nodes, sizes))
            .sum(),
        Node::File(size) => *size,
    };
    if let Node::Dir(_) = nodes.get(node).unwrap() {
        sizes.insert(node.to_string(), size);
    }
    size
}

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let mut stack = vec!["".to_string()];
    let mut nodes = HashMap::<String, Node>::new();
    nodes.insert("".to_string(), Node::Dir(vec![]));

    for line in lines.skip(1) {
        if line.starts_with("$ ") {
            // parse command
            let command = &line[2..4];
            match command {
                "ls" => {
                    // Actually we don't need to do anything here
                }
                "cd" => {
                    let arg = &line[5..].to_string();
                    let name = stack.last().unwrap().clone() + "/" + arg;
                    if arg == ".." {
                        stack.pop();
                        continue;
                    } else {
                        if let Node::Dir(children) = nodes.get_mut(stack.last().unwrap()).unwrap() {
                            children.push(name.clone());
                        }
                        nodes
                            .entry(name.clone())
                            .or_insert_with(|| Node::Dir(vec![]));
                        stack.push(name);
                    }
                }
                _ => panic!("Unknown command"),
            }
        } else {
            // part of ls output
            if line.starts_with("dir") {
                // dir
                // ignore, don't need to do anything
            } else {
                // file
                let (size, name) = line.split_once(' ').unwrap();
                let size = size.parse::<usize>().unwrap();
                let name = stack.last().unwrap().clone() + "/" + name;
                if let Node::Dir(children) = nodes.get_mut(stack.last().unwrap()).unwrap() {
                    children.push(name.to_string());
                    nodes.insert(name.to_string(), Node::File(size));
                }
            }
        }
    }

    let mut sizes = HashMap::<String, usize>::new();
    traverse("", &nodes, &mut sizes);

    let used_space = sizes.get("").unwrap();
    let required = used_space - 40000000;

    let mut best = 70000001;
    for (name, size) in sizes {
        if size >= required && size < best {
            println!("{}: {}", name, size);
            best = size;
        }
    }
    println!("Best: {}", best);
}
