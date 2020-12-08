use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use trees::{fr, tr};

// struct Bag {
//     color: String,
//     quantity: i32,
// }

#[derive(Debug)]
struct Bag {
    name: String,
    parent: Option<Box<Bag>>,
    children: Vec<Bag>,
}

fn count_bags(bag: Option<Box<Bag>>) -> i32 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sgold = Bag {
        name: "shiny gold".to_string(),
        parent: None,
        children: Vec::new(),
    };

    let mut lines = read_file("input")?;

    // while !converged {
    //     if lines.len() == 0 {
    //         converged = true;
    //     }

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

        for s in temp[1].clone() {
            // let temp2: Vec<&str> = s.split(" ").collect();
            // let mut name: String = temp2[1].to_string();
            // name.push_str(" ");
            // name.push_str(temp2[2]);
            if s.contains("shiny gold") {
                let child = Bag {
                    name: temp[0][0].clone(),
                    parent: Some(Box::new(sgold)),
                    children: Vec::new(),
                };
                println!("{}", temp[0][0]);
                break;
            }
        }
    }
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
