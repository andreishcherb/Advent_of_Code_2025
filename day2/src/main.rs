use fancy_regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("--- Day 2: Gift Shop ---");

    let file = File::open("input.txt")?;
    let mut br = BufReader::new(file);
    let mut contents = String::new();
    br.read_to_string(&mut contents)?;
    let ranges: Vec<&str> = contents.trim().split(',').collect();

    let re = Regex::new(r"^(\d+)(\1+)$").expect("should be valid regex");
    let mut result = 0;

    for range in ranges {
        if let Some(range) = range.split_once('-') {
            let start = range.0.parse::<u64>();
            if let Ok(mut start) = start {
                let end = range.1.parse::<u64>();
                if let Ok(end) = end {
                    println!("start: {}, end: {}", start, end);
                    while start <= end {
                        let id = start.to_string();
                        if let Ok(v) = re.is_match(id.as_str()) {
                            if v {
                                result += start;
                            }
                        }
                        start += 1;
                    }
                }
            }
        }
    }

    println!("result = {}", result);

    Ok(())
}
