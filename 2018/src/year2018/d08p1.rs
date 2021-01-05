use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day08.txt")?;
    let reader = BufReader::new(f);
    let mut num_iter = reader.split(b' ').map(|s| {
        String::from_utf8(s.unwrap())
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });

    let root = make_node(&mut num_iter);
    println!("{}", get_sum(&root));
    Ok(())
}

fn get_sum(node: &Node) -> usize {
    node.metadata.iter().sum::<usize>() + node.children.iter().map(|c| get_sum(c)).sum::<usize>()
}

fn make_node<I>(num_iter: &mut I) -> Node
where
    I: Iterator<Item = usize>,
{
    let num_children = num_iter.next().unwrap();
    let num_meta = num_iter.next().unwrap();

    let mut node = Node {
        children: Vec::with_capacity(num_children),
        metadata: Vec::with_capacity(num_meta),
    };

    for _ in 0..num_children {
        node.children.push(make_node(num_iter));
    }

    for _ in 0..num_meta {
        node.metadata.push(num_iter.next().unwrap());
    }

    node
}
