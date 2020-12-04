use regex::Regex;
use std::collections::HashMap;

const BIRTH_YEAR: &str = "byr";
const ISSUED_YEAR: &str = "iyr";
const EXPIRATION_YEAR: &str = "eyr";
const HEIGHT: &str = "hgt";
const HAIR_COLOUR: &str = "hcl";
const EYE_COLOUR: &str = "ecl";
const PASSPORT_ID: &str = "pid";
const COUNTRY_ID: &str = "cid";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq)]
struct Passport {
    pub birth_year: u16,
    pub issued_year: u16,
    pub expiration_year: u16,
    pub height: u16,
    pub hair_colour: String,
    pub eye_colour: String,
    pub passport_id: String,
    pub country_id: Option<String>,
}

fn is_between(num: u16, min: u16, max: u16) -> Result<u16, String> {
    if num >= min && num <= max {
        Ok(num)
    } else {
        Err(format!("year {} was out of bounds ({},{})", num, min, max))
    }
}

fn validate_year(str: String, min: u16, max: u16) -> Result<u16, String> {
    str.parse::<u16>()
        .map_err(|_| format!("could not parse {} into a year", str))
        .and_then(|year: u16| is_between(year, min, max))
}

fn validate_height(str: String) -> Result<u16, String> {
    match str.split_at(str.len() - 2) {
        (magnitude, "cm") => magnitude
            .parse::<u16>()
            .map_err(|_| format!("could not parse {} to u16", magnitude))
            .and_then(|num| is_between(num, 150, 193)),
        (magnitude, "in") => magnitude
            .parse::<u16>()
            .map_err(|_| format!("could not parse {} to u16", magnitude))
            .and_then(|num| is_between(num, 59, 76)),
        (_, measurement) => Err(format!("Invalid measurement {}", measurement)),
    }
}

fn validate_hair_colour(string: String) -> Result<String, String> {
    let hex_re = Regex::new(r"#([a-f]|[0-9]){1,6}").unwrap();
    hex_re
        .captures(&string)
        .ok_or(format!("{} is not a valid hex string", string))
        .map(|_| string.clone())
        .and_then(|output| {
            if output.len() == 7 {
                Ok(output)
            } else {
                Err("hex string is not 7 characters".to_string())
            }
        })
}

fn validate_eye_colour(string: String) -> Result<String, String> {
    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&string.as_str()) {
        Ok(string)
    } else {
        Err(format!("{} is not a recognised eye colour", string))
    }
}

fn validate_passport_id(string: String) -> Result<String, String> {
    let hex_re = Regex::new(r"^[0-9]{9,9}$").unwrap();
    hex_re
        .captures(&string)
        .ok_or(format!("{} is not a valid passport id", string))
        .map(|_| string.clone())
}

fn parse_entry(entry: &str) -> Result<Passport, String> {
    let key_value_re = Regex::new(r"(?P<key>\S*):(?P<value>\S*)\b").unwrap();
    let details: HashMap<String, String> = key_value_re
        .captures_iter(entry)
        .map(|key_value| (key_value["key"].to_string(), key_value["value"].to_string()))
        .collect();

    let birth_year: u16 = details
        .get(BIRTH_YEAR)
        .ok_or("could not find birth year".to_string())
        .and_then(|str| validate_year(str.to_string(), 1920, 2002))?;

    let issued_year: u16 = details
        .get(ISSUED_YEAR)
        .ok_or("could not find issued year".to_string())
        .and_then(|str| validate_year(str.to_string(), 2010, 2020))?;

    let expiration_year: u16 = details
        .get(EXPIRATION_YEAR)
        .ok_or("could not find expiration year".to_string())
        .and_then(|str| validate_year(str.to_string(), 2020, 2030))?;

    let height: u16 = details
        .get(HEIGHT)
        .ok_or("could not find height".to_string())
        .and_then(|str| validate_height(str.to_string()))?;

    let hair_colour: String = details
        .get(HAIR_COLOUR)
        .ok_or("could not find hair colour".to_string())
        .and_then(|str| validate_hair_colour(str.to_string()))?;

    let eye_colour: String = details
        .get(EYE_COLOUR)
        .ok_or("could not find eye colour".to_string())
        .and_then(|str| validate_eye_colour(str.to_string()))?;

    let passport_id: String = details
        .get(PASSPORT_ID)
        .ok_or("could not find passport id".to_string())
        .and_then(|str| validate_passport_id(str.to_string()))?;

    let country_id: Option<String> = details.get(COUNTRY_ID).map(|str| str.to_string());

    Ok(Passport {
        birth_year,
        issued_year,
        expiration_year,
        height,
        hair_colour,
        eye_colour,
        passport_id,
        country_id,
    })
}

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
        .map(|passport_entry| parse_entry(passport_entry))
        .filter_map(Result::ok)
        .collect::<Vec<Passport>>()
}

fn main() {
    let input: &str = include_str!("input.txt");
    let passports = parse_input(input);

    println!("part one valid passports {}", passports.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_multiple_entries() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let result = parse_input(input);
        assert_eq!(4, result.len());
    }

    #[test]
    fn can_parse_passport_entry() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected = Passport {
            expiration_year: 2020,
            issued_year: 2017,
            passport_id: 860033327.to_string(),
            birth_year: 1937,
            hair_colour: "#fffffd".to_string(),
            country_id: Some(147.to_string()),
            eye_colour: "gry".to_string(),
            height: 183,
        };
        let result = parse_entry(input);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn fails_to_parse() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        let result = parse_entry(input);
        assert_eq!(Err("could not find height".to_string()), result);
    }
}
