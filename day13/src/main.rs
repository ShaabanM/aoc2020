use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
fn main() {
    let lines = read_file("input").unwrap();
    let time: i32 = lines[0].parse().unwrap();
    let buses: Vec<String> = lines[1].split(",").map(|s| s.to_string()).collect();
    let mut min_id = std::i32::MAX;
    let mut min_minutes = std::i32::MAX;
    let mut acc: u128 = 19;
    let mut x: u128 = 0;

    for (i, bus) in buses.iter().enumerate() {
        if bus.chars().nth(0).unwrap() != 'x' {
            let bus_num: i32 = bus.parse().unwrap();
            let minutes_2_next = bus_num - (time % bus_num);
            if minutes_2_next < min_minutes {
                min_minutes = minutes_2_next;
                min_id = bus_num;
            }

            if i == 0 {
                x = bus_num as u128;
            } else {
                while (x + (i as u128)) % (bus_num as u128) != 0 {
                    x += acc;
                }
                acc *= bus_num as u128; /* only way to ensure that adding new number
                                        to your solution will not change the value of the mod is if
                                        you increment by products of the mod to ensure zeroing
                                        e.g. t+n(a1*a2*a3*a4...) % ai = t % ai
                                        thus you dont lose the "memory of the previous value" */
            }
        }
    }

    println!(" aaaa ={}", x);
    println!("{},{}", min_minutes, min_id);
    println!("{}", min_id * min_minutes);
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
