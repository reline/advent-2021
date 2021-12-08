use std::fs::{read_to_string};
use std::path::PathBuf;

pub fn split_num_commas(path: PathBuf) -> Vec<i32> {
    read_to_string(&path)
        .unwrap()
        .trim()
        .split(',')
        .map(|n| {
            n.parse::<i32>()
        })
        .filter_map(Result::ok)
        .collect::<Vec<i32>>()
}
