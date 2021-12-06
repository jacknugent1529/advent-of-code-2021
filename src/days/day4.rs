use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut blocks = file_str.split("\n\n");

    let drawn_nums_str = blocks.next().unwrap();
    let drawn_nums: Vec<i32> = drawn_nums_str.trim().split(',').map(|s| s.parse().unwrap()).collect();
    
    let mut boards: Vec<Board> = blocks.map(|b| b.parse().unwrap()).collect();
    
    for num in drawn_nums {
        for i in 0..boards.len() {
            let b = &mut boards[i];
            b.draw_num(num);
            if b.check_win() {
                return Ok(b.sum_unchecked() * num);
            }
        }
    }
    

    Err("ERROR: Not implemented".to_string())
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut blocks = file_str.split("\n\n");

    let drawn_nums_str = blocks.next().unwrap();
    let drawn_nums: Vec<i32> = drawn_nums_str.trim().split(',').map(|s| s.parse().unwrap()).collect();
    
    let mut boards: Vec<Board> = blocks.map(|b| b.parse().unwrap()).collect();
    
    let mut last_win_val = 0;
    let mut boards_remaining = boards.len();
    for num in drawn_nums {
        for i in 0..boards.len() {
            let b = &mut boards[i];
            b.draw_num(num);
            if !b.game_won && b.check_win() {
                last_win_val = b.sum_unchecked() * num;
            }
        }
    }
    Ok(last_win_val)
}

#[derive(Debug)]
struct Board {
    values: [[i32; 5]; 5],
    checked: [[bool; 5]; 5],
    game_won: bool,
}

impl Board {
    fn check_win(&mut self) -> bool {
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                if !self.checked[i][j] { 
                    win = false; 
                    break; 
                }
            }
            if win { self.game_won = true; return true; }

        }
        for j in 0..5 {
            let mut win = true;
            for i in 0..5 {
                if !self.checked[i][j] { 
                    win = false; 
                    break; 
                }
            }
            if win { self.game_won = true; return true; }
        }
        return false;
    } 
    fn draw_num(&mut self, num: i32) -> bool {
        for (i,row) in self.values.into_iter().enumerate() {
            for (j, val) in row.into_iter().enumerate() {
                if *val == num {
                    self.checked[i][j] = true;
                    return true;
                }
            }
        }
        return false;
    }
    fn sum_unchecked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.checked[i][j] {
                    sum += self.values[i][j];
                }
            }
        }
        return sum;
    }
}

impl FromStr for Board {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board_arr: [[i32; 5]; 5] = Default::default();
        for (i,line) in s.lines().enumerate() {
            let mut j = 0;
            for c in line.trim().split(' ') {
                match c.trim().parse() {
                    Err(_) => continue,
                    Ok(n) => { board_arr[i][j] = n; j += 1; }
                }
            }
        }
        let checked: [[bool; 5]; 5] = Default::default();
        let mut board = Board {
            values: board_arr,
            checked,
            game_won: false,
        };
        board.check_win();
        return Ok(board);
    }
}






