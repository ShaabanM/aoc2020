use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Populate a vector that contains the input lines
    let lines = read_file("input")?;
    // Initalize a var that will store the invalid number
    let mut invalid_number: u64 = 0;

    // Loop over the enumerate lines
    for (current, line) in lines.iter().enumerate() {
        if current < 25 {
            // do nothing for first 25
        } else {
            // initalize a boolean accumulator
            let mut matched = false;

            // loop over every combination of the 25 entreis
            for i in current - 25..current {
                for j in current - 25..current {
                    let x: u64 = lines[i].parse()?;
                    let y: u64 = lines[j].parse()?;
                    let z: u64 = line.parse()?;

                    // if they sum to the next number then its valid
                    if x + y == z {
                        matched = true;
                    }
                }
            }

            // if an invalid number occured record it and exit loop
            if !matched {
                invalid_number = line.parse()?;
                break;
            }
        }
    }

    // loop over the lines one last time
    for (current, line) in lines.iter().enumerate() {
        // init accumulators for sum, min and max in a contigous set
        let mut sum: u64 = 0;
        let mut max: u64 = std::u64::MIN;
        let mut min: u64 = std::u64::MAX;

        for i in current..lines.len() {
            // parse the string
            let x: u64 = lines[i].parse()?;

            // increment the accumulators
            sum += x;
            if x < min {
                min = x
            };
            if x > max {
                max = x
            };

            // if you found set that sums to the number return the min + max
            if sum == invalid_number {
                println!("{}", max + min);
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
