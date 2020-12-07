use std::{
    fs::File,
    io::{BufRead, BufReader}
};

fn calc_column(instruction: &String) -> usize {
    let mut cols = Vec::new();
    let mut col = 0;
    for i in 0..=7 {
        cols.push(i);
    }

    // Use a hard-coded index since we know where the Ls and Rs start.
    let mut idx = 7;
    loop {
        let c = instruction.chars().nth(idx).unwrap();
        if idx == 9 {
            if c == 'L' {
                col = cols[0];
            }
            else if c == 'R' {
                col = cols[1];
            }
            break;
        }
        
        let col_slice = cols.clone();
        let (lower, upper)= col_slice.split_at(col_slice.len() / 2);
        if c == 'L' {
            cols = lower.to_vec();
        }
        else if c == 'R' {
            cols = upper.to_vec(); 
        } 

        idx += 1;

    }

    col
}

fn calc_row(instruction: &String) -> usize {
    let mut rows = Vec::new();
    let mut row = 0;
    for i in 0..=127 {
        rows.push(i)
    }

    for (i, c) in instruction.chars().enumerate() {
        if i == 6 {
            if c == 'F' {
                row = rows[0];
            }
            else if c == 'B' {
                row = rows[1];
            }
            break;
        }
        let row_slice = rows.clone();
        let (lower, upper) = row_slice.split_at(row_slice.len() / 2);
        if c == 'F' {
            rows = lower.to_vec();
        }
        else if c == 'B' {
            rows = upper.to_vec(); 
        }
    }
    row
}

fn calc_seat_id(instruction: &String) -> usize {
    let row_num = calc_row(instruction);
    let col_num = calc_column(instruction);
    row_num * 8 + col_num
}

fn vec_from_file(file_name: String) -> Vec<String> {
    let file = File::open(file_name).expect("unable to open file");
    let buf = BufReader::new(file);
    buf.lines().map(|line| line.unwrap()).collect()
}

fn main() {
    let mut seat_ids = Vec::<usize>::new();
    let boarding_pass_instructions = vec_from_file(String::from("input.txt"));
    for inst in boarding_pass_instructions {
        let id = calc_seat_id(&inst);
        seat_ids.push(id);
    }

    // Find the maximum seat id
    let max = seat_ids.iter().max().expect("unable to determine max");
    println!("Max seat id is {}", max);

    // Using the max as the top of the range, print the seat ids that are missing
    println!("Missing seat ids:");
    for i in 0..*max {
        let seat = &(i + 1);
        if !seat_ids.contains(seat) {
            println!("{}", seat);
        }
    }
}
