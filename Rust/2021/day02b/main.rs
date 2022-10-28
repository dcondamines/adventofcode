use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let input = File::open("../2021_02.txt").expect("");
    let reader = BufReader::new(input);
    let mut depth = 0;
    let mut x_pos = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line_uw = line.unwrap();

        let movement = line_uw.split_whitespace().nth(1).unwrap().to_string().parse::<i32>().unwrap();
        match line_uw.split_whitespace().nth(0).unwrap() {
            "forward" => {
                x_pos += movement;
                depth += aim * movement;
            } 
            "down" => aim += movement,
            "up" => aim -= movement,
            _ => println!("Shouldn't reach this."),
        }
    }
    println!("Of the given input file, {} depth times {} horizontal position equals {}.",depth, x_pos, depth * x_pos);
    Ok(())
}