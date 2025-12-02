use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("--- Day 1: Secret Entrance ---");
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut rotations: Vec<String> = contents.split('\n').map(|s| String::from(s)).collect();

    rotations.pop();
    let mut res = 50;
    let mut password = 0;

    for mut r in rotations {
        let (dir, distance) = r.split_at_mut(1);
        if let Ok(mut distance) = distance.parse::<i32>() {
            println!("dir: {}, distance: {}", dir, distance);

            if distance >= 100 {
                password += distance / 100;
                distance %= 100;
            }

            if dir == "L" {
                if res - distance >= 0 {
                    res -= distance;
                } else {
                    if res != 0 {
                        password += 1;
                    }
                    res = 100 + res - distance;
                }
            } else if dir == "R" {
                if res + distance <= 100 {
                    res += distance;
                    if res == 100 {
                        res = 0;
                    }
                } else {
                    password += 1;
                    res = (res + distance) % 100;
                }
            }
            println!("res: {}", res);
            if res == 0 {
                password += 1;
            }
        }
    }

    println!("password: {}", password);
    Ok(())
}
