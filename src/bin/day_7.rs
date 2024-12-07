use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::util::bitfutz::get_bit_at;
use crate::util::parsing;
use crate::util::parsing::colon_split;

mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_7_input.txt")?;
    let equations = parse(&lines);
    let total: i64 = equations.iter().enumerate().map(|(idx, eq)|{
        let solvable = eq.solvable_2_rec();
        if solvable {
            eq.left
        } else {
            0
        }
    }).sum();
    println!("The total calibration result is:\n{}", total);

    let total_3: i64 = equations.iter().enumerate().map(|(idx, eq)|{
        let solvable = eq.solvable_3_rec();
        if solvable {
            eq.left
        } else {
            0
        }
    }).sum();
    println!("The total revised calibration result is:\n{}", total_3);

    Ok(())
}

fn parse(lines: &Vec<String>) -> Vec<Equation> {
    colon_split(&lines).iter().map(|sides| {
        if sides.len() != 2 {
            return None;
        }
        let left = sides[0].parse::<i64>().unwrap();
        let a = (&sides[1]).split_whitespace();
        let right: Vec<i64> = a.map(|x| {
            x.parse::<i64>().unwrap()
        }).collect();
        Some(Equation {
            left,
            right,
        })
    }).into_iter().filter_map(|x| x).collect()
}

struct Equation {
    left: i64,
    right: Vec<i64>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = ", self.left)?;
        let len = self.right.len();
        for (idx, n) in self.right.iter().enumerate() {
            write!(f, "{}", n)?;
            if idx != len - 1 {
                write!(f, " ? ")?;
            }
        }
        write!(f, " :: {}", self.possible_solutions())?;
        Ok(())
    }
}

impl Equation {
    pub fn possible_solutions(&self) -> u32 {
        2u32.pow(self.n_operators())
    }

    pub fn possible_3_solutions(&self) -> u32 {
        3u32.pow(self.n_operators())
    }

    pub fn n_operators(&self) -> u32 {
        (self.right.len() - 1) as u32
    }

    pub fn solvable(&self) -> bool {
        self.solve().is_some()
    }

    pub fn solve(&self) -> Option<Vec<Operator>> {
        let mut operator_set: Vec<Operator> = Vec::with_capacity(self.n_operators() as usize);
        for _ in 0..self.n_operators() {
            operator_set.push(Operator::Plus)
        }
        for i in 0..self.possible_solutions() {
            for j in 0u8..(self.n_operators() as u8) {
                operator_set[j as usize] = Operator::from(get_bit_at(i, j));
            }
            let mut acc = self.right[0];
            for j in 1..self.right.len() {
                let b = self.right[j];
                let operator = &operator_set[j - 1];
                acc = operator.operate(acc, b);

            }
            if acc == self.left {
                return Some(operator_set)
            }
        }
        None
    }

    pub fn solvable_3(&self) -> bool {
        self.solve_3().is_some()
    }

    pub fn solve_3(&self) -> Option<Vec<Operator3>> {
        let mut operator_set: Vec<Operator3> = vec![Operator3::Plus; self.n_operators() as usize];
        for _ in 0..self.possible_3_solutions() {
            let mut acc = self.right[0];
            'a: for j in 1..self.right.len() {
                let b = self.right[j];
                let operator = &operator_set[j - 1];
                acc = operator.operate(acc, b);
                if acc > self.left {
                    break 'a;
                }
            }
            if acc == self.left {
                return Some(operator_set)
            }
            next_base_3(&mut operator_set); }
        None
    }

    pub fn solvable_2_rec(&self) -> bool {
        self.solvable_2_rec_(0, 0)
    }

    fn solvable_2_rec_(&self, acc: i64, idx: usize) -> bool {
        if idx >= self.right.len() {
            return acc == self.left
        }
        let lhs = acc;
        let rhs = self.right[idx];
        let a = self.solvable_rec_2_inner(lhs, rhs, Operator3::Plus, idx);
        let b = self.solvable_rec_2_inner(lhs, rhs, Operator3::Times, idx);

        a || b
    }

    pub fn solvable_3_rec(&self) -> bool {
        self.solvable_3_rec_(0, 0)
    }

    fn solvable_3_rec_(&self, acc: i64, idx: usize) -> bool {
        if idx >= self.right.len() {
            return acc == self.left
        }
        let lhs = acc;
        let rhs = self.right[idx];
        let a = self.solvable_rec_3_inner(lhs, rhs, Operator3::Plus, idx);
        let b = self.solvable_rec_3_inner(lhs, rhs, Operator3::Times, idx);
        let c = self.solvable_rec_3_inner(lhs, rhs, Operator3::Cat, idx);

        a || b || c
    }

    fn solvable_rec_3_inner(&self, lhs: i64, rhs: i64, op: Operator3, idx: usize) -> bool {
        let acc = op.operate(lhs, rhs);
        if acc <= self.left {
            self.solvable_3_rec_(acc, idx + 1)
        } else {
            false
        }
    }

    fn solvable_rec_2_inner(&self, lhs: i64, rhs: i64, op: Operator3, idx: usize) -> bool {
        let acc = op.operate(lhs, rhs);
        if acc <= self.left {
            self.solvable_2_rec_(acc, idx + 1)
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    pub fn from(bit: bool) -> Self {
        match bit {
            false => Self::Plus,
            true => Self::Times,
        }
    }

    pub fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Plus => a + b,
            Operator::Times => a * b,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Times => write!(f, "x"),
        }
    }
}

#[derive(Debug, Clone)]
enum Operator3 {
    Plus,
    Times,
    Cat,
}

impl Operator3 {
    pub fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
            Self::Cat => {
                let cat = format!("{}{}", a, b);
                cat.parse::<i64>().unwrap()
            },
        }
    }

    pub fn next(&self) -> (Self, bool) {
        match self {
            Operator3::Plus => (Self::Times, false),
            Operator3::Times => (Self::Cat, false),
            Operator3::Cat => (Self::Plus, true),
        }
    }

    pub fn double_next(&self) -> (Self, bool) {
        match self {
            Operator3::Plus => (Self::Cat, false),
            Operator3::Times => (Self::Plus, true),
            Operator3::Cat => (Self::Times, true),
        }
    }
}

impl Display for Operator3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Times => write!(f, "x"),
            Self::Cat => write!(f, "||"),
        }
    }
}

pub fn next_base_3(mut num: &mut Vec<Operator3>) {
    let mut carry = true;
    let mut n = Operator3::Plus;
    for i in 0..num.len() {
        (n, carry) = match carry {
            true => num[i].next(),
            false => (num[i].clone(), false),
        };
        num[i] = n;
    }
}

#[cfg(test)]
mod tests {
    use crate::{next_base_3, parse, Operator3};

    #[test]
    fn basic() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let expected_solvable = [
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            true
        ];

        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let equations = parse(&lines);
        let total: i64 = equations.iter().enumerate().map(|(idx, eq)|{
            let solution = eq.solve();
            let solvable = eq.solvable();
            assert_eq!(solvable, expected_solvable[idx]);
            if solvable {
                eq.left
            } else {
                0
            }
        }).sum();
        assert_eq!(total, 3749)
    }

    #[test]
    fn test_solve_3() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let expected_solvable = [
            true,
            true,
            false,
            true,
            true,
            false,
            true,
            false,
            true
        ];

        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let equations = parse(&lines);
        let total: i64 = equations.iter().enumerate().map(|(idx, eq)|{
            let solution = eq.solve_3();
            println!("{} :: {:?}", eq, solution);
            let solvable = solution.is_some();
            assert_eq!(solvable, expected_solvable[idx]);
            if solvable {
                eq.left
            } else {
                0
            }
        }).sum();
        assert_eq!(total, 11387)
    }

    #[test]
    fn test_solve_3_recursive() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let expected_solvable = [
            true,
            true,
            false,
            true,
            true,
            false,
            true,
            false,
            true
        ];

        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let equations = parse(&lines);
        let total: i64 = equations.iter().enumerate().map(|(idx, eq)|{
            let solvable = eq.solvable_3_rec();
            assert_eq!(solvable, expected_solvable[idx]);
            if solvable {
                eq.left
            } else {
                0
            }
        }).sum();
        assert_eq!(total, 11387)
    }

    #[test]
    fn test_solve_2_recursive() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let expected_solvable = [
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            true
        ];

        let lines: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
        let equations = parse(&lines);
        let total: i64 = equations.iter().enumerate().map(|(idx, eq)|{
            let solvable = eq.solvable_2_rec();
            assert_eq!(solvable, expected_solvable[idx]);
            if solvable {
                eq.left
            } else {
                0
            }
        }).sum();
        assert_eq!(total, 3749)
    }

    #[test]
    fn test_triple_next() {
        let mut a = vec![Operator3::Plus, Operator3::Plus, Operator3::Plus];
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
        next_base_3(&mut a);
        println!("{:?}", a);
    }
}
