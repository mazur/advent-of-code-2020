use std::collections::HashMap;

pub fn run() {
    let valid_passports = count_valid_passports(include_str!("input.txt"));

    println!("Day04 - Part 1: {}", valid_passports.0);
    println!("Day04 - Part 2: {}", valid_passports.1);
}

fn count_valid_passports(pp_str: &str) -> (i32, i32) {
    let pp_data: Vec<&str> = pp_str.split("\n\n")
                                    .collect();
    
    let mut valid_passports = 0;
    let mut valid_and_validated = 0;
    
    for pp in pp_data {
        let passport = Passport::from_str(pp);
    
        if passport.is_valid() {
            valid_passports += 1;
        }

        if passport.is_valid_and_validated() {
            valid_and_validated += 1;
        }
    }

    (valid_passports, valid_and_validated)
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

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn is_valid_ecl(&self) -> bool {
        match self.passport_data.get("ecl") {
            Some(ecl) => {
                match ecl.as_ref() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false
                }
            },
            None => false
        }
    }

    // hgt (Height) - a number followed by either cm or in:
    //  If cm, the number must be at least 150 and at most 193.
    //  If in, the number must be at least 59 and at most 76.
    fn is_valid_hgt(&self) -> bool {
        match self.passport_data.get("hgt") {
            Some(hgt) => Passport::validate_hgt(hgt),
            None => false
        }
    }

    fn validate_hgt(hgt: &str) -> bool {
        let (hgt, unit) = hgt.split_at(hgt.len()-2);
        let hgt: i32 = match hgt.parse() {
            Ok(h) => h,
            Err(_) => 0
        };

        match unit {
            "cm" => hgt >= 150 && hgt <= 193,
            "in" => hgt >= 59 && hgt <= 76,
            _ => false
        }
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn is_valid_byr(&self) -> bool {
        self.is_year_field_between("byr", 1920, 2002)
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn is_valid_iyr(&self) -> bool {
        self.is_year_field_between("iyr", 2010, 2020)
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn is_valid_eyr(&self) -> bool {
        self.is_year_field_between("eyr", 2020, 2030)
    }

    fn is_year_field_between(&self, field: &str, from: i32, to: i32) -> bool {
        match self.passport_data.get(field) {
            Some(year) => {
                let y: i32 = year.parse().expect(&format!("Could not parse {}", field));
                y >= from && y<= to
            },
            None => false
        }
    }

    fn is_valid_and_validated(&self) -> bool {
        /*
        hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        pid (Passport ID) - a nine-digit number, including leading zeroes.
        cid (Country ID) - ignored, missing or not.
        */
        
        self.is_valid_byr() && self.is_valid_iyr() && self.is_valid_eyr() && self.is_valid_hgt() && self.is_valid_ecl()
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

        assert_eq!(2, valid_passports.0);
    }

}
