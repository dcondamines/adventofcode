use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let input = File::open("2021_01.txt").expect("");
    let reader = BufReader::new(input);
    let mut depth = 0;
    let mut increments = 0;

    for line in reader.lines() {
        let current_depth = line?.parse::<i32>().unwrap();

        if depth != 0 && depth < current_depth {
            increments += 1;
        }
        depth = current_depth;

    }

    println!("Of the given input file, {increments} measurements were larger than the previous one.");
    Ok(())
}