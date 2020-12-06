use crate::read_lines;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn verify(field: &str, value: &str) -> bool {
    let between = |num: &str, min, max| {
        if let Ok(num) = num.parse::<u16>() {
            return num >= min && num <= max;
        }
        false
    };
    match field {
        "byr" => between(value, 1920, 2002),
        "iyr" => between(value, 2010, 2020),
        "eyr" => between(value, 2020, 2030),
        "hgt" => {
            let last = value.len();
            if &value[last - 2..last] == "cm" {
                between(&value[..last - 2], 150, 193)
            } else if &value[last - 2..last] == "in" {
                between(&value[..last - 2], 59, 76)
            } else {
                false
            }
        }
        "hcl" => {
            let color = Regex::new(r"^#(?:[0-9a-fA-F]){6}$").unwrap();
            color.is_match(value)
        }
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => {
            let passport = Regex::new(r"^\d{9}$").unwrap();
            passport.is_match(value)
        }
        "cid" => true,
        _ => false,
    }
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut num_valid = 0;
    let mut info = HashSet::new();
    let mut check = |info: &mut HashSet<String>| {
        let contains_all = info.len() == 8;
        let optional = info.len() == 7 && !info.contains("cid");
        if contains_all || optional {
            num_valid += 1;
        }
        info.clear();
    };
    for each in read_lines(filename)? {
        let line = each?;
        if line == "" {
            check(&mut info);
        } else {
            let pairs: Vec<&str> = line.split(" ").collect();
            for each in pairs {
                let pair: Vec<&str> = each.split(":").collect();
                info.insert(pair[0].to_owned());
            }
        }
    }
    // check again because the input might not have a new line at the end
    if info.len() > 0 {
        check(&mut info);
    }
    Ok(num_valid)
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut num_valid = 0;
    let mut info = HashMap::new();
    let mut check = |info: &mut HashMap<String, String>| {
        let optional = if let Some(_) = info.get("cid") {
            false
        } else {
            true
        };
        let mut verified_fields = 0;
        for (key, value) in info.iter() {
            if verify(key, value) {
                verified_fields += 1;
            }
        }
        if verified_fields == 8 || (verified_fields == 7 && optional) {
            num_valid += 1;
        }
        info.clear();
    };
    for each in read_lines(filename)? {
        let line = each?;
        if line == "" {
            check(&mut info);
        } else {
            let pairs: Vec<&str> = line.split(" ").collect();
            for each in pairs {
                let pair: Vec<&str> = each.split(":").collect();
                info.insert(pair[0].to_owned(), pair[1].to_owned());
            }
        }
    }
    // check again because the input might not have a new line at the end
    if info.len() > 0 {
        check(&mut info);
    }
    Ok(num_valid)
}
