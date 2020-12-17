use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

fn main() {
    // Read input
    let lines = match read_file("input") {
        Ok(value) => value,
        Err(_) => panic!("Something went wrong while reading lines"),
    };

    // Set that will hold all active positions
    let mut actives: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut inactives: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    // populate initial sublatice slice from input
    for (i, row) in lines.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column == '#' {
                actives.insert((i as i32, j as i32, 0, 0));
            }
            if *column == '.' {
                inactives.insert((i as i32, j as i32, 0, 0));
            }
        }
    }

    for i in -6..16 {
        for j in -6..16 {
            for z in -6..7 {
                for w in -6..7 {
                    inactives.insert((i as i32, j as i32, z, w));
                }
            }
        }
    }

    for i in 0..6 {
        let temp = cycle(actives, inactives);
        actives = temp.0;
        inactives = temp.1;
    }

    println!("{}", actives.len());
}

// perform a single cycle
fn cycle(
    actives: HashSet<(i32, i32, i32, i32)>,
    inactives: HashSet<(i32, i32, i32, i32)>,
) -> (HashSet<(i32, i32, i32, i32)>, HashSet<(i32, i32, i32, i32)>) {
    // sets to hold the updated active and inactive list
    let mut new_actives = HashSet::new();
    let mut new_inactives = HashSet::new();

    for active in actives.iter() {
        let active_neighbours = get_active_neighbours(&actives, active);

        if active_neighbours == 2 || active_neighbours == 3 {
            new_actives.insert(active.clone());
        } else {
            new_inactives.insert(active.clone());
        }
    }

    for inactive in inactives.iter() {
        let active_neighbours = get_active_neighbours(&actives, inactive);

        if active_neighbours == 3 {
            new_actives.insert(inactive.clone());
        } else {
            new_inactives.insert(inactive.clone());
        }
    }

    (new_actives, new_inactives)
}

fn get_active_neighbours(
    actives: &HashSet<(i32, i32, i32, i32)>,
    pos: &(i32, i32, i32, i32),
) -> i32 {
    let mut active_neighbours = 0;

    let neighbours: Vec<(i32, i32, i32, i32)> = get_neighbours(pos.clone());
    for n in neighbours {
        if actives.contains(&n) {
            active_neighbours += 1;
        }
    }
    active_neighbours
}

// function that given a positions will return all the 3d neighbouring positions
fn get_neighbours(pos: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut res = Vec::new();

    for x in pos.0 - 1..pos.0 + 2 {
        for y in pos.1 - 1..pos.1 + 2 {
            for z in pos.2 - 1..pos.2 + 2 {
                for w in pos.3 - 1..pos.3 + 2 {
                    if !(x == pos.0 && y == pos.1 && z == pos.2 && w == pos.3) {
                        res.push((x, y, z, w))
                    }
                }
            }
        }
    }

    res
}

// Box error is used as a parent to handle the fact that there are
// multiple different errors that can be raised ehre
fn read_file(name: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    // Open file using file open
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);
    // lines as a vector of strings
    let mut lines: Vec<Vec<char>> = Vec::new();

    // Read the file line by line
    for line in f.lines() {
        // Get the line
        let rline: String = line?;
        let rline: String = rline.trim().parse::<String>()?;
        // Push it to the array
        lines.push(rline.chars().collect());
    }

    Ok(lines)
}
