mod util;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;
use util::parsing;

type Num = i64;

lazy_static! {
    static ref EXPR: Regex = Regex::new(r"\d+").unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let lines = parsing::file_into_vec("files/day_13_input.txt")?;
    let machines = parse_input(&lines);

    let solutions: Vec<Option<Num>> = machines.iter().map(|m| m.solve()).collect();
    let cost: Num = solutions.into_iter().filter_map(|x| x).sum();
    println!("Total cost:\n{}", cost);

    let true_solutions: Vec<Option<Num>> = machines.iter().map(|m| { m.solve_sad() }).collect();
    let true_cost: Num = true_solutions.into_iter().filter_map(|x| x).sum();
    println!("Total _true_ cost:\n{}", true_cost);


    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);

    Ok(())
}

fn parse_input(input: &Vec<String>) -> Vec<Machine> {
    let mut output: Vec<Machine> = Vec::with_capacity(input.len() / 4);
    for i in 0..=(input.len() / 4) {
        let idx = i * 4;
        let a: Vec<Num> = EXPR
            .find_iter(&input[idx + 0])
            .map(|m| m.as_str().parse::<Num>().unwrap())
            .collect();
        let b: Vec<Num> = EXPR
            .find_iter(&input[idx + 1])
            .map(|m| m.as_str().parse::<Num>().unwrap())
            .collect();
        let prize: Vec<Num> = EXPR
            .find_iter(&input[idx + 2])
            .map(|m| m.as_str().parse::<Num>().unwrap())
            .collect();

        output.push(Machine {
            n: idx,
            a: (a[0], a[1]),
            b: (b[0], b[1]),
            prize: (prize[0], prize[1]),
        })
    }
    output
}

struct Machine {
    n: usize,
    a: (Num, Num),
    b: (Num, Num),
    prize: (Num, Num),
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button A: X+{}, Y+{}\n", self.a.0, self.a.1)?;
        write!(f, "Button B: X+{}, Y+{}\n", self.b.0, self.b.1)?;
        write!(f, "Prize: X={}, Y={}\n", self.prize.0, self.prize.1)
    }
}

impl Machine {
    pub fn solve(&self) -> Option<Num> {
        let (a_x, a_y) = self.n_a_sad(0);
        let (b_x, b_y) = self.n_b_sad(0);
        if a_x % a_y == 0 && b_x % b_y == 0 {
            return Some((a_x / a_y) * 3 + (b_x / b_y))
        }
        None
    }

    pub fn solve_sad(&self) -> Option<Num> {
        let plus = 10000000000000;
        let (a_x, a_y) = self.n_a_sad(plus);
        let (b_x, b_y) = self.n_b_sad(plus);
        // is a_x / a_y an int? is b_x / b_y an int?

        if a_x % a_y == 0 && b_x % b_y == 0 {
            return Some((a_x / a_y) * 3 + (b_x / b_y))
        }
        None
    }

    pub fn n_a(&self) -> f64 {
        random_equality(self.a.0 as f64, self.a.1 as f64, self.b.0 as f64, self.b.1 as f64, self.prize.0 as f64, self.prize.1 as f64)
    }

    pub fn n_b(&self) -> f64 {
        random_equality(self.b.0 as f64, self.b.1 as f64, self.a.0 as f64, self.a.1 as f64, self.prize.0 as f64, self.prize.1 as f64)
    }

    pub fn n_a_sad(&self, plus: Num) -> (Num, Num) {
        random_equality_parts(self.a.0, self.a.1, self.b.0, self.b.1, self.prize.0 + plus, self.prize.1 + plus)
    }

    pub fn n_b_sad(&self, plus: Num) -> (Num, Num) {
        random_equality_parts(self.b.0, self.b.1, self.a.0, self.a.1, self.prize.0 + plus, self.prize.1 + plus)
    }
}

pub fn random_equality(x1: f64, y1: f64, x2: f64, y2: f64, xp: f64, yp: f64) -> f64 {
    (y2 * xp - x2 * yp) / (y2 * x1 - x2 * y1)
}

pub fn random_equality_parts(x1: Num, y1: Num, x2: Num, y2: Num, xp: Num, yp: Num) -> (Num, Num) {
    (y2 * xp - x2 * yp, y2 * x1 - x2 * y1)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::time::Instant;
    use crate::{parse_input, Num};
    use crate::util::parsing;

    #[test]
    fn test_small() -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let lines = parsing::file_into_vec("files/day_13_small.txt")?;
        let machines = parse_input(&lines);
        let solutions: Vec<Option<Num>> = machines.iter().map(|m| m.solve()).collect();
        let expected: Vec<Option<Num>> = vec![
            Some(280),
            None,
            Some(200),
            None,
        ];

        for idx in 0..solutions.len() {
            assert_eq!(solutions[idx], expected[idx]);
        }

        let cost: Num = solutions.into_iter().filter_map(|x| x).sum();
        assert_eq!(cost, 480);

        Ok(())
    }

    #[test]
    fn test_sad_small() -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let lines = parsing::file_into_vec("files/day_13_small.txt")?;
        let machines = parse_input(&lines);
        let solutions: Vec<Option<Num>> = machines.iter().map(|m| m.solve_sad()).collect();
        let expected: Vec<Option<Num>> = vec![
            None,
            Some(0),
            None,
            Some(0),
        ];

        for idx in 0..solutions.len() {
            println!("{}", idx);
            assert_eq!(solutions[idx].is_some(), expected[idx].is_some());
        }

        Ok(())
    }
}