use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("input")?;
    let mut seats: Vec<Seat> = Vec::new();

    for line in lines {
        seats.push(Seat::new(line));
    }

    seats.sort_by_key(|x| x.id);
    let mut id_prev = seats[0].id - 1;

    for (i, seat) in seats.iter().enumerate() {
        if seat.id - id_prev == 1 {
            id_prev = seat.id;
        } else {
            println!("{:?}", seats[i - 1 as usize]);
            println!("{:?}", seat);
            break;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Seat {
    row: i32,
    column: i32,
    id: i32,
}

impl Seat {
    fn new(code: String) -> Seat {
        let row = get_row(&code);
        let column = get_column(&code);
        let id = row * 8 + column;

        Seat {
            row: row,
            column: column,
            id: id,
        }
    }
}

fn get_row(code: &String) -> i32 {
    let chars: Vec<char> = code.chars().collect();
    let mut row_h: i32 = 128;
    let mut row_l: i32 = 0;
    for i in 0..7 {
        if chars[i] == 'F' {
            row_h = (row_h - row_l) / 2 + row_l;
        }

        if chars[i] == 'B' {
            row_l = row_h - (row_h - row_l) / 2;
        }
    }

    row_l
}

fn get_column(code: &String) -> i32 {
    let chars: Vec<char> = code.chars().collect();
    let mut row_h: i32 = 8;
    let mut row_l: i32 = 0;
    for i in 7..10 {
        if chars[i] == 'L' {
            row_h = (row_h - row_l) / 2 + row_l;
        }

        if chars[i] == 'R' {
            row_l = row_h - (row_h - row_l) / 2;
        }
    }

    row_l
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
