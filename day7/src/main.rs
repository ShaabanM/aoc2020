use std::io::{self, BufReader};
use std::{collections::HashMap, fs::File};
use std::{collections::HashSet, io::prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("input")?;
    let mut inverse_bag_tree: HashMap<String, Vec<String>> = HashMap::new();
    let mut bag_tree: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        // Clean up that data by removing uninformative info
        let line = line.replace("bags", "");
        let line = line.replace("bag", "");
        let line = line.replace(".", "");

        // Split the string into components
        // temp [0][0] = parent bag
        // temp [1][:] = list of all contained bags
        let temp: Vec<Vec<String>> = line
            .split(" contain ")
            .map(|s| {
                s.trim()
                    .split(",")
                    .map(|ss| ss.trim().to_string())
                    .collect()
            })
            .collect();

        bag_tree.insert(temp[0][0].clone(), temp[1].clone());

        // inverse bag tree will be hashmap of child: vec<parents>
        for bag in temp[1].iter() {
            if bag != "no other" {
                let bag = bag[1..].trim().to_string();
                match inverse_bag_tree.get_mut(&bag) {
                    Some(parents) => parents.push(temp[0][0].clone()),
                    None => {
                        let mut parents = Vec::new();
                        parents.push(temp[0][0].clone());
                        match inverse_bag_tree.insert(bag, parents) {
                            Some(_) => {}
                            None => {}
                        }
                    }
                }
            }
        }
    }
    let containers = get_containers(&"shiny gold".to_string(), &inverse_bag_tree, HashSet::new());

    println!("part1 = {:?}", containers.len());
    println!(
        "part2 = {:?}",
        get_contained(&"shiny gold".to_string(), &bag_tree) - 1
    );

    Ok(())
}

fn get_containers(
    start: &String,
    graph: &HashMap<String, Vec<String>>,
    mut nodes: HashSet<String>,
) -> HashSet<String> {
    // handle no parents
    match graph.get(start) {
        Some(parents) => {
            let mut acc = HashSet::new();
            for parent in parents {
                nodes.insert(parent.clone());
                acc.extend(get_containers(parent, graph, nodes.clone()));
            }
            return acc;
        }
        None => {
            return nodes;
        }
    }
}

fn get_contained(start: &String, graph: &HashMap<String, Vec<String>>) -> u32 {
    let children = graph.get(start).unwrap();
    if children[0].contains("other") {
        return 1;
    }

    let mut acc = 1;
    for child in children {
        let num = child.chars().next().unwrap();
        let num = num.to_digit(10).unwrap();
        let child = child[1..].trim().to_string();
        acc += num * get_contained(&child, &graph);
    }
    acc
}

// Box error is used as a parent to handle the fact that there are
// multiple different errors that can be raised ehre
fn read_file(name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open file using file open
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);
    // lines as a vector of strings
    let mut lines: Vec<String> = Vec::new();

    // Read the file line by line
    for line in f.lines() {
        // Get the line
        let rline: String = line?;
        let rline: String = rline.trim().parse::<String>()?;
        // Push it to the array
        lines.push(rline);
    }

    Ok(lines)
}
