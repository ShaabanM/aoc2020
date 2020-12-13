use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let actions = read_file("input")?;
    let mut ship = Ship {
        ns: 0,
        ew: 0,
        direction: 90,
        wns: 1,
        wew: 10,
    };
    let mut ship2 = ship.clone();

    for action in actions.iter() {
        ship.action(action);
        ship2.action2(action)
    }

    println!("{:?}", ship);
    println!("{}", ship.manhattan_distance());

    println!("{:?}", ship2);
    println!("{}", ship2.manhattan_distance());

    Ok(())
}

#[derive(Debug, Clone)]
struct Ship {
    ns: i32,        // location in north(+) south (-)
    ew: i32,        // location in east(+) west (-)
    direction: i32, // direction in angle
    wns: i32,       // waypoint north south
    wew: i32,       //waypoint eeast west
}

impl Ship {
    fn moveby(&mut self, amount: f64) {
        let direction = self.direction as f64;
        let ew_amount = (direction.to_radians().sin() * amount) as i32;
        let ns_amount = (direction.to_radians().cos() * amount) as i32;

        self.ew += ew_amount;
        self.ns += ns_amount;
    }

    fn rotate_wp_by(&mut self, amount: f64) {
        let cs = amount.to_radians().cos() as i32;
        let sn = amount.to_radians().sin() as i32;
        let wew = self.wew * cs - self.wns * sn;
        let wns = self.wew * sn + self.wns * cs;

        self.wew = wew as i32;
        self.wns = wns as i32;
    }

    fn manhattan_distance(&self) -> i32 {
        self.ew.abs() + self.ns.abs()
    }

    fn action(&mut self, action: &Action) {
        match action {
            Action::N(v) => {
                self.ns += v;
            }
            Action::S(v) => {
                self.ns -= v;
            }
            Action::E(v) => {
                self.ew += v;
            }
            Action::W(v) => {
                self.ew -= v;
            }
            Action::L(v) => {
                self.direction -= v;
            }
            Action::R(v) => {
                self.direction += v;
            }
            Action::F(v) => {
                self.moveby(*v as f64);
            }
        }
    }

    fn action2(&mut self, action: &Action) {
        match action {
            Action::N(v) => {
                self.wns += v;
            }
            Action::S(v) => {
                self.wns -= v;
            }
            Action::E(v) => {
                self.wew += v;
            }
            Action::W(v) => {
                self.wew -= v;
            }
            Action::L(v) => {
                self.rotate_wp_by((*v) as f64);
            }
            Action::R(v) => {
                self.rotate_wp_by(-(*v) as f64);
            }
            Action::F(v) => {
                self.ns += v * self.wns;
                self.ew += v * self.wew;
            }
        }
    }
}
#[derive(Debug, Clone)]
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl Action {
    fn new(s: String) -> Action {
        let action_type = match s.chars().nth(0) {
            Some(v) => v,
            None => ' ',
        };
        let action_value: i32 = s[1..].parse().unwrap();

        match action_type {
            'N' => return Action::N(action_value),
            'S' => return Action::S(action_value),
            'E' => return Action::E(action_value),
            'W' => return Action::W(action_value),
            'L' => return Action::L(action_value),
            'R' => return Action::R(action_value),
            'F' => return Action::F(action_value),
            _ => panic!(),
        };
    }
}

// Box error is used as a parent to handle the fact that there are
// multiple different errors that can be raised ehre
fn read_file(name: &str) -> Result<Vec<Action>, Box<dyn std::error::Error>> {
    // Open file using file open
    let f = File::open(name).expect("problems");
    let f = BufReader::new(f);
    // lines as a vector of strings
    let mut lines: Vec<Action> = Vec::new();

    // Read the file line by line
    for line in f.lines() {
        // Get the line
        let rline: String = line?;
        let rline: String = rline.trim().parse::<String>()?;
        // Push it to the array
        lines.push(Action::new(rline));
    }

    Ok(lines)
}
