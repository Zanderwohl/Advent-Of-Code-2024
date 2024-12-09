use std::error::Error;
use std::fmt::{write, Display, Formatter};
use std::path::Path;
use crate::util::bytewise::ByteIterator;

mod util;

type Num = u16;

const ASCII_ZERO: u8 = 48;

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = ByteIterator::new("files/day_09_input.txt").unwrap();
    let disk = parse(bytes);
    let disk_backup = disk.clone();
    let part_1 = solve_part_1(disk);
    println!("The checksum of the fragmented disk is:\n{}", part_1);
    let part_2 = solve_part_2(disk_backup);
    println!("The checksum of the sorted but unfragmented disk is:\n{}", part_2);

    Ok(())
}

pub fn parse(bytes: ByteIterator) -> Vec<Num> {
    let mut disk: Vec<Num> = Vec::new();
    let mut file_id: Num = 0;
    let mut is_file = true;
    'exit: for byte in bytes {
        if byte.is_err() {
            break 'exit;
        }
        let byte = byte.unwrap();
        if byte < 48 {
            break 'exit;
        }
        let size = byte - ASCII_ZERO;
        let n = match is_file {
            true => {
                is_file = false;
                let temp = file_id;
                file_id += 1;
                temp
            }
            false => {
                is_file = true;
                Num::MAX
            },
        };
        for _ in 0..size {
            disk.push(n);
        }
    }
    disk
}

pub fn solve_part_1(mut disk: Vec<Num>) -> u64 {
    let mut front: usize = 0;
    let mut back: usize = disk.len() - 1;
    let limit: usize = disk.len();
    let mut loops: usize = 0;
    'quit: while front < back && loops < limit {
        loops += 1;

        while disk[front] != Num::MAX {
            front += 1;
            if front == back {
                break 'quit;
            }
        }
        while disk[back] == Num::MAX {
            back -= 1;
            if front == back {
                break 'quit;
            }
        }
        disk[front] = disk[back];
        disk[back] = Num::MAX;
        // debug_print(&disk);
    }

    checksum(&disk)
}

pub fn solve_part_2(mut disk: Vec<Num>) -> u64 {
    // println!("{:?}", disk);
    let mut back: usize = disk.len() - 1;
    let limit: usize = disk.len();
    let mut loops: usize = 0;
    'quit: while back > 0 {
        loops += 1;

        while disk[back] == Num::MAX {
            back -= 1;
            if back <= 0 {
                // println!("A");
                break 'quit;
            }
        }
        let current_id = disk[back];
        let mut back_start = back;
        'found: while disk[back_start] == current_id {
            back_start -= 1;
            if back_start <= 0 {
                // println!("B");
                break 'quit;
            }
            if disk[back_start] != current_id {
                back_start += 1;
                break 'found;
            }
        }
        /*print!("[");*/
        let range_to_move = &disk[back_start..=back];
        let required_length = range_to_move.len();
        /*for i in back_start..=back {
            print!("{}", disk[i] as u32)
        }
        println!("] ({})", required_length);*/

        let mut front = 0;
        let mut front_end = 0;
        'outer: while front + required_length < back_start {
            while disk[front] != Num::MAX && front + 1 < back_start {
                front += 1;
                if disk[front] == Num::MAX {
                    front_end = front;
                    while disk[front_end] == Num::MAX {
                        front_end += 1;
                    }
                    if front_end - front >= required_length {
                        break 'outer;
                    }
                }
            }
            front += 1;
        }

        // println!("{} -> {}", front, front_end);
        if front_end >= front && front_end - front >= required_length {
            // println!("SWAP");
            for i in 0..required_length {
                disk[front + i] = disk[back_start + i];
                disk[back_start + i] = Num::MAX;
            }
        }

        //debug_print(&disk);
        back -= required_length;
    }
    // println!("{}/{}", loops, limit);

    checksum(&disk)
}

pub fn checksum(disk: &Vec<Num>) -> u64 {
    let mut sum: u64 = 0;
    for (idx, id) in disk.iter().enumerate() {
        let id = *id;
        if id != Num::MAX {
            sum += (id as u64) * (idx as u64)
        }
    }
    sum
}

pub fn debug_print(disk: &Vec<Num>) {
    for byte in disk {
        if *byte == Num::MAX {
            print!(".");
        } else {
            print!("{}", byte)
        }
    }
    println!()
}

pub fn debug_string(disk: &Vec<Num>) -> String {
    let mut result = String::with_capacity(disk.len());
    for byte in disk {
        if *byte == Num::MAX {
            result.push('.');
        } else {
            result.push((*byte as u8 + 48) as char)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{parse, solve_part_1, solve_part_2};
    use crate::util::bytewise::ByteIterator;

    #[test]
    fn tiny_input() {
        let bytes = ByteIterator::new("files/day_09_tiny.txt").unwrap();
        let disk = parse(bytes);
        let disk_backup = disk.clone();
        let part_1 = solve_part_1(disk);
        println!();
        let part_2 = solve_part_2(disk_backup);
        let steps = [
            "0..111....22222",
            "02.111....2222.",
            "022111....222..",
            "0221112...22...",
            "02211122..2....",
            "022111222......"
        ];
    }

    #[test]
    fn small_input() {
        "2333133121414131402";
        let bytes = ByteIterator::new("files/day_09_small.txt").unwrap();
        let disk = parse(bytes);
        let disk_backup = disk.clone();
        let part_1 = solve_part_1(disk);
        assert_eq!(part_1, 1928);
        let part_2 = solve_part_2(disk_backup);
    }

}
