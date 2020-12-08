use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
#[derive(Debug)] // enables printing the struct
struct Password {
    password: String,
    min: i32,
    max: i32,
    letter: char,
}

// implementing functions that work on the struct
impl Password {

    // Struct constructor from a string
    fn new(s: &String) -> Password {
        let bytes = s.as_bytes();

        let mut cidx = 0;
        let mut didx = 0;

        for (i, &char) in bytes.iter().enumerate() {
            if char == b'-' {
                didx = i;
            }

            if char == b':' {
                cidx = i;
            }
        }

        Password {
            password: String::from(s[cidx + 1..].trim()),
            min: s[..didx].trim().parse::<i32>().unwrap(),
            max: s[didx + 1..cidx - 1].trim().parse::<i32>().unwrap(),
            letter: s[cidx - 1..cidx].as_bytes()[0] as char,
        }
    }

    fn isvalid_p2(&self) -> bool {
        let mut bool_min = true;
        let mut bool_max = true;
        for (i, letter) in self.password.chars().enumerate() {
            if i as i32 == self.min - 1 {
                bool_min = letter == self.letter
            }

            if i as i32 == self.max - 1 {
                bool_max = letter == self.letter
            }
        }

        bool_max ^ bool_min // Xor
    }

    fn isvalid_p1(&self) -> bool {
        let mut count = 0;
        for letter in self.password.chars() {
            if letter == self.letter {
                count += 1;
            }
        }
        (count >= self.min) && (count <= self.max)
    }
}

// Box error is used as a parent to handle the fact that there are
// multiple different errors that can be raised ehre
fn read_file(f: BufReader<File>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open file using file open
    let name = "data_puzz1.dat";
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);

    // Result type here is handled by ?
    // This requires that main returns result
    let lines = read_file(f)?;

    let mut count1 = 0;
    let mut count2 = 0;

    for line in lines {
        let code = Password::new(&line);
        if code.isvalid_p1() {
            count1 += 1;
        }
        if code.isvalid_p2() {
            count2 += 1;
        }
    }

    println!("p1: {}, p2: {}", count1, count2);

    // If you made it this far then main can return ok with no errors
    Ok(())
}
