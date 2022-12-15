use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use regex::{Match, Regex};

/// matches '   ' or '[X]', where X is any letter A-Z, followed by space or new line
/// this will extract the letters or space in matched groups
const ARRANGEMENT_REGEX: &str = r#"(?:[ \[]([A-Z ])[ \]])(?: |$)"#;

// TODO this is very bad implementation and was done to just solve the problem quickly.
fn main() -> Result<(), String> {
    let file_path = "./input";
    let part_one_result = part_one(file_path)?;
    println!("part one: {part_one_result:?}");
    let part_two_result = part_two(file_path)?;
    println!("part two: {part_two_result:?}");
    Ok(())
}

fn part_one(file_path: &str) -> Result<String, String> {
    let file = std::fs::File::open(file_path).map_err(|_| "failed to read file".to_string())?;
    let mut lines = BufReader::new(file)
        .lines();
    let mut input_arrangement_lines = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let regex = regex::Regex::new(ARRANGEMENT_REGEX).expect("regex is invalid");
        let crates: Vec<_> = regex
            .captures_iter(&line)
            .map(|c| match c.get(1).unwrap().as_str() {
                " " => None,
                X => Some(X.to_string())
            }).collect();
        if crates.is_empty() {
            break;
        }

        input_arrangement_lines.push(crates);
    }
    let len = input_arrangement_lines.first().unwrap().len();
    let mut arrangements = input_arrangement_lines
        .into_iter()
        .fold(vec![VecDeque::new(); len] , |mut acc, next| {
            for (index, value) in next.into_iter().enumerate() {
                if let Some(x) = value {
                    let mut v = acc.get_mut(index).unwrap();
                    v.push_front(x);
                }
            }
            acc
        });

    // ignore empty line between initial arrangement and the steps
    let _ = lines.next();

    let regex = Regex::new(r#"move ([\d]+) from ([\d]+) to ([\d]+)"#).expect("regex is invalid");
    lines.map(|x| x.expect("line expected"))
        .map(|x| {
            let x = regex.captures(&x).expect("expected to match regex");
            let count = x.get(1).expect("expected match").as_str().to_string();
            let from = x.get(2).expect("expected match").as_str().to_string();
            let to = x.get(3).expect("expected match").as_str().to_string();
            (count.parse::<usize>().unwrap(), from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap())
        })
        .for_each(|(count, from, to)| {
            for _ in 0..count {
                let from = arrangements.get_mut(from - 1).unwrap();
                let elem = from.pop_back().unwrap();
                let to = arrangements.get_mut(to - 1).unwrap();
                to.push_back(elem);
            }
        });
    let result: Vec<String> = arrangements.into_iter().map(|mut x| { x.pop_back().unwrap() }).collect();
    Ok(result.join(""))
}

fn part_two(file_path: &str) -> Result<String, String> {
    let file = std::fs::File::open(file_path).map_err(|_| "failed to read file".to_string())?;
    let mut lines = BufReader::new(file)
        .lines();
    let mut input_arrangement_lines = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let regex = regex::Regex::new(ARRANGEMENT_REGEX).expect("regex is invalid");
        let crates: Vec<_> = regex
            .captures_iter(&line)
            .map(|c| match c.get(1).unwrap().as_str() {
                " " => None,
                X => Some(X.to_string())
            }).collect();
        if crates.is_empty() {
            break;
        }

        input_arrangement_lines.push(crates);
    }
    let len = input_arrangement_lines.first().unwrap().len();
    let mut arrangements = input_arrangement_lines
        .into_iter()
        .fold(vec![VecDeque::new(); len] , |mut acc, next| {
            for (index, value) in next.into_iter().enumerate() {
                if let Some(x) = value {
                    let mut v = acc.get_mut(index).unwrap();
                    v.push_front(x);
                }
            }
            acc
        });

    // ignore empty line between initial arrangement and the steps
    let _ = lines.next();

    let regex = Regex::new(r#"move ([\d]+) from ([\d]+) to ([\d]+)"#).expect("regex is invalid");
    lines.map(|x| x.expect("line expected"))
        .map(|x| {
            let x = regex.captures(&x).expect("expected to match regex");
            let count = x.get(1).expect("expected match").as_str().to_string();
            let from = x.get(2).expect("expected match").as_str().to_string();
            let to = x.get(3).expect("expected match").as_str().to_string();
            (count.parse::<usize>().unwrap(), from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap())
        })
        .for_each(|(count, from, to)| {
            let mut elements = VecDeque::new();
            for _ in 0..count {
                let from = arrangements.get_mut(from - 1).unwrap();
                let elem = from.pop_back().unwrap();
                elements.push_front(elem);
            }
            let to = arrangements.get_mut(to - 1).unwrap();
            for element in elements {
                to.push_back(element);
            }
        });
    let result: Vec<String> = arrangements.into_iter().map(|mut x| { x.pop_back().unwrap() }).collect();
    Ok(result.join(""))
}
