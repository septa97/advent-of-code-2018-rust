use std::collections::HashMap;
use std::fs;

// TODO: research more on the equality and ordering traits
// TODO: research more on Clone trait, needed for .to_vec method
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Detail {
    date: String,
    time: String,
    action: String,
}

fn main() {
    let mut details: Vec<Detail> = fs::read_to_string("input/input.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| {
            let splits: Vec<&str> = line.split(']').collect();
            let action = splits[1].trim().to_string();
            let datetime: Vec<&str> = splits[0][1..].split(' ').collect();
            let date = datetime[0].to_string();
            let time = datetime[1].to_string();

            Detail { date, time, action }
        })
        .collect();

    details.sort();

    let indices = 0..details.len();
    let shift_start_indices: Vec<usize> = details
        .iter()
        .map(|e| e.action.starts_with("Guard #"))
        .zip(indices)
        .filter(|(is_start, _)| *is_start)
        .map(|(_, idx)| idx)
        .collect();

    let mut sleep_count_map: HashMap<&str, Vec<u32>> = HashMap::new();

    for i in 0..shift_start_indices.len() - 1 {
        let j = i + 1;

        let guard_id = get_guard_id(&details, shift_start_indices[i]);
        let slice_start = shift_start_indices[i];
        let slice_end = shift_start_indices[j];
        let detail_slice = details[slice_start..slice_end].to_vec();
        let new_sleep_count = get_sleep_count(detail_slice);

        match sleep_count_map.get(guard_id) {
            Some(old_sleep_count) => {
                let new_value: Vec<u32> = old_sleep_count
                    .iter()
                    .zip(new_sleep_count)
                    .map(|(&old_val, new_val)| old_val + new_val)
                    .collect();
                sleep_count_map.insert(guard_id, new_value);
            }
            None => {
                sleep_count_map.insert(guard_id, new_sleep_count);
            }
        }
    }

    let part1_guard_id_max_opt = sleep_count_map
        .iter()
        .max_by(|&(_, a_value), &(_, b_value)| {
            let x = a_value.iter().sum::<u32>();
            let y = b_value.iter().sum::<u32>();
            x.cmp(&y)
        })
        .map(|(&k, _)| k);

    if let Some(guard_id_max) = part1_guard_id_max_opt {
        match sleep_count_map.get(guard_id_max) {
            Some(sleep_vec) => {
                let guard_id_max: usize = guard_id_max
                    .parse()
                    .expect("Failed to parse string to usize");

                if let Some(max_idx) = get_max_idx(sleep_vec) {
                    println!("part 1: {}", guard_id_max * max_idx);
                }
            }
            None => (),
        }
    }

    let part2_guard_id_max_opt = sleep_count_map
        .iter()
        .max_by(|&(_, a_value), &(_, b_value)| {
            let x = a_value.iter().max().unwrap();
            let y = b_value.iter().max().unwrap();

            x.cmp(y)
        })
        .map(|(&k, _)| k);

    if let Some(guard_id_max) = part2_guard_id_max_opt {
        match sleep_count_map.get(guard_id_max) {
            Some(sleep_vec) => {
                let guard_id_max: usize = guard_id_max
                    .parse()
                    .expect("Failed to parse string to usize");

                if let Some(max_idx) = get_max_idx(sleep_vec) {
                    println!("part 2: {}", guard_id_max * max_idx);
                }
            }
            None => (),
        }
    }
}

fn get_sleep_count(detail_slice: Vec<Detail>) -> Vec<u32> {
    let mut sleep_vec = vec![0u32; 60];

    let detail_slice: Vec<&Detail> = detail_slice
        .iter()
        .filter(|e| e.action == "wakes up" || e.action == "falls asleep") // TODO: Figure out what's happening here. I'm assuming you can access struct fields directly even on double references.
        .collect();

    if detail_slice.len() > 0 {
        for i in 0..detail_slice.len() - 1 {
            let j = i + 1;
            let curr = detail_slice[i];
            let next = detail_slice[j];
            let time: usize = (&next.time[3..])
                .parse()
                .expect("Failed to parse time to usize");

            let start_idx = (&curr.time[3..])
                .parse()
                .expect("Failed to parse time to usize");
            if curr.action == "falls asleep" {
                for i in start_idx..time {
                    sleep_vec[i] = 1;
                }
            }
        }
    }

    sleep_vec
}

fn get_guard_id(details: &Vec<Detail>, idx: usize) -> &str {
    let split: Vec<&str> = details[idx].action.split(' ').collect();

    &split[1][1..]
}

fn get_max_idx(sleep_vec: &Vec<u32>) -> Option<usize> {
    sleep_vec
        .iter()
        .enumerate()
        .max_by(|&(_, a_value), &(_, b_value)| a_value.cmp(b_value))
        .map(|(idx, _)| idx)
}
