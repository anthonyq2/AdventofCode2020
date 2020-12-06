use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_trees(path_vec: &Vec<String>, right: usize, down: usize) -> usize {
    let mut count = 0;
    let mut read_index = 0;

    let mut idx = 0;
    loop {
        if idx + down >= path_vec.len() { // Final row
            break;
        }
        
        let first_row = &path_vec[idx];
        let second_row = &path_vec[idx + down];
        let char_read_index = read_index + right;
        if char_read_index > first_row.len() - 1 {
            let remaining_right = char_read_index - (first_row.len() - 1);
            read_index = remaining_right - 1;
        }
        else {
            read_index = char_read_index;
        }

        // Check if there is a tree (#) at this space
        if second_row.chars().nth(read_index).unwrap() == '#' {
            count += 1
        }

        idx += down;
    }

    count
}

fn vec_from_file(file_name: String) -> Vec<String> {
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|line| line.unwrap()).collect()
}

fn main() {
    let mut counts = Vec::new();
    
    // Find the number of trees in the first slope (right 3, down 1)
    let path_vec = vec_from_file(String::from("input.txt"));
    let mut tree_count = count_trees(&path_vec, 3, 1);
    println!("Trees found in first slope: {}: ", tree_count);
    counts.push(tree_count);

    // Find the number of trees in other slopes
    tree_count = count_trees(&path_vec, 1, 1);
    counts.push(tree_count);
    tree_count = count_trees(&path_vec, 5, 1);
    counts.push(tree_count);
    tree_count = count_trees(&path_vec, 7, 1);
    counts.push(tree_count);
    tree_count = count_trees(&path_vec, 1, 2);
    counts.push(tree_count);
    
    // Get and print the products of all the slopes
    let slope_products = counts.iter().product::<usize>();
    println!("Product of all trees in slopes: {}", slope_products);
}
