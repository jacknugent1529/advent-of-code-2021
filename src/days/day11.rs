use std::fs;
use std::str::FromStr;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut grid: Grid = file_str.parse().unwrap();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += grid.step();
    }
    Ok(flashes)
}

struct Grid {
    arr: [[i32; 10]; 10],
}

impl Grid {
    fn flash(&mut self, i: i32, j: i32, has_flashed: &mut[[bool; 10]; 10]) -> i32 {
        let i_u = i as usize;
        let j_u = j as usize;
        if has_flashed[i_u][j_u] {
            return 0
        }
        self.arr[i_u][j_u] += 1;
        let mut flashed = 0;
        // println!("{:?}", self.arr);
        if self.arr[i_u][j_u] > 9 {
            flashed += 1;
            has_flashed[i_u][j_u] = true;
            self.arr[i_u][j_u] = 0;
            for (di, dj) in [(1,1), (0,1), (-1,1), (1,0), (-1,0), (-1,-1), (0,-1), (1,-1)].iter() {
                let x = i + di;
                let y = j + dj;
                if 0 <= x && x < 10 && 0 <= y && y < 10 {
                    flashed += self.flash(x,y, has_flashed);
                }
            }
        } 
        return flashed;
    }
    fn step(&mut self) -> i32 {
        let mut flashes = 0;
        let mut has_flashed = [[false; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                flashes += self.flash(i,j, &mut has_flashed);
            }
        }
        return flashes
    }
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arr = [[0; 10]; 10];
        for (i,l) in s.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                arr[i][j] = c.to_digit(10).unwrap() as i32;
            }
        }
        Ok(Grid{
            arr,
        })
    }
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut grid: Grid = file_str.parse().unwrap();
    let mut step = 1;
    loop {
        if grid.step() == 100 {
            return Ok(step)
        };
        step += 1;
    }
}