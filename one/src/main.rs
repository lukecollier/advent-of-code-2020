use itertools::Itertools;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let input: &str = include_str!("input.txt");
    let number_list = input
        .split(LINE_ENDING)
        .map(|number_str| number_str.parse::<i32>())
        .filter_map(Result::ok);

    let two_combinations = number_list
        .clone()
        .combinations(2)
        .find(|combinations| combinations.iter().sum::<i32>() == 2020)
        .map(|combinations| combinations.iter().product::<i32>());

    let three_combinations = number_list
        .clone()
        .combinations(3)
        .find(|combinations| combinations.iter().sum::<i32>() == 2020)
        .map(|combinations| combinations.iter().product::<i32>());

    // let three_combinations: Vec<Vec<i32>> = number_list.combinations(3).collect();

    println!(
        "two numbers that sum to 2020 then multiplied are := {}, three numbers that sum to 2020 then multiplied are := {}",
        two_combinations.unwrap(),
        three_combinations.unwrap()
    );
}
