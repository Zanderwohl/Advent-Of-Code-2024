mod util;

use std::error::Error;
use std::time::Instant;
use util::parsing;

type Num = u32;


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let lines = parsing::file_into_vec("files/day_12_input.txt")?;
    let mut garden = parse_input(&lines);

    let (price, bulk_price) = find_total_price(&mut garden);
    println!("Price of all fence is:\n{}", price);
    println!("Price of all fence in bulk is:\n{}", bulk_price);

    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);

    Ok(())
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|l| l.chars().collect()).collect()
}

pub fn debug_print_garden(garden: &Vec<Vec<char>>) {
    for line in garden {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

pub fn find_total_price(garden: &mut Vec<Vec<char>>) -> (Num, Num) {
    let mut total = 0;
    let mut bulk_total = 0;
    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            let plant = garden[i][j];
            if !plant.is_lowercase() {
                let (area, perimeter, n_corners) = consume_plot(garden, i, j);
                // println!("Plant {} has an area of {} and a perimeter of {} for a cost of {}", plant, area, perimeter, area * perimeter);
                // println!("Plant {} has an area of {} and {} corners for a cost of {}", plant, area, n_corners, area * n_corners);
                total += area * perimeter;
                bulk_total += area * n_corners;
                // debug_print_garden(&garden);
                // println!("-----");
            }
        }
    }
    (total, bulk_total)
}

fn consume_plot(garden: &mut Vec<Vec<char>>, i: usize, j: usize) -> (Num, Num, Num) {
    let kind = garden[i][j];
    if kind.is_lowercase() {
        return (0, 0, 0);
    }
    garden[i][j] = garden[i][j].to_ascii_lowercase();
    let neighbors = same_neighbors(garden, i, j, kind);
    let mut area = 1;
    let mut perimeter = 4 - (neighbors.len() as Num);
    let mut n_corners = n_corners(garden, kind, i, j);
    /*println!("\t<Resolving Neighbor {}, {}>", i, j);
    println!("+{} area, +{} perimeter", area, perimeter);
    debug_print_garden(&garden);
    println!("{:?}", neighbors);
    println!("\t</Resolving Neighbor>");*/
    for neighbor in neighbors {
        match neighbor {
            None => {}
            Some((neighbor_i, neighbor_j)) => {
                let (neighbor_area, neighbor_perimeter, neighbor_corners) = consume_plot(garden, neighbor_i, neighbor_j);
                area += neighbor_area;
                perimeter += neighbor_perimeter;
                n_corners += neighbor_corners;
            }
        }

    }

    (area, perimeter, n_corners)
}

fn same_neighbors(garden: &Vec<Vec<char>>, i: usize, j: usize, kind: char) -> Vec<Option<(usize, usize)>> {
    let mut neighbors: Vec<Option<(usize, usize)>> = Vec::new();
    let kind_lower = kind.to_ascii_lowercase();

    if i > 0  {
        if garden[i - 1][j] == kind_lower {
            neighbors.push(None);
        }
        if garden[i - 1][j] == kind {
            neighbors.push(Some((i - 1, j)));
        }
    }
    if j > 0 {
        if garden[i][j - 1] == kind_lower {
            neighbors.push(None);
        }
        if garden[i][j - 1] == kind {
            neighbors.push(Some((i, j - 1)));
        }
    }
    if i < garden.len() - 1 {
        if garden[i + 1][j] == kind_lower {
            neighbors.push(None);
        }
        if garden[i + 1][j] == kind {
            neighbors.push(Some((i + 1, j)));
        }
    }
    if j < garden[i].len() - 1 {
        if garden[i][j + 1] == kind_lower {
            neighbors.push(None);
        }
        if garden[i][j + 1] == kind {
            neighbors.push(Some((i, j + 1)));
        }
    }

    neighbors
}

pub fn n_corners(garden: &mut Vec<Vec<char>>, kind: char, i: usize, j: usize) -> Num {
    let kind_lower = kind.to_ascii_lowercase();

    let n_0_0: bool = i > 0 && j > 0 && (garden[i - 1][j - 1] == kind_lower || garden[i - 1][j - 1] == kind);
    let n_0_1: bool = i > 0 && (garden[i - 1][j] == kind_lower || garden[i - 1][j] == kind);
    let n_0_2: bool = i > 0 && j < garden[i].len() - 1 && (garden[i - 1][j + 1] == kind_lower || garden[i - 1][j + 1] == kind);

    let n_1_0: bool = j > 0 && (garden[i][j - 1] == kind_lower || garden[i][j - 1] == kind);
    let n_1_1: bool = true;
    let n_1_2: bool = j < garden[i].len() - 1 && (garden[i][j + 1] == kind_lower || garden[i][j + 1] == kind);

    let n_2_0: bool = i < garden.len() - 1 && j > 0 && (garden[i + 1][j - 1] == kind_lower || garden[i + 1][j - 1] == kind);
    let n_2_1: bool = i < garden.len() - 1 && (garden[i + 1][j] == kind_lower || garden[i + 1][j] == kind);
    let n_2_2: bool = i < garden.len() - 1 && j < garden[i].len() - 1 && (garden[i + 1][j + 1] == kind_lower || garden[i + 1][j + 1] == kind);

    // n_0_0 n_0_1 n_0_2
    // n_1_0 n_1_1 n_1_2
    // n_2_0 n_2_1 n_2_2

    /*println!("{}, {}", i, j);
    if j > 0 && i > 0 && i < garden.len() - 1 && j < garden[i].len() - 1 {
        println!("{}{}{}", garden[i - 1][j - 1].to_ascii_uppercase(), garden[i - 1][j].to_ascii_uppercase(), garden[i - 1][j + 1].to_ascii_uppercase());
        println!("{}{}{}", garden[i][j - 1].to_ascii_uppercase(), garden[i][j].to_ascii_uppercase(), garden[i][j + 1].to_ascii_uppercase());
        println!("{}{}{}", garden[i + 1][j - 1].to_ascii_uppercase(), garden[i + 1][j].to_ascii_uppercase(), garden[i + 1][j + 1].to_ascii_uppercase());
        println!();
        println!("{}{}{}", n_0_0 as u32, n_0_1 as u32, n_0_2 as u32);
        println!("{}{}{}", n_1_0 as u32, n_1_1 as u32, n_1_2 as u32);
        println!("{}{}{}", n_2_0 as u32, n_2_1 as u32, n_2_2 as u32);
    }*/

    let top_left = !n_0_0 && (n_0_1 == n_1_0);
    let top_right = !n_0_2 && (n_0_1 == n_1_2);
    let bottom_right = !n_2_2 && (n_1_2 == n_2_1);
    let bottom_left = !n_2_0 && (n_1_0 == n_2_1);

    let top_left_special = n_0_0 && (!n_0_1 && !n_1_0);
    let top_right_special = n_0_2 && (!n_0_1 && !n_1_2);
    let bottom_right_special = n_2_2 && (!n_1_2 && !n_2_1);
    let bottom_left_special = n_2_0 && (!n_1_0 && !n_2_1);

    /*println!("top_left: {}, top_right: {}, bottom_right: {}, bottom_left: {}", top_left, top_right, bottom_right, bottom_left);
    println!();*/

    (top_left as Num)
        + (top_right as Num)
        + (bottom_right as Num)
        + (bottom_left as Num)
        + (top_left_special as Num)
        + (top_right_special as Num)
        + (bottom_right_special as Num)
        + (bottom_left_special as Num)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::{debug_print_garden, find_total_price, parse_input};
    use crate::util::parsing;

    #[test]
    fn test_tiny() -> Result<(), Box<dyn Error>> {
        let lines = parsing::file_into_vec("files/day_12_tiny.txt")?;
        let mut garden = parse_input(&lines);

        let price_expected = 140;
        let (price_actual, bulk_price_actual) = find_total_price(&mut garden);

        assert_eq!(price_actual, price_expected);

        let bulk_price_expected = 80;

        assert_eq!(bulk_price_actual, bulk_price_expected);

        Ok(())
    }

    #[test]
    fn test_small() -> Result<(), Box<dyn Error>> {
        let lines = parsing::file_into_vec("files/day_12_small.txt")?;
        let mut garden = parse_input(&lines);

        let price_expected = 1930;
        let (price_actual, bulk_price_actual) = find_total_price(&mut garden);

        assert_eq!(price_actual, price_expected);

        let bulk_price_expected = 1206;

        assert_eq!(bulk_price_actual, bulk_price_expected);

        Ok(())
    }

    #[test]
    fn test_xoxo() -> Result<(), Box<dyn Error>> {
        let lines = parsing::file_into_vec("files/day_12_xoxo.txt")?;
        let mut garden = parse_input(&lines);
        let (price_actual, bulk_price_actual) = find_total_price(&mut garden);
        assert_eq!(bulk_price_actual, 436);
        Ok(())
    }

    #[test]
    fn test_e() -> Result<(), Box<dyn Error>> {
        let lines = parsing::file_into_vec("files/day_12_e.txt")?;
        let mut garden = parse_input(&lines);
        let (price_actual, bulk_price_actual) = find_total_price(&mut garden);
        assert_eq!(bulk_price_actual, 236);
        Ok(())
    }

    #[test]
    fn test_ab() -> Result<(), Box<dyn Error>> {
        let lines = parsing::file_into_vec("files/day_12_ab.txt")?;
        let mut garden = parse_input(&lines);
        let (price_actual, bulk_price_actual) = find_total_price(&mut garden);
        assert_eq!(bulk_price_actual, 368);
        Ok(())
    }
}