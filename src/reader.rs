use std::fs::File;
use std::io::{BufRead, BufReader};

// Parse through the csv, return a list of tuples which can then be read into the Stop struct
pub fn parse_csv(path: &str) -> Vec<(u32, u32, f32, f32, Vec<u32>, Vec<u32>)> {
    let mut result = Vec::new();
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut fields = line.split(',');
        let a: u32 = fields.next().unwrap().parse().expect("Failed to parse field a");
        let b: u32 = fields.next().unwrap().parse().expect("Failed to parse field b");
        let c: f32 = fields.next().unwrap().parse().expect("Failed to parse field c");
        let d: f32 = fields.next().unwrap().parse().expect("Failed to parse field d.");
        let e_str = fields.next().unwrap();
        let e_str = e_str.trim_matches(|c| c == '[' || c == ']');
        let e: Vec<u32> = e_str.split(';').map(|s| s.trim().parse().expect("Failed to parse field e")).collect();
        let f_str = fields.next().unwrap();
        let f_str = f_str.trim_matches(|c| c == '[' || c == ']');
        let f: Vec<u32> = f_str.split(';').map(|s| s.trim().parse().expect("Failed to parse field f")).collect();
        result.push((a, b, c, d, e, f));
    }
    result
}