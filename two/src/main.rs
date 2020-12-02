use regex::Regex;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq)]
struct Verification {
    pub max: Box<u8>,
    pub min: Box<u8>,
    pub character: Box<char>,
}

impl Verification {
    pub fn is_valid<'a>(&self, string: &'a str) -> bool {
        let amount = string
            .chars()
            .filter(|character| *self.character == *character)
            .count();
        (usize::from(*self.min)..=usize::from(*self.max)).contains(&amount)
    }

    pub fn is_valid_updated<'a>(&self, string: &'a str) -> bool {
        let characters = string.chars().collect::<Vec<char>>();
        let pos_one_is_character: Option<bool> = characters
            .get(usize::from(*self.min - 1))
            .map(|character| *self.character == *character);
        let pos_two_is_character: Option<bool> = characters
            .get(usize::from(*self.max - 1))
            .map(|character| *self.character == *character);

        pos_one_is_character.unwrap_or(false) ^ pos_two_is_character.unwrap_or(false)
    }
}

fn parse<'a>(str: &'a str) -> Result<(Verification, String), &'a str> {
    let re =
        Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<character>\w{1}): (?P<password>\w*)").unwrap();
    let caps = re.captures(str).ok_or("failed to get captures")?;
    let max = caps["max"]
        .parse::<u8>()
        .map(|result| Box::new(result.clone()))
        .map_err(|_| "no max!")?;

    let min = caps["min"]
        .parse::<u8>()
        .map(|result| Box::new(result.clone()))
        .map_err(|_| "no min!")?;

    let character = caps["character"]
        .chars()
        .next()
        .ok_or("no character")
        .map(|result| Box::new(result.clone()))
        .map_err(|_| "no min!")?;

    let password = caps["password"].to_string();

    let verification = Verification {
        max,
        min,
        character,
    };
    Ok((verification, password))
}

fn main() {
    let input = include_str!("input.txt");
    let part_one_result = input
        .split(LINE_ENDING)
        .map(|string: &str| parse(string))
        .filter_map(Result::ok)
        .filter(|(validation, password)| validation.is_valid(password))
        .count();

    let part_two_result = input
        .split(LINE_ENDING)
        .map(|string: &str| parse(string))
        .filter_map(Result::ok)
        .filter(|(validation, password)| validation.is_valid_updated(password))
        .count();

    println!(
        "part_one:={}, part_two:={}",
        part_one_result, part_two_result
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_validate_updated_is_correct() {
        let input = "aab";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid_updated(input);
        assert_eq!(true, result);
    }

    #[test]
    fn can_validate_updated_is_incorrect_at_end() {
        let input = "aaa";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid_updated(input);
        assert_eq!(false, result);
    }

    #[test]
    fn when_validate_updated_has_neither_is_false() {
        let input = "bab";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid_updated(input);
        assert_eq!(false, result);
    }

    #[test]
    fn when_validate_updated_has_last_is_true() {
        let input = "baa";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid_updated(input);
        assert_eq!(true, result);
    }

    #[test]
    fn can_validate_updated_is_correct_when_no_char_exists_at_max() {
        let input = "aa";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid_updated(input);
        assert_eq!(true, result);
    }

    #[test]
    fn can_validate_when_at_upper_boundary() {
        let input = "aaa";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid(input);
        assert_eq!(true, result);
    }

    #[test]
    fn can_validate_when_at_lower_boundary() {
        let input = "a";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid(input);
        assert_eq!(true, result);
    }

    #[test]
    fn empty_input_always_fails() {
        let input = "";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid(input);
        assert_eq!(false, result);
    }

    #[test]
    fn can_validate_password_is_correct() {
        let input = "abcde";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('a'),
        }
        .is_valid(input);
        assert_eq!(true, result);
    }

    #[test]
    fn can_validate_password_is_incorrect() {
        let input = "cdefg";
        let result = Verification {
            max: Box::new(3),
            min: Box::new(1),
            character: Box::new('b'),
        }
        .is_valid(input);
        assert_eq!(false, result);
    }

    #[test]
    fn can_parse_input() {
        let expected = (
            Verification {
                max: Box::new(3),
                min: Box::new(1),
                character: Box::new('a'),
            },
            "abcde".to_string(),
        );
        let input = "1-3 a: abcde";
        let result = parse(input);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn can_parse_input_with_large_numbers() {
        let expected = (
            Verification {
                max: Box::new(200),
                min: Box::new(30),
                character: Box::new('c'),
            },
            "ccccccccc".to_string(),
        );
        let input = "30-200 c: ccccccccc";
        let result = parse(input);
        assert_eq!(Ok(expected), result);
    }
}
