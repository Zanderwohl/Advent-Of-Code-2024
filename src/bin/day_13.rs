mod util;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;
use util::parsing;

type Num = i32;

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
        let mut acc_x: Num = 0;
        let mut acc_y: Num = 0;
        let mut cost: Num = 0;
        while acc_x < self.prize.0 && acc_y < self.prize.1
        && !((self.prize.0 - acc_x) % self.b.0 == 0 && (self.prize.1 - acc_y) % self.b.1 == 0) {
            acc_x += self.a.0;
            acc_y += self.a.1;
            cost += 3;
        }
        if (self.prize.0 - acc_x) % self.b.0 == 0 && (self.prize.1 - acc_y) % self.b.1 == 0 {
            cost += (self.prize.0 - acc_x) / self.b.0;
            return Some(cost);
        }
        None
    }
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
}