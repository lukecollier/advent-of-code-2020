use regex::Regex;
use itertools::*;
use std::env;
use std::fs;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop
}

impl Operation {
    fn parse_line(str: &str) -> Result<Operation, &str> {
        let re = Regex::new(r"(?P<operation>\w+) (?P<argument>[-+]\d+)").map_err(|_| "could not get regex")?;
        let captures = re.captures(str).ok_or("found no captures")?;
        let operation = &captures["operation"];
        let argument = &captures["argument"].parse::<isize>().map_err(|_| "couldn't parse argument")?;

        match (operation, argument) {
            ("acc", num) => Ok(Operation::Acc(num.clone())),
            ("jmp", num) => Ok(Operation::Jmp(num.clone())),
            ("nop", _) => Ok(Operation::Nop),
            (_, _) => Err("unrecognized operation"),
        }
    }

    fn run_fix(program: &Vec<Operation>) -> isize {
        let mut position: isize = 0;
        let mut path: Vec<isize> = Vec::new();
        while !path.contains(&(position + 1)) {
            let operation = program.get(position as usize); 
            match operation {
                Some(Operation::Acc(_)) => {
                    position = position + 1;
                },
                Some(Operation::Jmp(argument)) => {
                    path.push(position);
                    position = position + argument;
                },
                Some(Operation::Nop) => {
                    path.push(position);
                    position = position + 1;
                },
                None => panic!("error")
            }
        }
        // if final position is a Nop change to Jmp and jump the amount of Jmp's ahead
        let (change_position, change) = path.iter().sorted().rev().find_map(|pos| {
            let length = program.len() as usize;
            let position = pos.clone() as usize;
            let operation = program.get(pos.clone() as usize);
            let result: Option<(isize, Operation)> = match operation {
                Some(Operation::Jmp(_)) => {
                    let remaining_program = &program[(position + 1)..length].into_iter().filter(|op| match op {
                        Operation::Jmp(_) => true,
                        _ => false,
                    }).count();
                    if remaining_program == &0 {
                        Some((position as isize, Operation::Nop))
                    } else {
                        None
                    }
                },
                Some(Operation::Nop) => {
                    let jump_to_final = program[(position + 1)..length].into_iter().enumerate().rev().find_map(|(idx, op)| match op {
                        Operation::Jmp(_) => Some((position as isize, Operation::Jmp(idx as isize))),
                        _ => None,
                    });
                    jump_to_final
                },
                _ => panic!("error"),
            };
            result
        }).unwrap();
        let mut fixed_program = program.clone();
        std::mem::replace(&mut fixed_program[change_position as usize], change);
        Operation::run(&fixed_program)
    }

    fn run(program: &Vec<Operation>) -> isize {
        let mut acc: isize = 0;
        let mut position: isize = 0;
        let mut path: Vec<isize> = Vec::new();
        while !path.contains(&(position + 1)) {
            let operation = program.get(position as usize); 
            match operation {
                Some(Operation::Acc(argument)) => {
                    position = position + 1;
                    acc = acc + argument;
                    path.push(position);
                },
                Some(Operation::Jmp(argument)) => {
                    position = position + argument;
                    path.push(position);
                },
                Some(Operation::Nop) => {
                    position = position + 1;
                    path.push(position);
                },
                None => {
                    path.push(position + 1);
                }
            }
        }
        acc
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_path) => {
            let msg = &format!("usage: file at {} failed to read to string", &file_path);
            let input = fs::read_to_string(file_path).expect(msg);
            let program = input
                .split(&LINE_ENDING)
                .map(|str| Operation::parse_line(str))
                .filter_map(Result::ok)
                .collect::<Vec<Operation>>();
            let output = Operation::run(&program);
            let part_two_output = Operation::run_fix(&program);
            println!("output: {} part_two_output: {}", &output, &part_two_output);
        }
        None => eprintln!("usage: needs input file"),
    };
}
