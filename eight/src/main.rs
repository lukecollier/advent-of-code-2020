use regex::Regex;
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

    fn run(program: Vec<Operation>) -> isize {
        let mut acc: isize = 0;
        let mut position: isize = 0;
        let mut stack: Vec<isize> = Vec::new();
        while !stack.contains(&(position + 1)) {
            let operation = program.get(position as usize); 
            match operation {
                Some(Operation::Acc(argument)) => {
                    position = position + 1;
                    acc = acc + argument;
                    stack.push(position);
                },
                Some(Operation::Jmp(argument)) => {
                    position = position + argument;
                    stack.push(position);
                },
                Some(Operation::Nop) => {
                    position = position + 1;
                    stack.push(position);
                },
                None => panic!("error")
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
            let output = Operation::run(program);
            println!("output: {}", &output);
        }
        None => eprintln!("usage: needs input file"),
    };
}
