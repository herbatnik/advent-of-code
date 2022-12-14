extern crate core;

use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

fn main() -> Result<(), String> {
    let file_path = "./input";
    let part_one_result = part_one(file_path)?;
    println!("part one: {part_one_result:?}");
    let part_two_result = part_two(file_path)?;
    println!("part two: {part_two_result:?}");
    Ok(())
}

fn part_one(file_path: &str) -> Result<i32, String> {
    let file = std::fs::File::open(file_path).map_err(|_| "failed to read file".to_string())?;
    Ok(BufReader::new(file)
        .lines()
        .map(|x| x.expect("failed to read line"))
        .map(|x| {
            let (first, second) = x.split_at(x.len() / 2);
            let set = first.chars().into_iter().collect::<HashSet::<char>>();
            for char in second.chars() {
                if set.contains(&char) {
                    return match char as i32 {
                        x if ('A' as i32) <= x && x <= ('Z' as i32) => x - 'A' as i32 + 27,
                        x => x - 'a' as i32 + 1
                    };
                }
            }
            unreachable!("should always contain common character");
        })
        .sum())
}

fn part_two(file_path: &str) -> Result<i32, String> {
    let file = std::fs::File::open(file_path).map_err(|_| "failed to read file".to_string())?;
    Ok(BufReader::new(file)
        .lines()
        .map(|x| x.expect("failed to read line"))
        .collect::<Vec::<String>>()
        .chunks(3)
        .map(|x| {
            match x {
                [first, second, third] => {
                    let mut map = HashMap::new();
                    for char in first.chars() {
                        map.insert(char, 1);
                    }
                    for char in second.chars() {
                        map.entry(char).and_modify(|x| *x = 2);
                    }
                    for char in third.chars() {
                        let entry = map.get(&char).unwrap_or(&0);
                        if *entry == 2 {
                            return match char as i32 {
                                x if ('A' as i32) <= x && x <= ('Z' as i32) => x - 'A' as i32 + 27,
                                x => x - 'a' as i32 + 1
                            };
                        }
                    }
                    unreachable!("should always contain common character");
                }
                _ => unreachable!("should always have groups of 3 lines")
            }
        })
        .sum())
}
