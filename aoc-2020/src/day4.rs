use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Passport {
    fields: HashMap<String, String>,
}

#[aoc_generator(day4)]
pub fn generate(s: &str) -> Vec<Passport> {
    let mut passports = vec![];
    let mut new_passport = HashMap::new();
    for line in s.lines() {
        if line.is_empty() {
            passports.push(Passport {
                fields: new_passport.clone(),
            });
            new_passport.clear();
        }
        new_passport.extend(line.split_ascii_whitespace().filter_map(|field| {
            let mut split = field.split(':');
            Some((split.next()?.to_owned(), split.next()?.to_owned()))
        }))
    }

    passports.push(Passport {
        fields: new_passport,
    });

    passports
}

#[aoc(day4, part1)]
pub fn day4_part1(passports: &[Passport]) -> usize {
    let required_fields = &[
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        // "cid",
    ];

    passports
        .iter()
        .filter(|p| required_fields.iter().all(|&f| p.fields.contains_key(f)))
        .count()
}

#[aoc(day4, part2)]
pub fn day4_part2(passports: &[Passport]) -> usize {
    fn validate(passport: &Passport) -> bool {
        let required_fields = &[
            "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
            // "cid",
        ];

        passport
            .fields
            .iter()
            .filter_map(|(key, value)| {
                let valid = match key.as_str() {
                    "byr" => (1920i32..=2002).contains(&value.parse().ok()?),
                    "iyr" => (2010i32..=2020).contains(&value.parse().ok()?),
                    "eyr" => (2020i32..=2030).contains(&value.parse().ok()?),
                    "hgt" => {
                        let (v, unit) = value.split_at(value.len().saturating_sub(2));
                        match unit {
                            "cm" => (150i32..=193).contains(&v.parse().ok()?),
                            "in" => (59i32..=76).contains(&v.parse().ok()?),
                            _ => false,
                        }
                    }
                    "hcl" => {
                        let mut chars = value.chars();
                        let starts_with_hash = chars.next()? == '#';
                        let hex_digits = chars
                            .by_ref()
                            .take(6)
                            .filter(|c| matches!(c, '0'..='9' | 'a'..='f'))
                            .count()
                            == 6;
                        let six_digits = chars.next().is_none();
                        starts_with_hash && hex_digits && six_digits
                    }
                    "ecl" => matches!(
                        value.as_str(),
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                    ),
                    "pid" => value.len() == 9 && value.chars().all(|c| matches!(c, '0'..='9')),
                    _ => false,
                };

                if valid {
                    Some(())
                } else {
                    // println!("invalid: {}: {}", key, value);
                    None
                }
            })
            .count()
            == required_fields.len()
    }

    passports
        .iter()
        .filter(|p| {
            let valid = validate(p);
            if false {
                print!("{}: ", if valid { "valid" } else { "invalid" });
                println!("{:#?}", p);
            }
            valid
        })
        .count()
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID_INPUT: &str = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID_INPUT: &str = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    // fn get_example_data() -> Vec<Passport> {
    //     todo!()
    // }

    // #[test]
    // fn generate() {
    //     assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    // }

    #[test]
    fn day4_part1() {
        assert_eq!(super::day4_part1(&super::generate(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn day4_part2_invalid() {
        assert_eq!(super::day4_part2(&super::generate(INVALID_INPUT)), 0);
    }

    #[test]
    fn day4_part2_valid() {
        assert_eq!(super::day4_part2(&super::generate(VALID_INPUT)), 4);
    }
}
