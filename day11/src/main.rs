use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Map = Vec<Vec<char>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map: Map = read_file("input")?;

    loop {
        let temp = get_nextmap(map.clone());
        //println!("{}", temp.0);
        if !temp.0 {
            break;
        }
        map = temp.1.clone();
    }

    let mut acc = 0;
    for row in map {
        for column in row {
            if column == '#' {
                acc += 1;
            }
        }
    }

    //println!("{:?}", get_neighbours2(map, 9, 2));
    println!("{}", acc);

    Ok(())
}

fn get_nextmap(map: Map) -> (bool, Map) {
    let mut next_map: Map = map.clone();
    let mut changed: bool = false;

    for (i, row) in map.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            match column {
                'L' => {
                    let mut occupied = false;
                    let neighbours = get_neighbours2(map.clone(), i, j);
                    for neighbour in neighbours {
                        if neighbour == '#' {
                            occupied = true;
                            break;
                        }
                    }
                    if !occupied {
                        next_map[i][j] = '#';
                        changed = true;
                    }
                }
                '#' => {
                    let mut occupied = 0;
                    let neighbours = get_neighbours2(map.clone(), i, j);
                    for neighbour in neighbours {
                        if neighbour == '#' {
                            occupied += 1;
                        }
                    }

                    if occupied >= 5 {
                        next_map[i][j] = 'L';
                        changed = true;
                    }
                }
                _ => (),
            }
        }
    }
    (changed, next_map)
}

fn get_direction(
    map: Map,
    i: isize,
    j: isize,
    diri: isize,
    dirj: isize,
    edgei: isize,
    edgej: isize,
) -> char {
    let mut ii = i.wrapping_add(diri);
    let mut jj = j.wrapping_add(dirj);
    loop {
        // handle overflow
        if ii < 0 || jj < 0 || ii > edgei || jj > edgej {
            return '.';
        }

        if map[ii as usize][jj as usize] != '.' {
            break;
        }

        ii = ii.wrapping_add(diri);
        jj = jj.wrapping_add(dirj);
    }
    return map[ii as usize][jj as usize];
}

fn get_neighbours2(map: Map, i: usize, j: usize) -> Vec<char> {
    let len_rows = map.len() - 1;
    let len_columns = map[0].len() - 1;
    let mut neighbours = Vec::new();
    let dirs = vec![-1, 0, 1];
    for ii in dirs.iter() {
        for jj in dirs.iter() {
            if !(*ii == 0 && *jj == 0) {
                neighbours.push(get_direction(
                    map.clone(),
                    i as isize,
                    j as isize,
                    ii.clone(),
                    jj.clone(),
                    len_rows as isize,
                    len_columns as isize,
                ));
            }
        }
    }

    neighbours
}

fn get_neighbours(map: Map, i: usize, j: usize) -> Vec<char> {
    let len_rows = map.len() - 1;
    let len_columns = map[0].len() - 1;
    let mut neighbours = Vec::new();
    let mut idxi = vec![i, i.wrapping_add(1), i.wrapping_sub(1)];
    let mut idxj = vec![j, j.wrapping_add(1), j.wrapping_sub(1)];

    // handle edge cases
    if i == 0 {
        idxi.remove(2);
    }

    if i == len_rows {
        idxi.remove(1);
    }

    if j == 0 {
        idxj.remove(2);
    }

    if j == len_columns {
        idxj.remove(1);
    }

    for ii in idxi {
        for jj in idxj.iter() {
            if !(ii == i && *jj == j) {
                neighbours.push(map[ii][*jj]);
            }
        }
    }

    neighbours
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
