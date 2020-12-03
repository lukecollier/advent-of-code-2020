#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn count_tree_hits(map: &Vec<Vec<u8>>, right: &usize, down: &usize) -> u8 {
    let (result, _) = map
        .into_iter()
        .step_by(*down)
        .fold((0, 0), |(acc, pos), line| {
            let wrapped_pos = pos % line.len();
            let value = line.get(wrapped_pos).unwrap_or(&0);
            (acc + value, pos + right)
        });
    result
}

fn main() {
    let input = include_str!("input.txt");
    let map = input
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
        .collect::<Vec<Vec<u8>>>();
    let part_one_trees_hit = count_tree_hits(&map, &3, &1);
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part_two_trees_hit = slopes.iter().fold(1, |acc, (right, down)| {
        acc * usize::from(count_tree_hits(&map, right, down))
    });
    println!("part one we would hit {}", part_one_trees_hit);
    println!("part two we would hit {}", part_two_trees_hit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_validate_updated_is_correct() {
        assert_eq!(true, true);
    }
}
