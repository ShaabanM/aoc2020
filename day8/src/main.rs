use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

// Instruction struct to represents tha valid isntructions
#[derive(Debug, Clone)]
enum Instruction {
    Acc { value: i32, executed: bool },
    Jmp { value: i32, executed: bool },
    Nop { value: i32, executed: bool },
}

// Implementation of a constructor for the instruction
impl Instruction {
    fn new(s: String) -> Instruction {
        let temp: Vec<String> = s.split(" ").map(|x| x.trim().to_string()).collect();
        if temp[0] == "acc" {
            Instruction::Acc {
                value: temp[1].parse::<i32>().unwrap(),
                executed: false,
            }
        } else if temp[0] == "jmp" {
            Instruction::Jmp {
                value: temp[1].parse::<i32>().unwrap(),
                executed: false,
            }
        } else {
            Instruction::Nop {
                value: temp[1].parse::<i32>().unwrap(),
                executed: false,
            }
        }
    }
}

// Struct to represent a program and its state
#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    next: usize,
    acc: i32,
}

// Constructor for the program
impl Program {
    fn new() -> Program {
        Program {
            instructions: Vec::new(),
            next: 0,
            acc: 0,
        }
    }
}

// Function that runs a program and returns the modified program and an exit status
// exit status false => a repeat instruction occured
// exit status true => no repeat instruction occured
fn run(mut program: Program) -> (bool, Program) {
    loop {
        match program.instructions[program.next] {
            // Handle acc instruction
            Instruction::Acc { value, executed } => {
                program.instructions[program.next] = Instruction::Acc {
                    value: value,
                    executed: true,
                };
                program.acc += value;
                program.next += 1;

                // If next has executed before return with exit state false
                if executed {
                    return (false, program);
                }
                // if next is the end of the program return with exit state true
                else if program.next == program.instructions.len() {
                    return (true, program);
                }
            }
            // Handle jump isntruction
            Instruction::Jmp { value, executed } => {
                program.instructions[program.next] = Instruction::Jmp {
                    value: value,
                    executed: true,
                };
                program.next = (value + program.next as i32) as usize;
                // If next has executed before return with exit state false
                if executed {
                    return (false, program);
                }
                // if next is the end of the program return with exit state true
                else if program.next == program.instructions.len() {
                    return (true, program);
                }
            }
            // Handle nop instruction
            Instruction::Nop { value, executed } => {
                program.instructions[program.next] = Instruction::Nop {
                    value: value,
                    executed: true,
                };
                program.next += 1;
                // If next has executed before return with exit state false
                if executed {
                    return (false, program);
                }
                // if next is the end of the program return with exit state true
                else if program.next == program.instructions.len() {
                    return (true, program);
                }
            }
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program = Program::new();
    let lines = read_file("input")?;

    for line in lines {
        program.instructions.push(Instruction::new(line));
    }

    for (i, instruction) in program.instructions.iter().enumerate() {
        match instruction {
            Instruction::Acc { value, executed } => {
                //do nothing
            }
            Instruction::Jmp { value, executed } => {
                let mut new_program = program.clone();
                new_program.instructions[i] = Instruction::Nop {
                    value: *value,
                    executed: *executed,
                };
                let out = run(new_program);
                if out.0 {
                    println!("acc = {}", out.1.acc)
                }
            }
            Instruction::Nop { value, executed } => {
                let mut new_program = program.clone();
                new_program.instructions[i] = Instruction::Jmp {
                    value: *value,
                    executed: *executed,
                };
                let out = run(new_program);
                if out.0 {
                    println!("acc = {}", out.1.acc)
                }
            }
        }
    }

    let a = run(program.clone());
    println!("{}", a.0);

    Ok(())
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
