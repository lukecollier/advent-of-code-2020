#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let input = include_str!("input.txt");
    let (value, _) = input
        .split(LINE_ENDING)
        .filter(|line| !line.is_empty())
        .map(|line: &str| {
            line.chars()
                .map(|character: char| match character {
                    '#' => Ok(1),
                    '.' => Ok(0),
                    _ => Err("cannot understand character"),
                })
                .filter_map(Result::ok)
                .collect::<Vec<u8>>()
        })
        .fold((0, 0), |(acc, pos), line| {
            let wrapped_pos = pos % line.len();
            let value = line.get(wrapped_pos).unwrap_or(&0);
            (acc + value, pos + 3)
        });
    println!("we would hit {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_validate_updated_is_correct() {
        assert_eq!(true, true);
    }
}
