use std::iter::FromIterator;
use std::{self, io::BufReader};
use std::{cmp, fs::File};
use std::{collections::HashMap, io::prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut history: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut indexmap: HashMap<i32, usize> = HashMap::new();
    let mut output_joltages = read_file("input")?;
    let mut gaps = String::new();
    output_joltages.sort();

    for (i, &jolt) in output_joltages.iter().enumerate() {
        if i < output_joltages.len() - 1 {
            gaps.push_str(&(output_joltages[i + 1] - jolt).to_string());
        }
        let mut children = Vec::new();
        let end = cmp::min(i + 4, output_joltages.len());
        for j in i..end {
            let diff = output_joltages[j] - jolt;
            if diff <= 3 && diff > 0 {
                children.push(output_joltages[j]);
            }
        }
        graph.insert(jolt, children.clone());
        history.insert(jolt, Vec::new());
        indexmap.insert(jolt, i);
    }

    // let mut acc = 0;
    // let node0 = &output_joltages[0];
    // let mut current_node = node0;
    // let mut branching: Vec<&i32> = Vec::new();
    // loop {
    //     // get children of current node
    //     let children = graph.get(current_node).unwrap();
    //     // get history of current node
    //     let node_history = history.get_mut(current_node).unwrap();

    //     // if node has no children then its at bottom, count it and start going up
    //     if children.is_empty() {
    //         acc += 1;
    //         current_node = node0;
    //     } else if children.len() > 1 {
    //         if children.len() == node_history.len() {
    //             // all children have been visited!
    //             current_node = branching.pop().unwrap();
    //         } else {
    //             branching.push(current_node);
    //             current_node = &children[node_history.len()];
    //             node_history.push(*current_node);
    //         }
    //     }
    // }

    let mut acc: u128 = 1;
    let splits: Vec<String> = gaps.split("3").map(|s| s.to_string()).collect();
    for split in splits {
        if split.len() == 2 {
            acc *= 2;
        }
        if split.len() == 3 {
            acc *= 4;
        }
        if split.len() == 4 {
            acc *= 7;
        }
    }

    println!("{:?}", acc);

    Ok(())
}

// note the recursive method is too ineffiecnt and will not work on main input
fn recurisve_method(graph: &HashMap<i32, Vec<i32>>, node: &i32) -> u128 {
    let children = graph.get(&node).unwrap();
    if children.is_empty() {
        return 1;
    }

    let mut sum: u128 = 0;
    for child in children {
        sum += recurisve_method(graph, child);
    }
    return sum;
}

// Box error is used as a parent to handle the fact that there are
// multiple different errors that can be raised ehre
fn read_file(name: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    // Open file using file open
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);
    // lines as a vector of strings
    let mut lines: Vec<i32> = Vec::new();
    lines.push(0);
    // Read the file line by line
    for line in f.lines() {
        // Get the line
        let rline: String = line?;
        let rline: i32 = rline.trim().parse::<i32>()?;
        // Push it to the array
        lines.push(rline);
    }

    Ok(lines)
}
