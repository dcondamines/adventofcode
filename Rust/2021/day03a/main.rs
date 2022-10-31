use std::io::Result;
use std::fs;
fn main() -> Result<()> {
    let input = fs::read_to_string("../2021_03.txt").expect("No file found.");
    let input_lines: Vec<&str> = input.lines().collect();
    let y_length = input_lines.len();
    let x_length = input_lines[0].len();
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for x in 0..x_length {
        let mut count = 0;

        for y in 0..y_length {
            if input_lines[y].chars().nth(x).unwrap() == '1' {
                count += 1;
            } else {
                count -= 1;
            }
        }
        
        if count > 0  {
            gamma += "1";
            epsilon += "0";
            println!("Count positive : {}", count);
        } else if count < 0  {
            gamma += "0";
            epsilon += "1";
            println!("Count negative : {}", count);
        } else if count == 0 {
            gamma += "0";
            epsilon += "0";
            println!("The values are equal!")
        }
    }

    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
        println!("{gamma} - {epsilon} -- {gamma_int} * {epsilon_int} = {}", gamma_int * epsilon_int);
    Ok(())
}