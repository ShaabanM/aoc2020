use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut lines = match read_file("input") {
        Ok(output) => output,
        Err(_) => panic!("Something went wrong in the file reading phase"),
    };

    // hashmap of fields
    let mut fields = HashMap::new();
    let mut your_ticket: Ticket = Ticket { values: Vec::new() };
    let mut tickets = Vec::new();
    let mut valid_tickets = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if i <= 19 {
            let f = Field::new(line.clone());
            fields.insert(f.clone().name, f);
        }

        if i == 22 {
            your_ticket = Ticket::new(line.clone());
            valid_tickets.push(your_ticket.clone());
        }

        if i >= 25 {
            let t = Ticket::new(line.clone());
            tickets.push(t);
        }
    }

    // look over all the tickets and some up all the invalid entries
    let mut acc = 0;
    for ticket in tickets.iter() {
        let mut valid = true;
        for val in &ticket.values {
            if !isvalid(*val, &fields) {
                acc += val;
                valid = false;
            }
        }
        if valid {
            valid_tickets.push(ticket.clone())
        }
    }

    println!("Part 1 = {}", acc);

    let mut value_map: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut field_map = HashMap::new();

    for i in 0..20 {
        value_map.insert(i, Vec::new());
    }

    for ticket in valid_tickets.iter() {
        for (i, value) in ticket.values.iter().enumerate() {
            match value_map.get_mut(&i) {
                Some(vec) => vec.push(*value),
                None => {
                    panic!("something went wrong")
                }
            }
        }
    }

    // go through every value 0-19 in the map
    for vmap in value_map {
        let mut f = Vec::new();
        // for each position check each field
        for field in fields.iter() {
            let mut acc = 0;
            // for each field check each value
            for value in vmap.1.iter() {
                if field.1.range.contains(&value) {
                    acc += 1;
                } else {
                    break;
                }
            }
            if acc == vmap.1.len() {
                f.push(field.0.clone());
            }
        }
        field_map.insert(vmap.0.clone(), f);
    }

    let mut reduced_map = HashMap::new();
    loop {
        let mut remove = 0;
        let mut one = String::new();
        for i in field_map.iter() {
            let temp = field_map.get(i.0).unwrap();
            if temp.len() == 1 {
                reduced_map.insert(i.0.clone(), i.1[0].clone());
                remove = i.0.clone();
                one = i.1[0].clone();
                break;
            }
        }

        field_map.remove(&remove);

        for i in &mut field_map {
            i.1.retain(|x| *x != one);
        }

        if field_map.is_empty() {
            break;
        }
    }

    let mut acc: u128 = 1;
    let vals = your_ticket.values.clone();
    for i in reduced_map {
        if i.1.contains("departure") {
            acc *= vals[i.0] as u128;
        }
    }

    println!("{}", acc);
}

#[derive(Debug, Clone, PartialEq)]
struct Ticket {
    values: Vec<i32>,
}

impl Ticket {
    fn new(s: String) -> Ticket {
        // collect all the comma seperated values into a vector
        let values = s.split(',').map(|s| s.parse().unwrap()).collect();
        // return the ticket
        Ticket { values }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Field {
    name: String,
    range: HashSet<i32>,
}

impl Field {
    fn new(s: String) -> Field {
        // split at : to get <"name", "range1 or range2" >
        let temp: Vec<String> = s.split(":").map(|s| s.to_string()).collect();

        // clone "name" into a variable
        let name = temp[0].clone();

        // Shadow temp into a double split at "or" and "-" to get
        // <<r1_low,r1_hi>,<r2_low,r2_hi>>
        let temp: Vec<Vec<String>> = temp[1]
            .split("or")
            .map(|s| {
                s.to_string()
                    .trim()
                    .split('-')
                    .map(|s| s.to_string())
                    .collect()
            })
            .collect();
        let r1_low: i32 = temp[0][0].parse().unwrap();
        let r1_hi: i32 = temp[0][1].parse().unwrap();
        let r2_low: i32 = temp[1][0].parse().unwrap();
        let r2_hi: i32 = temp[1][1].parse().unwrap();

        let mut range: HashSet<i32> = (r1_low..r1_hi + 1).collect();

        for i in r2_low..r2_hi + 1 {
            range.insert(i);
        }

        Field { name, range }
    }
}

// Check if a number is valid given a list of fields
fn isvalid(number: i32, fields: &HashMap<String, Field>) -> bool {
    let mut res = false;
    for field in fields.iter() {
        match field.1.range.get(&number) {
            Some(_) => {
                res = true;
                break;
            }
            None => {}
        }
    }
    res
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
