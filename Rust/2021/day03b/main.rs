use std::io::Result;
use std::fs;
fn main() -> Result<()> {
    let input = fs::read_to_string("../2021_03.txt").expect("No file found.");
    let mut input_lines_oxygen: Vec<&str> = input.lines().collect();
    let mut input_lines_co2: Vec<&str> = input.lines().collect();
    let x_length = input_lines_oxygen[0].len();
    let mut oxygen = "".to_string();
    let mut co2 = "".to_string();

    for x in 0..x_length {
        let mut count_oxygen = 0;
        let mut count_co2 = 0;

        if input_lines_oxygen.len() <= 1 {
            oxygen += &input_lines_oxygen[0].chars().nth(x).unwrap().to_string();
        } else {
            for y in 0..input_lines_oxygen.len() {
                if input_lines_oxygen[y].chars().nth(x).unwrap() == '1' {
                    count_oxygen += 1;
                } else {
                    count_oxygen -= 1;
                }
            } 

            if count_oxygen > 0 || count_oxygen == 0  {
                input_lines_oxygen.retain(|&c| c.chars().nth(x).unwrap() == '1');
                oxygen += "1";
                println!("Oxygen: {} left, string is {}", input_lines_oxygen.len(), oxygen );
            } else  {
                input_lines_oxygen.retain(|&c| c.chars().nth(x).unwrap() == '0');
                oxygen += "0";
                println!("Oxygen: {} left, string is {}", input_lines_oxygen.len(), oxygen );
            } 
        }

        if input_lines_co2.len() <= 1 {
            co2 += &input_lines_co2[0].chars().nth(x).unwrap().to_string();
        } else {
            for y in 0..input_lines_co2.len() {
                if input_lines_co2[y].chars().nth(x).unwrap() == '1' {
                    count_co2 += 1;
                } else {
                    count_co2 -= 1;
                }
            }

            if count_co2 > 0 || count_co2 == 0  {
                input_lines_co2.retain(|&c| c.chars().nth(x).unwrap() == '0');
                co2 += "0";
                println!("CO2: {} left, string is {}", input_lines_co2.len(), co2 );
            } else  {
                input_lines_co2.retain(|&c| c.chars().nth(x).unwrap() == '1');
                co2 += "1";
                println!("CO2: {} left, string is {}", input_lines_co2.len(), co2 );
            } 
        }
        
    
        
    }

    let oxygen_int = isize::from_str_radix(&oxygen, 2).unwrap();
    let co2_int = isize::from_str_radix(&co2, 2).unwrap();
        println!("{oxygen} - {co2} -- {oxygen_int} * {co2_int} = {}", oxygen_int * co2_int);
    Ok(())
}