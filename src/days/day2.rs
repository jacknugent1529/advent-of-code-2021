use std::str::FromStr;
use std::fs;
use std::num::ParseIntError;

enum Direction {
    FORWARD,
    DOWN, 
    UP
}

struct Instruction {
    direction: Direction,
    number: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let direction = match split[0] {
            "forward" => Direction::FORWARD,
            "down" => Direction::DOWN,
            _ => Direction::UP,
        };
        let n = split[1].parse()?;
        Ok(Instruction{
            direction,
            number: n
        })
    }
}

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let instructions_res: Result<Vec<Instruction>, std::num::ParseIntError> = file_str.lines().map(|l| l.parse()).collect();
    let instructions = match instructions_res {
        Err(e) => return Err(e.to_string()),
        Ok(n) => n,
    };
    let mut total_up = 0;
    let mut total_forward = 0;
    for i in instructions {
        match i.direction {
            Direction::UP => total_up -= i.number,
            Direction::DOWN => total_up += i.number,
            Direction::FORWARD => total_forward += i.number,
        }
    }
    Ok(total_up * total_forward)
}
pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let instructions_res: Result<Vec<Instruction>, std::num::ParseIntError> = file_str.lines().map(|l| l.parse()).collect();
    let instructions = match instructions_res {
        Err(e) => return Err(e.to_string()),
        Ok(n) => n,
    };
    let mut depth = 0;
    let mut total_forward = 0;
    let mut aim = 0;
    for i in instructions {
        match i.direction {
            Direction::UP => aim -= i.number,
            Direction::DOWN => aim += i.number,
            Direction::FORWARD => { total_forward += i.number; depth += aim * i.number },
        }
    }
    Ok(depth * total_forward)
}