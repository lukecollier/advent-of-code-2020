#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let input: &str = include_str!("input.txt");
    let numbers: Vec<i32> = input
        .split(LINE_ENDING)
        .map(|number_str| number_str.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    let mut part_one_result = 0;
    let mut part_two_result = 0;
    for first_num in &numbers {
        for second_num in &numbers {
            if (first_num + second_num) == 2020 {
                part_one_result = first_num * second_num
            };
            for third_num in &numbers {
                if (first_num + second_num + third_num) == 2020 {
                    part_two_result = first_num * second_num * third_num
                };
            }
        }
    }

    println!(
        "two numbers that sum to 2020 then multiplied are := {}, three numbers that sum to 2020 then multiplied are := {}",
        part_one_result,
        part_two_result
    );
}
