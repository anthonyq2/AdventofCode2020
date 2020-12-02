use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct PasswordRule {
    character: char,
    min: u8,
    max: u8,
    password: String
}

impl PasswordRule {
    fn count_recurring_char(&self) -> u8 {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.character {
                count += 1;
            }
        }
        count
    }

    fn is_valid(&self) -> bool {
        let count = self.count_recurring_char();
        count >= self.min && count <= self.max
    }
}

fn password_rule_from_line(value: &String) -> PasswordRule {
    let mut colon_index: usize = 0; 
    let mut hypen_index: usize = 0;
    for (i, c) in value.chars().enumerate() {
        if c == '-' {
            hypen_index = i;
        }
        if c == ':' {
            colon_index = i;
        }
    }

    // Create the password rule
    PasswordRule { 
        character: value.chars().nth(colon_index - 1).unwrap(),
        min: value[0..hypen_index].trim().parse().unwrap(),
        max: value[hypen_index + 1..colon_index - 2].trim().parse().unwrap(),
        password: value[colon_index + 1..value.len()].trim().to_string()
    }
}

fn password_rules_from_file(file_name: String, password_rules: &mut Vec<PasswordRule>) {
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    for line in buf.lines() {
        let value = &line.unwrap();
        if value.trim().is_empty() {
            continue;
        }
        let password_rule: PasswordRule = password_rule_from_line(value);
        password_rules.push(password_rule);
    }
}

fn main() {
    let mut passwords = Vec::new();
    password_rules_from_file(String::from("input.txt"), &mut passwords);
    
    let mut valid_count: u16 = 0;
    for password in passwords {
        println!("{}", password.password);
        
        if password.is_valid() {
            valid_count += 1;
        }
    }

    println!("{} passwords are valid", valid_count);
}
