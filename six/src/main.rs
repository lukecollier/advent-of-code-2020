use itertools::Itertools;
use std::collections::HashSet;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let input: &str = include_str!("input.txt");
    let part_one: usize = input
        .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
        .filter(|str| !str.is_empty())
        .map(|str| {
            str.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<char>>()
                .into_iter()
                .count()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sum();

    let intersections = input
        .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
        .filter(|line| !line.is_empty())
        .map(|group| {
            let answers: Vec<HashSet<char>> = group
                .split(LINE_ENDING)
                .map(|answer| answer.chars().collect::<HashSet<char>>())
                .collect();

            answers
                .into_iter()
                .fold1(|acc, answer| {
                    acc.intersection(&answer)
                        .map(|x| x.clone())
                        .collect::<HashSet<char>>()
                })
                .unwrap_or(HashSet::new())
        })
        .collect::<Vec<_>>();

    let part_two: usize = intersections
        .into_iter()
        .map(|chars| chars.into_iter().count())
        .sum();
    println!(
        "amount of people who answered yes in each group {}\npart two: {:?}",
        part_one, part_two
    );
}
