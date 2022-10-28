use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let input = File::open("2021_01.txt").expect("");
    let reader = BufReader::new(input);
    let mut depths: [i32; 3] = [0; 3];
    let mut increments = 0;

    for line in reader.lines() {
        let current_depth = line?.parse::<i32>().unwrap();
        if depths[0] != 0 && depths[1] != 0 && depths[2] != 0{
            let depths_sum = depths[0] + depths[1] + depths[2];
            let current_depths_sum = depths[1] + depths[2] + current_depth;
            if depths_sum < current_depths_sum {
                println!("{} < {}",depths_sum, current_depths_sum);
                increments += 1;
            } else {
                println!("{} >= {}",depths_sum, current_depths_sum);
            }

        }
        depths[0] = depths[1];
        depths[1] = depths[2];
        depths[2] = current_depth;      
    }

    println!("Of the given input file, {increments} three-measurements were larger than the previous one.");
    Ok(())
}