use std::fs;

#[derive(Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn main() {
    let mut matrix = [[0u32; 1000]; 1000];
    let claims: Vec<Claim> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|e| {
            let splits: Vec<&str> = e.split(' ').collect();

            let id: u32 = splits[0][1..]
                .parse()
                .expect("Failed to parse number to u32"); // remove the hash `#` prefix then parse to u32

            let left_top_vec: Vec<&str> = splits[2].split(',').collect();
            let width_height_vec: Vec<&str> = splits[3].split('x').collect();

            let top_len = left_top_vec[1].len();

            // TODO: see if there's a way to destructure these
            let left: u32 = left_top_vec[0]
                .parse()
                .expect("Failed to parse number to u32");
            let top: u32 = left_top_vec[1][0..top_len - 1]
                .parse()
                .expect("Failed to parse number to u32");
            let width: u32 = width_height_vec[0]
                .parse()
                .expect("Failed to parse number to u32");
            let height: u32 = width_height_vec[1]
                .parse()
                .expect("Failed to parse number to u32");

            Claim {
                id,
                left,
                top,
                width,
                height,
            }
        })
        .collect();

    for claim in &claims {
        for i in claim.top..claim.top + claim.height {
            for j in claim.left..claim.left + claim.width {
                // TODO: is casting to `usize` idiomatic? maybe there's another way? or should I use `usize` in the first place and not `u32`?
                matrix[i as usize][j as usize] += 1;
            }
        }
    }

    let more_than_two = matrix.iter().flatten().filter(|e| **e >= 2).count();
    // these two below are valid too
    // let more_than_two = matrix.iter().flatten().filter(|&e| *e >= 2).count();
    // let more_than_two = matrix.iter().flatten().filter(|&&e| e >= 2).count();

    println!("part 1: {}", more_than_two);
    if let Some(id) = get_no_overlap_id_opt(&claims, &matrix) {
        println!("part 2: {}", id);
    }
}

fn get_no_overlap_id_opt(claims: &Vec<Claim>, matrix: &[[u32; 1000]; 1000]) -> Option<u32> {
    for claim in claims {
        // TODO: proper way of slicing, flattening, and filtering 2D arrays (there's a bug on my commented implementation)
        // let height_start = claim.top as usize;
        // let height_end = (claim.top + claim.height) as usize;
        // let width = claim.width as usize;

        // let count_of_ones = matrix[height_start..height_end]
        //     .iter()
        //     .map(|e| &e[..width])
        //     .flatten()
        //     .filter(|e| **e == 1)
        //     .count();
        // let total_cell_count = (claim.width * claim.height) as usize;

        let total_cell_count = claim.width * claim.height;
        let mut count_of_ones = 0;

        for i in claim.top..claim.top + claim.height {
            for j in claim.left..claim.left + claim.width {
                count_of_ones += matrix[i as usize][j as usize];
            }
        }

        if count_of_ones == total_cell_count {
            return Some(claim.id);
        }
    }

    None
}
