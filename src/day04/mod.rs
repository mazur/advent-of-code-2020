use std::collections::HashMap;
use regex::Regex;

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

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn is_valid_hcl(&self) -> bool {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new("^#[0-9a-fA-F]{6}$").unwrap();
        }
        match self.passport_data.get("hcl") {
            Some(hgt) => HCL_RE.is_match(hgt),
            None => false
        }
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn is_valid_pid(&self) -> bool {
        lazy_static! {
            static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
        }
        match self.passport_data.get("pid") {
            Some(hgt) => PID_RE.is_match(hgt),
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
        self.is_valid_byr() && 
        self.is_valid_iyr() && 
        self.is_valid_eyr() && 
        self.is_valid_hgt() && 
        self.is_valid_ecl() &&
        self.is_valid_hcl() &&
        self.is_valid_pid()
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

    static INVALIDATED_PASSPORTS: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    static VALIDATED_PASSPORTS: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

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

    #[test]
    fn test_count_invalidated_passports() {
        let valid_passports = count_valid_passports(INVALIDATED_PASSPORTS);

        assert_eq!(0, valid_passports.1);
    }

    #[test]
    fn test_count_validated_passports() {
        let valid_passports = count_valid_passports(VALIDATED_PASSPORTS);

        assert_eq!(4, valid_passports.1);
    }

    #[test]
    fn test_invalidated_passport_pid() {
        let passport = Passport::from_str("pid:08749970004 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#62ffff");

        assert_eq!(false, passport.is_valid_and_validated());
    }

    #[test]
    fn test_invalidated_passport_hcl() {
        let passport = Passport::from_str("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2faa");

        assert_eq!(false, passport.is_valid_and_validated());
    }
}
