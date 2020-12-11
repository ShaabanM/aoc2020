use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

// struct Bag {
//     color: String,
//     quantity: i32,
// }

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    children: Vec<String>,
}

fn count_bags(bags: Vec<Bag>, name: &String) -> i32 {
    let mut acc = 0;
    for bag in bags {
        for child in &bag.children {
            if child.contains(name) {
                acc += 1;
                break;
            }
        }
    }
    acc
}

fn get_parents(bags: Vec<Bag>, name: &String) -> Vec<Bag> {
    let mut parents = Vec::new();
    for bag in bags {
        for child in &bag.children {
            if child.contains(name) {
                parents.push(bag.clone());
                break;
            }
        }
    }
    parents
}

fn f(bags: Vec<Bag>, bag: Bag) -> i32 {
    // if reached end of node return 1
    let parents = get_parents(bags.clone(), &bag.name);
    if parents.is_empty() {
        return 1;
    }

    let mut sum = 1;
    for parent in parents {
        sum += f(bags.clone(), parent);
    }

    return sum;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("test")?;
    let mut bags: Vec<Bag> = Vec::new();

    // while !converged {
    //     if lines.len() == 0 {
    //         converged = true;
    //     }

    //let mut acc = 0;

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

        let bag = Bag {
            name: temp[0][0].clone(),
            children: temp[1].clone(),
        };
        bags.push(bag);
    }
    let bag0: Bag = Bag {
        name: "shiny gold".to_string(),
        children: Vec::new(),
    };

    let count = f(bags, bag0);

    println!("{}", count - 1);
    Ok(())
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
