use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    // Open file using file open
    let f = File::open("data_puzz1.dat").expect("problems");
    let f = BufReader::new(f);

    // Let stars be a vector of type i32
    let mut stars: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in f.lines() {
        // Get the star
        let star: i32 = line.unwrap().trim().parse::<i32>().unwrap();
        // Push it to the array
        stars.push(star)
    }

    for i in 0..stars.len() {
        for j in i..stars.len() {
            // Puzzle one
            if stars[i] + stars[j] == 2020 {
                println!("{} + {} = 2020", stars[i], stars[j]);
                println!("{} x {} = {}", stars[i], stars[j], stars[i] * stars[j]);
            }

            // Puzzle two
            for k in j..stars.len() {
                if stars[i] + stars[j] + stars[k] == 2020 {
                    println!("{} + {} + {} = 2020", stars[i], stars[j], stars[k]);
                    println!(
                        "{} x {} x {} = {}",
                        stars[i],
                        stars[j],
                        stars[k],
                        stars[i] * stars[j] * stars[k]
                    );
                }
            }
        }
    }
}
