use regex::Captures;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq, Clone)]
struct RawBag {
    pub name: String,
    pub can_contain: HashMap<String, usize>,
}

impl RawBag {
    pub fn contains_bag(&self, bag: &str) -> bool {
        self.can_contain.contains_key(bag)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RecBag<'a> {
    pub name: String,
    pub can_contain: Vec<&'a RecBag<'a>>,
}

impl<'a> RecBag<'a> {
    pub fn from<'b>(name: String, contains: HashSet<String>) -> RecBag<'b> {
        panic!("")
    }

    pub fn contains_bag(&self, str: &str) -> bool {
        str == self.name
            || self
                .can_contain
                .iter()
                .map(|bag| self.contains_bag(&bag.name))
                .find(|res| res == &true)
                .is_some()
    }
}

fn parse_inner_bags(line: &str) -> Result<HashMap<String, usize>, String> {
    let contained_bags_re = Regex::new(r"(?P<amount>\d+) (?P<name>[\w\s]*) bags?[,\.]")
        .map_err(|_| "could not get contained bags regex")?;
    Ok(contained_bags_re
        .captures_iter(line)
        .map(|captures: Captures| -> Result<(String, usize), String> {
            let amount: usize = captures
                .name("amount")
                .ok_or("could not find amount".to_string())
                .and_then(|amount| {
                    amount
                        .as_str()
                        .parse::<usize>()
                        .map_err(|_| "could not parse bag amount".to_string())
                })?;
            let name: &str = captures
                .name("name")
                .map(|name| name.as_str())
                .ok_or("could not find name".to_string())?;
            Ok((name.to_string(), amount))
        })
        .filter_map(Result::ok)
        .collect::<HashMap<String, usize>>())
}

fn parse_bags_from_line(line: &str) -> Result<RawBag, String> {
    let bag_name_re = Regex::new(r"(?P<name>\w+) bags contain (?P<rest>.*)")
        .map_err(|_| "could not get bag name regex")?;

    let initial_captures = bag_name_re
        .captures(&line)
        .ok_or("initial capture".to_string())?;
    let bag_name = initial_captures
        .name("name")
        .map(|capture| capture.as_str())
        .ok_or("bag name not found".to_string())?;

    initial_captures
        .name("rest")
        .map(|capture| capture.as_str())
        .map(|inner_bags| parse_inner_bags(inner_bags))
        .map(|can_contain| RawBag {
            name: bag_name.to_string(),
            can_contain: can_contain.unwrap(),
        })
        .ok_or("could not find the rest of the bags".to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_path) => {
            let input = fs::read_to_string(file_path).expect(&format!(
                "usage: file at {} failed to read to string",
                &file_path
            ));
            let bags = input
                .split(&LINE_ENDING)
                .map(|line| parse_bags_from_line(&line))
                .filter_map(Result::ok)
                .collect::<Vec<RawBag>>();
            let direct_shiny_gold_bags = bags
                .into_iter()
                .clone()
                .filter(|bag| bag.contains_bag("shiny gold"))
                .collect::<Vec<RawBag>>();
            println!(
                "bag colours that contain shiny gold {:?}",
                &direct_shiny_gold_bags
            );
        }
        None => eprintln!("usage: needs input file"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_bags() {
        let input = "1 bright white bag, 2 muted yellow bags.";
        let result = parse_inner_bags(input);
        let expected = vec![
            ("bright white".to_string(), 1),
            ("muted yellow".to_string(), 2),
        ]
        .into_iter()
        .collect::<HashMap<String, usize>>();
        assert_eq!(Ok(expected), result);
    }
}
