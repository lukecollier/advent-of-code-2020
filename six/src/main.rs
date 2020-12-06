use std::collections::HashSet;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let input: usize = include_str!("input.txt")
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
    println!("amount of people who answered yes in each group {}", input);
}
