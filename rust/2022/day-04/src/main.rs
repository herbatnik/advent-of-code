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
            let mut split = x.split(",")
                .map(|x| {
                    let mut split = x.split("-");
                    let start = split.next().expect("should contain range start")
                        .parse::<i32>()
                        .expect("range start should be integer");
                    let end = split.next().expect("should contain range end")
                        .parse::<i32>()
                        .expect("range end should be integer");
                    (start, end)
                });
            (split.next().expect("should contain first range"), split.next().expect("should contain second range"))
        })
        .map(|((first_start, first_end), (second_start, second_end))| {
            if first_start <= second_start && first_end >= second_end {
                1
            } else if second_start <= first_start && second_end >= first_end {
                1
            } else {
                0
            }
        })
        .sum())
}

fn part_two(file_path: &str) -> Result<i32, String> {
    let file = std::fs::File::open(file_path).map_err(|_| "failed to read file".to_string())?;
    Ok(BufReader::new(file)
        .lines()
        .map(|x| x.expect("failed to read line"))
        .map(|x| {
            let mut split = x.split(",")
                .map(|x| {
                    let mut split = x.split("-");
                    let start = split.next().expect("should contain range start")
                        .parse::<i32>()
                        .expect("range start should be integer");
                    let end = split.next().expect("should contain range end")
                        .parse::<i32>()
                        .expect("range end should be integer");
                    (start, end)
                });
            (split.next().expect("should contain first range"), split.next().expect("should contain second range"))
        })
        .map(|((first_start, first_end), (second_start, second_end))| {
            if first_start <= second_start && first_end >= second_start {
                1
            } else if second_start <= first_start && second_end >= first_start {
                1
            } else {
                0
            }
        })
        .sum())
}