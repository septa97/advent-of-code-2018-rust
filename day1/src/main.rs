use std::fs;
use std::collections::HashSet;

fn main() {
    let num_vec: Vec<i32> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|x| x.parse().expect("Failed to parse number to i32"))
        .collect();

    println!("part 1: {}", num_vec.iter().sum::<i32>());
    println!("part 2: {}", get_first_repeated_freq(&num_vec));
}

fn get_first_repeated_freq(num_vec: &Vec<i32>) -> i32 {
    let mut occured = HashSet::new();
    let mut curr_freq = 0;

    loop {
        for num in num_vec {
            curr_freq += num;

            if occured.contains(&curr_freq) {
                return curr_freq;
            } else {
                occured.insert(curr_freq);
            }
        }
    }
}
