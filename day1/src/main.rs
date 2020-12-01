use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn vector_from_file(file_name: String) -> Vec<usize>{
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.ok().and_then(|l| l.parse().ok()))
        .collect()
}

fn main() {
    let entries = vector_from_file(String::from("input.txt"));

    // Product of 2 entries whose sum is 2020
    'outer: for i in &entries {
        for j in &entries { 
            if i + j == 2020 {
                let solution = i * j;
                println!("{}", solution);
                break 'outer;
            }
        }
    }

    // Product of 3 entries whose sum is 2020
    'outer2: for i in &entries {
        for j in &entries {
            for k in &entries {
                if i + j + k == 2020 {
                    let solution = i * j * k;
                    println!("{}", solution);
                    break 'outer2;
                }
            }
        }
    }
}
