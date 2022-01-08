use std::fs;
use std::collections::HashMap;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut lines = file_str.lines();
    let p1_pos = lines.next().unwrap().split(": ").skip(1).next().unwrap().parse::<i32>().unwrap() - 1;
    let p2_pos = lines.next().unwrap().split(": ").skip(1).next().unwrap().parse::<i32>().unwrap() - 1;
    let dice_rolls: Vec<i32> = (1..=100).collect();
    
    let mut game = Game {
        p1_pos,
        p2_pos,
        p1_score: 0,
        p2_score: 0,
        dice_rolls,
        rolls_taken: 0,
        is_p1_turn: true,
    };

    //println!("game state: {:?}", game);
    //for _ in 0..3 {
    loop {
        if game.take_turn() {
            // winner
            println!("game final state: {:?}", game);
            let losing_score = if game.p1_score < game.p2_score { game.p1_score } else { game.p2_score };
            return Ok(losing_score * (game.rolls_taken as i32));
        }
        //println!("game state: {:?}", game);
    }
    Err("ERROR: Not implemented".to_string())
}

pub fn soln_b(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut lines = file_str.lines();
    let p1_pos = lines.next().unwrap().split(": ").skip(1).next().unwrap().parse::<i32>().unwrap() - 1;
    let p2_pos = lines.next().unwrap().split(": ").skip(1).next().unwrap().parse::<i32>().unwrap() - 1;
    let dice_rolls: Vec<i32> = (1..=100).collect();
    
    let mut game = DiracGame {
        p1_pos,
        p2_pos,
        p1_score: 0,
        p2_score: 0,
        is_p1_turn: true,
        dice_sum: 0,
        num_dice_rolls: 0,
    };

    let mut memos = HashMap::new();
    let (p1, p2) = game.calc_winners(&mut memos);
    if p1 > p2 {
        Ok(p1)
    } else {
        Ok(p2)
    }
}

#[derive(Debug)]
struct Game {
    p1_pos: i32,
    p2_pos: i32,
    p1_score: i32,
    p2_score: i32,
    dice_rolls: Vec<i32>,
    rolls_taken: usize,
    is_p1_turn: bool,
}

impl Game {
    fn next_rolls(&mut self, n: i32) -> i32 {
        let mut sum = 0;
        for i in 0..n {
            sum += self.dice_rolls[self.rolls_taken % self.dice_rolls.len()];
            //println!("num: {}", self.dice_rolls[self.rolls_taken % self.dice_rolls.len()]);
            self.rolls_taken += 1;
        }
        sum
    }
    fn take_turn(&mut self) -> bool {
        let dist: i32 = self.next_rolls(3);
        if self.is_p1_turn {
            self.p1_pos += dist;
            self.p1_pos %= 10;
            self.p1_score += self.p1_pos + 1;
            if self.p1_score >= 1000 {
                return true;
            }
        } else {
            self.p2_pos += dist;
            self.p2_pos %= 10;
            self.p2_score += self.p2_pos + 1;
            if self.p2_score >= 1000 {
                return true;
            }
        }
        self.is_p1_turn = !self.is_p1_turn;
        return false;
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiracGame {
    p1_pos: i32,
    p2_pos: i32,
    p1_score: i32,
    p2_score: i32,
    is_p1_turn: bool,
    dice_sum: i32,
    num_dice_rolls: i32
}

impl DiracGame {
    fn calc_winners(&mut self, memos: &mut HashMap<DiracGame, (u64, u64)>) -> (u64, u64) {
        if self.num_dice_rolls == 3 {
            if self.is_p1_turn {
                self.p1_pos += self.dice_sum;
                self.p1_pos %= 10;
                self.p1_score += self.p1_pos + 1;
                if self.p1_score >= 21 {
                    return (1,0);
                }
            } else {
                self.p2_pos += self.dice_sum;
                self.p2_pos %= 10;
                self.p2_score += self.p2_pos + 1;
                if self.p2_score >= 21 {
                    return (0,1);
                }
            }
            self.is_p1_turn = !self.is_p1_turn;
            self.num_dice_rolls = 0;
            self.dice_sum = 0;
        }
        if let Some(res) = memos.get(&self) {
            return *res;
        }
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        for r in 1..=3 {
            let (p1, p2) = DiracGame {
                dice_sum: self.dice_sum + r,
                num_dice_rolls: self.num_dice_rolls + 1,
                ..self.clone()
            }.calc_winners(memos);
            p1_wins += p1;
            p2_wins += p2;
        }
        memos.insert(self.clone(), (p1_wins, p2_wins));
        (p1_wins, p2_wins)
    }
}

