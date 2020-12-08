use std::io::prelude::*;
use std::io::{self, BufReader};
use std::iter::FromIterator;
use std::{collections::HashSet, fs::File};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("input")?;
    let mut group = "".to_string();
    let mut q_by_anyone = 0;
    let mut q_by_all = 0;
    let mut group_len = 0;

    // Loop over all the lines in the input file
    for line in lines {
        // append that line to an accumulator string that represnets a group
        group.push_str(&line);

        // if line is empty then initalize end of group routine
        if line.len() == 0 {
            // convert the accumulated string into a hashset to get unique members
            let set: HashSet<char> = HashSet::from_iter(group.chars());
            // Count how many unique instanes occur for part 1
            q_by_anyone += set.len();
            // loop over the unique set to count duplicates
            for &element in set.iter() {
                // Count the number of occurances of the char in the accumulated string
                let duplicates = group.matches(element).count();

                // if it has occured as many time as there are people in the group then count it
                if duplicates == group_len {
                    q_by_all += 1;
                }
            }

            // clear the group for reuse
            group_len -= group_len;
            group.clear();
        // check length of unique set
        } else {
            group_len += 1;
        }
    }

    println!("Part1 = {}", q_by_anyone);
    println!("Part2 = {}", q_by_all);

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
