use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use regex::RegexSet;

fn passport_is_valid(passport: &String) -> bool {
    let patterns = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for pattern in patterns {
        if !passport.contains(pattern) {
            return false;
        }
    }

    true
}

fn passport_is_valid2(passport: &String) -> bool {
    let set = RegexSet::new(&[
        r"(byr):(19[2-9][0-9]|200[0-2])",
        r"(iyr):(201[0-9]|2020)",
        r"(eyr):(202[0-9]|2030)",
        r"hgt:(1[5-8][0-9]|19[0-3])cm|hgt:(59|6[0-9]|7[0-6])in",
        r"(hcl):#([0-9a-f]){6}",
        r"(ecl):(amb|blu|brn|gry|grn|hzl|oth)",
        r"(pid):[0-9]{9}"
    ]).unwrap();
    
    let matches: Vec<_> = set.matches(passport).into_iter().collect();
    matches == vec![0, 1, 2, 3, 4, 5, 6]
}

fn vec_from_file(file_name: String) -> Vec<String> {
    let mut vec_result = Vec::<String>::new();
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    let lines = buf.lines();

    let mut temp = String::new();
    for line in lines {
        let new_line = line.unwrap();
        
        // New line from the input separates each document. Push previous lines to the vec and continue.
        if new_line.trim().is_empty() {
            vec_result.push(temp.clone());
            temp.clear();
            continue;
        }
        temp.push_str(&new_line);
    }
    // push last value
    vec_result.push(temp);

    vec_result
}

fn main() {
    let doc_vec = vec_from_file(String::from("input.txt"));
    let mut count = 0;
    let mut count2 = 0;
    for val in doc_vec {
        if passport_is_valid(&val) {
            count += 1;
        }
        if passport_is_valid2(&val) {
            count2 += 1;
        }
    }

    println!("Number of valid passports: {}", count);
    println!("Number of valid passports with strict rules: {}", count2);
}
