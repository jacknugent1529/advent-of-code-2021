use std::fs;
use std::str::FromStr;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut split = file_str.split("\n\n");
    let mut paper: Paper = split.next().unwrap().parse().unwrap();
    let instructions: Vec<Instruction> = split.next().unwrap().lines().map(|l| l.parse().unwrap()).collect();
    
    paper.fold(&instructions[0]);
    
    Ok(paper.num_dots())
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut split = file_str.split("\n\n");
    let mut paper: Paper = split.next().unwrap().parse().unwrap();
    let instructions: Vec<Instruction> = split.next().unwrap().lines().map(|l| l.parse().unwrap()).collect();
    
    for i in instructions.iter() {
        paper.fold(i);
    }
    paper.print_grid();
    
    Ok(0) // read output for actual solution
}

enum Instruction {
    Up(usize),
    Left(usize)
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("instr: {}", s);
        // println!("instr: {:?}", s[13..].to_string());
        let n = s[13..].parse::<usize>().unwrap();
        match s.chars().nth(11) {
            Some('y') => Ok(Instruction::Up(n)),
            Some('x') => Ok(Instruction::Left(n)),
            _ => Err("Incorrect instruction format".to_string())
        }
    }
}

#[derive(Debug)]
struct Paper {
    arr: Box<Vec<Vec<bool>>> // false represents dot
}

impl Paper {
    fn new(arr: Box<Vec<Vec<bool>>>) -> Paper {
        return Paper{ arr }
    }
    
    fn fold(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Left(n) => self.fold_vertical(*n),
            Instruction::Up(n) => self.fold_horizontal(*n)
        }
    }
    
    fn fold_vertical(&mut self, n: usize) {
        // fold along vertical line
        let nrows = self.arr.len();
        let mut new_arr = Box::new(vec![vec![false; n]; nrows]);
        for i in 0..nrows {
            for j in 0..n {
                if self.arr[i][j] || (self.arr[i][2 * n - j]) {
                    new_arr[i][j] = true;
                }
            }
        }
        self.arr = new_arr;
    }

    fn fold_horizontal(&mut self, n: usize) {
        // fold along horizontal line
        let ncols = self.arr[0].len();
        let mut new_arr = Box::new(vec![vec![false; ncols]; n]);
        for i in 0..n {
            for j in 0..ncols {
                if self.arr[i][j] || (self.arr[2 * n - i][j]) {
                    new_arr[i][j] = true;
                }
            }
        }
        self.arr = new_arr;
    }
    
    fn num_dots(&self) -> i32 {
        let mut n = 0;
        for i in 0..self.arr.len() {
            for j in 0..self.arr[0].len() {
                if self.arr[i][j] {
                    n += 1;
                }
            }
        }
        return n;
    }
    
    fn print_grid(&self) {
        for i in 0..self.arr.len() {
            let mut s = String::new();
            for j in 0..self.arr[0].len() {
                if self.arr[i][j] {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            println!("{}", s);
        }
    }
}

impl FromStr for Paper {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nrows = 0;
        let mut ncols = 0;
        let mut pairs = vec![];
        for l in s.lines() {
            let xy: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
            if xy[1] + 1 > nrows { nrows = xy[1] + 1 }
            if xy[0] + 1 > ncols { ncols = xy[0] + 1 }
            pairs.push((xy[1], xy[0]));
        }
        let mut arr = Box::new(vec![vec![false; ncols]; nrows]);
        for (i,j) in pairs {
            arr[i][j] = true;
        }
        Ok(Paper {
            arr
        })
    }
}