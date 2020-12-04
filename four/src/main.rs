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
    pub height: String,
    pub hair_colour: String,
    pub eye_colour: String,
    pub passport_id: String,
    pub country_id: Option<String>,
}

fn parse_entry(entry: &str) -> Result<Passport, &str> {
    let key_value_re = Regex::new(r"(?P<key>\S*):(?P<value>\S*)\b").unwrap();
    let details: HashMap<String, String> = key_value_re
        .captures_iter(entry)
        .map(|key_value| (key_value["key"].to_string(), key_value["value"].to_string()))
        .collect();

    let birth_year: u16 = details
        .get(BIRTH_YEAR)
        .ok_or("could not find birth year")
        .and_then(|byr| byr.parse::<u16>().map_err(|_| "could not parse into year"))?;

    let issued_year: u16 = details
        .get(ISSUED_YEAR)
        .ok_or("could not find issued year")
        .and_then(|byr| byr.parse::<u16>().map_err(|_| "could not parse into year"))?;

    let expiration_year: u16 = details
        .get(EXPIRATION_YEAR)
        .ok_or("could not find expiration year")
        .and_then(|byr| byr.parse::<u16>().map_err(|_| "could not parse into year"))?;

    let height: String = details
        .get(HEIGHT)
        .map(|str| str.to_string())
        .ok_or("could not find height")?;

    let hair_colour: String = details
        .get(HAIR_COLOUR)
        .map(|str| str.to_string())
        .ok_or("could not find hair colour")?;

    let eye_colour: String = details
        .get(EYE_COLOUR)
        .map(|str| str.to_string())
        .ok_or("could not find eye colour")?;

    let passport_id: String = details
        .get(PASSPORT_ID)
        .map(|str| str.to_string())
        .ok_or("could not find passport id")?;

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
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let result = parse_input(input);
        assert_eq!(2, result.len());
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
            height: "183cm".to_string(),
        };
        let result = parse_entry(input);
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn fails_to_parse() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        let result = parse_entry(input);
        assert_eq!(Err("could not find height"), result);
    }

    #[test]
    fn can_parse_passport_entry_with_hash_and_new_line() {
        let input = "eyr:2025
            hgt:161cm iyr:1962
            pid:394421140
            ecl:gry
            cid:209 hcl:#efcc98 byr:2001";
        let expected = Passport {
            expiration_year: 2025,
            issued_year: 1962,
            passport_id: 394421140.to_string(),
            birth_year: 2001,
            hair_colour: "#efcc98".to_string(),
            country_id: Some(209.to_string()),
            eye_colour: "gry".to_string(),
            height: "161cm".to_string(),
        };
        let result = parse_entry(input);
        assert_eq!(Ok(expected), result);
    }
}
