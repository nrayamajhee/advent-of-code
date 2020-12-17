use crate::read_lines;
use anyhow::Result;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

const BAG: &'static str = "shiny gold";

#[derive(Debug)]
struct Line {
    bag: String,
    contents: Vec<(String, usize)>,
}

impl Line {
    pub fn parse(line: &str) -> Result<Self> {
        let split: Vec<_> = line.split("bags contain").collect();
        let bag = split[0].trim().to_string();
        let split: Vec<_> = split[1].trim().split(',').collect();
        let regex = Regex::new(r"(\d) ([a-zA-Z].*) bag")?;
        let mut contents = Vec::new();
        for each in split.iter() {
            if let Some(caps) = regex.captures(&each.trim()) {
                contents.push((caps[2].to_owned(), caps[1].parse()?));
            }
        }
        Ok(Self { bag, contents })
    }
}

#[derive(Debug)]
struct Node {
    found_bag: RefCell<Option<bool>>,
    contents: Vec<(String, usize)>,
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut graph: HashMap<String, Node> = HashMap::new();
    for each in read_lines(filename)? {
        let line = Line::parse(&each?)?;
        graph.insert(
            line.bag,
            Node {
                found_bag: RefCell::new(None),
                contents: line.contents,
            },
        );
    }
    let sum = graph.iter().fold(0, |sum, (bag, node)| {
        let mut found = false;
        if bfs(&graph, &bag, BAG) {
            found = true;
        }
        // this will prevent unecessary bf search for other bags that contain this 'bag'
        *node.found_bag.borrow_mut() = Some(found);
        if found {
            sum + 1
        } else {
            sum
        }
    });
    Ok(sum)
}

fn bfs(graph: &HashMap<String, Node>, current: &str, looking_for: &str) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(current.to_owned());
    while let Some(top) = queue.pop_front() {
        let top_node = graph.get(&top).unwrap();
        if let Some(found) = top_node.found_bag.borrow().as_ref() {
            if *found {
                return true;
            }
        } else {
            for (each, _) in &top_node.contents {
                // only compare children and not the 'looking_for' bag itself
                // because the bag should be contained in another bag
                // and not be a standalone line
                if each == looking_for {
                    return true;
                } else {
                    queue.push_back(each.to_owned());
                }
            }
        }
    }
    false
}

fn dfs(graph: &HashMap<String, Node>, bag: &str) -> usize {
    let node = graph.get(bag).unwrap();
    let mut total_contents = 0;
    for (each, num) in &node.contents {
        let inner = dfs(graph, &each);
        // inner + 1 because you should count this bag
        // even if it doesn't contain more bag
        total_contents += num * (inner + 1);
    }
    total_contents
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut graph: HashMap<String, Node> = HashMap::new();
    for each in read_lines(filename)? {
        let line = Line::parse(&each?)?;
        graph.insert(
            line.bag,
            Node {
                found_bag: RefCell::new(None),
                contents: line.contents,
            },
        );
    }
    Ok(dfs(&graph, BAG))
}
