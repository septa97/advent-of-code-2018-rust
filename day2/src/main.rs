use std::fs;

fn main() {
    let file_string = fs::read_to_string("input/input.txt").expect("Failed to read file!");
    let mut twos = 0;
    let mut threes = 0;

    for line in file_string.lines() {
        let mut frequencies = [0; 26];

        for c in line.chars() {
            let idx = c as usize - 'a' as usize;
            frequencies[idx] += 1;
        }

        if frequencies.iter().any(|&e| e == 2) {
            twos += 1;
        }

        if frequencies.iter().any(|&e| e == 3) {
            threes += 1;
        }
    }

    println!("part 1: {}", twos * threes);
    if let Some(matched_string) = get_matched_string(&file_string) {
        println!("part 2: {}", matched_string);
    }
}

fn get_matched_string(file_string: &String) -> Option<String> {
    let str_vec: Vec<&str> = file_string.lines().collect();

    for i in 0..str_vec.len() {
        for j in i+1..str_vec.len() {
            let zipped = str_vec[i].chars().zip(str_vec[j].chars());
            let matched_string: String =
                zipped.clone().filter(|(c1, c2)| c1 == c2).map(|(c, _)| c).collect(); // TODO: used `clone` for now, will come back to this later after I finished the Rust book
            let mismatched_string: String =
                zipped.clone().filter(|(c1, c2)| c1 != c2).map(|(c, _)| c).collect(); // TODO: used `clone` for now, will come back to this later after I finished the Rust book

            if mismatched_string.len() == 1 {
                return Some(matched_string);
            }
        }
    }

    None
}
