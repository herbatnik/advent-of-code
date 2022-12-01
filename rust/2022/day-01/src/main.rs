use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "./input";
    let part_one_result = part_one(file_path);
    let part_two_result = part_two(file_path);
    println!("part one: {part_one_result:?}");
    println!("part two: {part_two_result:?}");
}

fn part_one(file_path: &str) -> i32 {
    let mut calories_per_elf = read_calories_per_elf(file_path);
    calories_per_elf.sort_unstable();
    calories_per_elf.into_iter().last().unwrap_or(0)
}

fn part_two(file_path: &str) -> i32 {
    let mut calories_per_elf = read_calories_per_elf(file_path);
    calories_per_elf.sort_unstable();
    calories_per_elf.pop().unwrap_or(0)
        + calories_per_elf.pop().unwrap_or(0)
        + calories_per_elf.pop().unwrap_or(0)
}

enum LineType {
    Calories(i32),
    Blank,
}

fn read_calories_per_elf(file_path: &str) -> Vec<i32> {
    let file = std::fs::File::open(file_path).expect("failed to read file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(parse_line)
        .fold(Vec::new(), aggregate_calories)
}

fn parse_line(line: String) -> LineType {
    match line.as_str() {
        "" => LineType::Blank,
        calories => LineType::Calories(calories.parse().unwrap_or(0)),
    }
}

fn aggregate_calories(mut acc: Vec<i32>, next: LineType) -> Vec<i32> {
    match next {
        LineType::Calories(cal) => {
            if let Some(last) = acc.last_mut() {
                *last += cal;
            } else {
                acc.push(cal);
            }
        }
        LineType::Blank => acc.push(0),
    };
    acc
}
