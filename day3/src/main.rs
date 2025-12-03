use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("--- Day 3: Lobby --- ");

    let file = File::open("input.txt")?;
    let mut br = BufReader::new(file);
    let mut contents = String::new();
    br.read_to_string(&mut contents)?;
    let banks: Vec<&str> = contents.trim().split('\n').collect();
    let mut total_output_joltage: u32 = 0;
    let mut max_bank_joltage: u32 = 0;

    for bank in banks {
        for fb in bank.chars().enumerate() {
            for sb in bank.chars().enumerate() {
               if sb.0 > fb.0 {
                    let joltage = format!("{}{}", fb.1, sb.1);
                    if let Ok(joltage) = joltage.parse::<u32>() {
                        if joltage > max_bank_joltage {
                            max_bank_joltage = joltage;
                        }
                    }
               }
            }
        }

        total_output_joltage += max_bank_joltage;
        max_bank_joltage = 0;
    }

    println!("Total output joltage is {}", total_output_joltage);
    Ok(())
}
