mod util;

use std::cmp::min;
use std::error::Error;
use crate::util::parsing;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EXPR: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    static ref EXPR_B: Regex = Regex::new(r"^mul\(\d{1,3},\d{1,3}\)").unwrap();
    static ref DO: Regex = Regex::new(r"^do\(\)").unwrap();
    static ref DONT: Regex = Regex::new(r"^don't\(\)").unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_03_input.txt")?;
    let sum: u64 = lines.iter().map(|line| {
        eval_line(line)
    }).sum();
    println!("The sum of all mults is:\n{}", sum);
    let smart_sum = eval_do_dont(lines);
    println!("The sum of mults with conditionals is:\n{}", smart_sum);

    Ok(())
}

pub fn eval_mul(expr: &str) -> u64 {
    let mut split_index: usize = 0;
    for (idx, char) in expr.chars().enumerate() {
        if char == ',' {split_index = idx; break;}
    }
    let first = {
        let from = expr.char_indices().nth(4).unwrap().0;
        let to = split_index;
        let slice = &expr[from..to];
        slice.parse::<u64>().unwrap()
    };
    let second = {
        let from = split_index + 1;
        let to = expr.char_indices().nth_back(1).unwrap().0;
        let slice = &expr[from..=to];
        slice.parse::<u64>().unwrap()
    };
    let product: u64 = first * second;
    product
}

pub fn eval_line(line: &str) -> u64 {
    EXPR.find_iter(line).map(|expr| {
        let expr = expr.as_str();
        eval_mul(expr)
    }).sum()
}

pub fn eval_do_dont(lines: Vec<String>) -> u64 {
    let mut active = true;
    let mut sum = 0;
    for line in lines {
        let indices: Vec<(usize, char)> = line.char_indices().collect();
        let len = line.chars().count();
        for (idx, c) in line.chars().enumerate() {
            let from = indices[idx].0;
            let to = indices[min(idx + 11, len - 1)].0;
            let slice = &line[from..=to];
            if DO.is_match(slice) {
                active = true;
            } else if DONT.is_match(slice) {
                active = false;
            } else if EXPR_B.is_match(slice) {
                if let Some(captures) = EXPR_B.captures(slice) {
                    let expr = &captures[0];
                    if active {
                        sum += eval_mul(expr);
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{eval_line, eval_do_dont, eval_mul};

    #[test]
    fn test_eval_mul() {
        let pairs = [
            ("mul(962,335)", 322270),
            ("mul(73,181)", 13213),
            ("mul(1,1)", 1),
            ("mul(5,4)", 20),
            ("mul(21,45)", 945),
        ];
        for (expr, expected) in &pairs {
            let actual = eval_mul(expr);
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn test_eval_line() {
        let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual = eval_line(line);
        assert_eq!(actual, 161);
    }

    #[test]
    fn test_do_dont() {
        let line = vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()];
        let actual = eval_do_dont(line);
        assert_eq!(actual, 48);
    }
}