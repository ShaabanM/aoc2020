use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    part1();
    part2();
}

// this is basically the exact same thing as part 1 but ALOT faster since it uses hash lookups
fn part2() {
    let mut spoken: HashMap<usize, usize> = HashMap::new();
    let mut current = 0;
    let mut prev = 0;
    let mut acc = 8;

    spoken.insert(18, 1);
    spoken.insert(8, 2);
    spoken.insert(0, 3);
    spoken.insert(5, 4);
    spoken.insert(4, 5);
    spoken.insert(1, 6);
    spoken.insert(20, 7);

    loop {
        prev = current;
        match spoken.get(&prev) {
            Some(idx) => current = acc - idx.clone(),
            None => current = 0,
        };

        spoken.insert(prev, acc);
        if acc == 30000000 - 1 {
            println!("{}", current);
            break;
        }
        acc += 1;
    }
}

fn part1() {
    let mut spoken = vec![18, 8, 0, 5, 4, 1, 20];
    let mut current: usize = spoken.len() - 1;

    loop {
        let mut count = 0;
        let mut idx_hi = 0;
        let mut idx_low = 0;

        for (i, s) in spoken.iter().enumerate().rev() {
            if *s == spoken[current] {
                if count == 1 {
                    idx_low = i + 1;
                    count += 1;
                    break;
                }
                if count == 0 {
                    idx_hi = i + 1;
                    count += 1;
                }
            }
        }

        if current == 2019 {
            println!("{}", spoken[current]);
            //println!("{:?}", spoken);
            break;
        }

        if count == 2 {
            spoken.push(idx_hi - idx_low);
        } else {
            spoken.push(0);
        }
        current += 1;
    }
}
