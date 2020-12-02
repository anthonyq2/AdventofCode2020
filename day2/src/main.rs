use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct PasswordRule {
    character: char,
    min: usize,
    max: usize,
    password: String
}

impl PasswordRule {
    fn count_recurring_char(&self) -> usize {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.character {
                count += 1;
            }
        }
        count
    }

    // Validate using the fist (wrong) password rule
    fn is_valid(&self) -> bool {
        let count = self.count_recurring_char();
        count >= self.min && count <= self.max
    }

    // Validate using the second rule
    fn is_valid2(&self) -> bool {
        (self.password.chars().nth(self.min - 1).unwrap() == self.character
        && self.password.chars().nth(self.max - 1).unwrap() != self.character)
        || (self.password.chars().nth(self.min - 1).unwrap() != self.character 
        && self.password.chars().nth(self.max - 1).unwrap() == self.character)
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
    let mut valid_count2: u16 = 0;

    for password in passwords {
        println!("{}", password.min);
        
        if password.is_valid() {
            valid_count += 1;
        }

        if password.is_valid2() {
            valid_count2 += 1;
        }
    }

    println!("{} passwords are valid using the old rules", valid_count);
    println!("{} passwords are valid with the new rules!", valid_count2);
}
