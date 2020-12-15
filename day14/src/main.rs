use std::fs::File;
use std::io::BufReader;
use std::{collections::HashMap, io::prelude::*};

fn main() {
    // read lines but without question mark operator
    let lines = match read_file("input") {
        Ok(lines) => lines,
        Err(_) => {
            panic!("Something went wrong in line reading");
        }
    };

    part1(&lines);
    part2(&lines);
}

fn part2(lines: &Vec<String>) {
    // initalize memory
    let mut memory: HashMap<u64, u64> = HashMap::new();

    // initalize a variable that will remeber the most recent mask
    let mut current_mask: Mask = Mask::new("0".to_string());

    // go through the input line by line
    for line in lines {
        // if the input is a mask then set current mask to this Mask object
        if &line[0..4] == "mask" {
            current_mask = Mask::new(line[7..].to_string());
        }

        // if it is a memory value handle it apropriatel
        if &line[0..3] == "mem" {
            // get the address and value numbers
            let addr_val = get_numbers_from_memline(line);
            let addreses = current_mask.apply_address_mask(addr_val.0);

            for addr in addreses {
                // sotre them in memory
                memory.insert(addr, addr_val.1);
            }
        }
    }

    // Sum all the numbers stored in memory
    let mut acc = 0;
    for val in memory.values() {
        acc += val;
    }

    println!("Part2 = {}", acc);
}

fn part1(lines: &Vec<String>) {
    // Part 1

    // initalize memory
    let mut memory: HashMap<u64, u64> = HashMap::new();

    // initalize a variable that will remeber the most recent mask
    let mut current_mask: Mask = Mask::new("0".to_string());

    // go through the input line by line
    for line in lines {
        // if the input is a mask then set current mask to this Mask object
        if &line[0..4] == "mask" {
            current_mask = Mask::new(line[7..].to_string());
        }

        // if it is a memory value handle it apropriatel
        if &line[0..3] == "mem" {
            // get the address and value numbers
            let addr_val = get_numbers_from_memline(line);
            let masked_val = current_mask.apply_mask(addr_val.1);

            // sotre them in memory
            memory.insert(addr_val.0, masked_val);
        }
    }

    // Sum all the numbers stored in memory
    let mut acc = 0;
    for val in memory.values() {
        acc += val;
    }

    println!("Part1 = {}", acc);
}

#[derive(Debug)]
struct Mask {
    m0s: u64,
    m1s: u64,
    str: String,
}

impl Mask {
    // Creates a mask form a string
    fn new(str: String) -> Mask {
        let mut m0s = 0;
        let mut m1s = 0;

        let mut acc = 35;
        for c in str.chars() {
            match c {
                '1' => {
                    m0s += 2u64.pow(acc);
                    m1s += 2u64.pow(acc)
                }
                '0' => {
                    //do nothing
                }
                'X' => m1s += 2u64.pow(acc),
                _ => panic!("invalid entry!"),
            }

            if acc != 0 {
                acc -= 1;
            }
        }

        Mask { m0s, m1s, str }
    }

    // applies the mask onto a number for part1
    fn apply_mask(&self, n: u64) -> u64 {
        let mut res = n;

        res = res & self.m1s; // turn 0s to 0s
        res = res | self.m0s; // turn 1s to 1s

        res
    }

    // function to apply the part2 rules to the address
    fn apply_address_mask(&self, addr: u64) -> Vec<u64> {
        // this will hold the list of addresses
        let mut res: Vec<u64> = Vec::new();

        // this is the address as a 36 bit binary string
        let addr_string = format!("{:036b}", addr);

        // this will hold the address binary with x representing float values
        let mut addr: String = String::new();

        // counstruct the address binary with float values for X
        for (i, c) in self.str.chars().enumerate() {
            if c == 'X' {
                addr.push('X');
            } else {
                addr.push(addr_string.clone().chars().nth(i).unwrap())
            }
        }

        // create a binary tree of all possible floats and get all bottoms (xless nodes)
        let nodes = get_binarytree_nodes_recursive(addr);
        for node in nodes {
            let mut value = 0;
            let mut acc = 35;
            for c in node.chars() {
                if c == '1' {
                    value += 2u64.pow(acc);
                }
                if acc != 0 {
                    acc -= 1;
                }
            }
            value = value | self.m0s; // enforce that all 1s in mask are 1s
            res.push(value);
        }

        res
    }
}

// helper function that returns two possible values of a float
fn floatx(s: String) -> (String, String) {
    return (s.replacen("X", "0", 1), s.replacen("X", "1", 1));
}

// helper function to recursively create binary tree and return all the bottom nodes
fn get_binarytree_nodes_recursive(s: String) -> Vec<String> {
    let mut res = Vec::new();

    if !s.contains('X') {
        return vec![s];
    }

    let temp = floatx(s);
    res.append(&mut get_binarytree_nodes_recursive(temp.0));
    res.append(&mut get_binarytree_nodes_recursive(temp.1));
    return res;
}

// memory will be a hashmap

fn get_numbers_from_memline(s: &String) -> (u64, u64) {
    let spliteq: Vec<String> = s.split("=").map(|s| s.to_string()).collect();
    let val = spliteq[1].clone().trim().parse().unwrap();

    let splitbrac: Vec<String> = spliteq[0].split("[").map(|s| s.to_string()).collect();
    let splitbrac: Vec<String> = splitbrac[1].split("]").map(|s| s.to_string()).collect();
    let addr = splitbrac[0].clone().trim().parse().unwrap();

    (addr, val)
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
