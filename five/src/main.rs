use itertools::Itertools;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq)]
struct BoardingPass {
    pub column: usize,
    pub row: usize,
}

impl BoardingPass {
    pub fn seat_id(&self) -> usize {
        (usize::from(self.row) * 8) + usize::from(self.column)
    }
}

fn binary_search_index(bits: Vec<bool>) -> usize {
    let max_num: usize = 2_usize.pow(bits.len() as u32);
    let (min, _) = bits.into_iter().fold((0, max_num), |(min, max), bit| {
        let range = max - min;
        if bit {
            (min + (range / 2), max) // true (upper)
        } else {
            (min, max - (range / 2)) // false (lower)
        }
    });
    min
}

fn main() {
    let input = include_str!("input.txt");
    let boarding_passes = input
        .split(LINE_ENDING)
        .filter(|str| !str.is_empty())
        .map(|boarding_pass| {
            let (row_chars, column_chars): (Vec<char>, Vec<char>) = boarding_pass
                .chars()
                .partition(|character: &char| ['F', 'B'].contains(character));
            let row_binary: Vec<bool> = row_chars
                .iter()
                .map(|&character| match character {
                    'B' => Ok(true),
                    'F' => Ok(false),
                    _ => Err("unknown row character"),
                })
                .filter_map(Result::ok)
                .collect();
            let column_binary: Vec<bool> = column_chars
                .iter()
                .map(|&character| match character {
                    'R' => Ok(true),
                    'L' => Ok(false),
                    _ => Err("unknown column character"),
                })
                .filter_map(Result::ok)
                .collect();
            BoardingPass {
                row: binary_search_index(row_binary),
                column: binary_search_index(column_binary),
            }
        })
        .collect::<Vec<BoardingPass>>();
    let max_seat_id = (&boarding_passes)
        .into_iter()
        .map(|boarding_pass| boarding_pass.seat_id())
        .max()
        .unwrap_or_else(|| panic!("failed to find maximum seat id"));
    let (_, missing_id) = (&boarding_passes)
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&b.seat_id(), &a.seat_id()))
        .fold(
            (max_seat_id + 1, max_seat_id),
            |(acc, cur), boarding_pass| {
                let delta = acc - boarding_pass.seat_id();
                if delta == 2 {
                    (boarding_pass.seat_id(), boarding_pass.seat_id() + 1)
                } else {
                    (boarding_pass.seat_id(), cur)
                }
            },
        );
    println!(
        "max seat id := {}, missing id := {}",
        max_seat_id, missing_id
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_binary_search_index_min() {
        let input = vec![false, false, false, false, false, false, false];
        let output = binary_search_index(input);
        assert_eq!(0, output);
    }

    #[test]
    fn can_find_binary_search_index_max() {
        let input = vec![true, true, true, true, true, true, true];
        let output = binary_search_index(input);
        assert_eq!(127, output);
    }

    #[test]
    fn can_find_binary_search_index_other() {
        let input = vec![true, false, true];
        let output = binary_search_index(input);
        assert_eq!(5, output);
    }

    #[test]
    fn can_find_binary_search_index() {
        let input = vec![false, true, false, true, true, false, false];
        let output = binary_search_index(input);
        assert_eq!(44, output);
    }
}
