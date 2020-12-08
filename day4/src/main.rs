use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open file using file open
    let name = "input";
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);

    // Result type here is handled by ?
    // This requires that main returns result
    let lines = read_file(f)?;

    let mut last_blank = 0;
    let mut count = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.len() == 0 || i == lines.len() {
            let mut passport: Vec<Vec<&str>> = Vec::new();

            for j in last_blank..i {
                passport.append(
                    &mut lines[j]
                        .split(" ")
                        .map(|s| s.split(":").collect())
                        .collect(),
                );
            }

            if last_blank != 0 {
                passport.remove(0);
            }

            if isvalid(&passport) {
                println!("{:?}", passport);
                count += 1;
            }
            last_blank = i;
        }
    }

    println!("{}", count);

    Ok(())
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

fn isvalid(passport: &Vec<Vec<&str>>) -> bool {
    let mut res = true;

    if passport.len() <= 6 {
        return false;
    }

    if passport.len() == 7 {
        for e in passport {
            if e[0] == "cid" {
                return false;
            }
        }
    }

    for e in passport {
        //byr
        if e[0] == "byr" {
            let temp: i32 = e[1].parse().unwrap();
            if temp < 1920 || temp > 2002 {
                return false;
            }
        }

        //iyr
        if e[0] == "iyr" {
            let temp: i32 = e[1].parse().unwrap();
            if temp < 2010 || temp > 2020 {
                return false;
            }
        }

        //eyr
        if e[0] == "eyr" {
            let temp: i32 = e[1].parse().unwrap();
            if temp < 2020 || temp > 2030 {
                return false;
            }
        }

        //hgt
        if e[0] == "hgt" {
            let l = e[1].len() - 1;
            let e1 = e[1].to_string();
            if e[1].chars().nth(l - 1).unwrap() == 'i' && e[1].chars().nth(l).unwrap() == 'n' {
                let temp: i32 = e1[0..l - 1].parse().unwrap();
                if temp < 59 || temp > 76 {
                    return false;
                }
            }

            if e[1].chars().nth(l - 1).unwrap() == 'c' && e[1].chars().nth(l).unwrap() == 'm' {
                let temp: i32 = e1[0..l - 1].parse().unwrap();
                if temp < 150 || temp > 193 {
                    return false;
                }
            }

            if e[1].chars().nth(l - 1).unwrap() != 'c' && e[1].chars().nth(l - 1).unwrap() != 'i' {
                return false;
            }

            if e[1].chars().nth(l).unwrap() != 'm' && e[1].chars().nth(l).unwrap() != 'n' {
                return false;
            }
        }

        //hcl
        if e[0] == "hcl" {
            if e[1].len() != 7 {
                return false;
            }

            if e[1].chars().nth(0).unwrap() != '#' {
                return false;
            }

            let temp = [
                'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '#',
            ];

            for c in e[1].chars() {
                if !temp.iter().any(|&x| x == c) {
                    return false;
                }
            }
        }

        //ecl
        if e[0] == "ecl" {
            let temp = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if !temp.iter().any(|&x| x == e[1]) {
                return false;
            }
        }

        //pid
        if e[0] == "pid" {
            if e[1].len() != 9 {
                return false;
            }
        }
    }

    true
}
