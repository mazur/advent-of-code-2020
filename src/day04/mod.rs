use std::collections::HashMap;

pub fn run() {
    let valid_passports = count_valid_passports(include_str!("input.txt"));

    println!("Day04 - Part 1: {}", valid_passports);
}

fn count_valid_passports(pp_str: &str) -> i32 {
    let pp_data: Vec<&str> = pp_str.split("\n\n")
                                    .collect();
    
    let mut valid_passports = 0;
    
    for pp in pp_data {
        let passport = Passport::from_str(pp);
    
        if passport.is_valid() {
            valid_passports += 1;
        }
    }

    valid_passports
}

struct Passport {
    passport_data: HashMap<String, String>
}

impl Passport {
    fn from_str (pp_str: &str) -> Self {
        let mut data = HashMap::new();

        let values = pp_str.trim().split(|c| c == ' ' || c == '\n');

        for val in values {
            let key_val: Vec<&str> = val.split(":").collect();

            data.insert(String::from(key_val[0]), String::from(key_val[1]));
        }

        Self { passport_data: data }
    }

    fn is_valid(&self) -> bool {
        self.passport_data.len() > 7 || 
        (self.passport_data.len() > 6 && !self.passport_data.contains_key("cid"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static VALID_PASSPORT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

    static VALID_NORTH_POLE_CRED: &str = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";

    #[test]
    fn test_split_string_on_empty_line() {
        let passports: Vec<&str> = TEST_INPUT.split("\n\n")
                                                .collect();

        assert_eq!(4, passports.len());
    }

    #[test]
    fn test_valid_passport_from_string() {
        let passport = Passport::from_str(VALID_PASSPORT);

        assert!(passport.is_valid());
    }

    #[test]
    fn test_north_pole_cred_from_string() {
        let passport = Passport::from_str(VALID_NORTH_POLE_CRED);

        assert!(passport.is_valid());
    }

    #[test]
    fn test_count_valid_passports() {
        let valid_passports = count_valid_passports(TEST_INPUT);

        assert_eq!(2, valid_passports);
    }

}
