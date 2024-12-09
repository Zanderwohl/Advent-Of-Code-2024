use std::error::Error;
use std::fmt::{write, Display, Formatter};
use std::path::Path;
use crate::util::bytewise::ByteIterator;

mod util;

type Num = u16;

const ASCII_ZERO: u8 = 48;

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = ByteIterator::new("files/day_9_input.txt").unwrap();
    let disk = parse(bytes);
    let part_1 = solve_part_1(disk);
    println!("The checksum of the fragmented disk is:\n{}", part_1);
    let bytes = ByteIterator::new("files/day_9_input.txt").unwrap();
    let disk_backup = parse_sillymode(bytes);
    let (part_2, changed_disk) = solve_part_2(disk_backup);
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

pub fn parse_sillymode(bytes: ByteIterator) -> Vec<Block> {
    let mut disk = Vec::with_capacity(bytes.len());
    let mut is_file = true;
    let mut file_id: Num = 0;
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
        disk.push(Block {
            id: n,
            size,
        });
    }
    for block in &disk {
        print!("{}", block)
    }
    println!();
    disk
}

struct Block {
    id: Num,
    size: u8,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.id == Num::MAX {
            write!(f, "[{}; _]", self.size)
        } else {
            let n = (self.id as u8 + 48) as char;
            write!(f, "[{};{}]", self.size, n)
        }
    }
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

pub fn solve_part_2(mut disk: Vec<Block>) -> (u64, Vec<Block>) {
    let mut back: usize = disk.len() - 1;
    let limit: usize = disk.len() * 2;
    let mut loops: usize = 0;

    'quit: while back > 0 && loops < limit {
        loops += 1;

        while disk[back].id == Num::MAX {
            back -= 1;
            if back == 0 {
                break 'quit;
            }
        }
        let mut front: usize = 0;
        while front < disk.len() && !(disk[front].id == Num::MAX && disk[front].size >= disk[back].size) {
            front += 1;
        }
        if front < disk.len() {
            println!("swapping");
            if disk[front].size > disk[back].size {
                disk.insert(front + 1, Block {
                    id: Num::MAX,
                    size: disk[front].size - disk[back].size,
                });
                back += 1;
            }
            disk[front].id = disk[back].id;
            disk[front].size = disk[back].size;
            disk[back].id = Num::MAX;
        }
    }


    (0, disk)
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
    use crate::{parse, parse_sillymode, solve_part_1, solve_part_2};
    use crate::util::bytewise::ByteIterator;

    #[test]
    fn tiny_input() {
        let bytes = ByteIterator::new("files/day_9_tiny.txt").unwrap();
        let disk = parse(bytes);
        let part_1 = solve_part_1(disk);
        let bytes = ByteIterator::new("files/day_9_tiny.txt").unwrap();
        let disk_backup = parse_sillymode(bytes);
        for block in &disk_backup {
            print!("{}", block);
        }
        println!();
        let (checksum, changed_disk) = solve_part_2(disk_backup);
        for block in changed_disk {
            print!("{}", block);
        }
        println!();
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
        let bytes = ByteIterator::new("files/day_9_small.txt").unwrap();
        let disk = parse(bytes);
        let part_1 = solve_part_1(disk);
        let bytes = ByteIterator::new("files/day_9_small.txt").unwrap();
        let disk_backup = parse_sillymode(bytes);
        for block in &disk_backup {
            print!("{}", block);
        }
        println!();
        let (checksum, changed_disk) = solve_part_2(disk_backup);
        for block in changed_disk {
            print!("{}", block);
        }
        assert_eq!(part_1, 1928);
    }

}
