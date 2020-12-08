use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "input.dat";
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);
    let lines = read_file(f)?;

    let c1 = get_tree_num(&lines, 1, 1);
    let c2 = get_tree_num(&lines, 3, 1);
    let c3 = get_tree_num(&lines, 5, 1);
    let c4 = get_tree_num(&lines, 7, 1);
    let c5 = get_tree_num(&lines, 1, 2);

    println!(
        "{},{},{},{}, {} = {}",
        c1,
        c2,
        c3,
        c4,
        c5,
        (c1 * c2 * c3 * c4 * c5)
    );

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

fn get_tree_num(lines: &Vec<String>, xslope: usize, yslope: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let len = lines[0].chars().count();

    for i in 0..(lines.len() - 1) / yslope {
        y += yslope;
        x += xslope;
        x = x % len;

        if i + yslope <= lines.len() - 1 {
            let c = lines[y].chars().nth(x).unwrap();
            if c == '#' {
                count += 1;
            }
        }
    }
    count
}
