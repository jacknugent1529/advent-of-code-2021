use std::str::FromStr;
use std::fs;


pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let fishes: Vec<SnailFishNum> = file_str.lines().map(|s| s.parse().unwrap()).collect();
    
    let sum = fishes.into_iter().reduce(|acc, f| SnailFishNum::sum(acc, f)).expect("must have at least 2 fish");
    let magnitude = sum.magnitude();
    Ok(magnitude)
}


pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let fishes: Vec<SnailFishNum> = file_str.lines().map(|s| s.parse().unwrap()).collect();
    
    let mut max = 0;
    let num_fishes = fishes.len();
    for i in 0..num_fishes {
        for j in 0..num_fishes {
            if i == j {
                continue;
            }
            let s = SnailFishNum::sum(fishes[i].clone(), fishes[j].clone()).magnitude();
            if s > max { 
                max = s; 
            }

        }
    }
    Ok(max)
}


#[derive(Debug, Clone)]
enum SnailFishNum {
    Literal(i32),
    Pair { left: Box<SnailFishNum>, right: Box<SnailFishNum>}
}

impl SnailFishNum {
    fn new_pair(a: SnailFishNum, b: SnailFishNum) -> SnailFishNum {
        return SnailFishNum::Pair {
            left: Box::new(a),
            right: Box::new(b)
        }
    }
    fn new_literal(x: i32) -> SnailFishNum {
        return SnailFishNum::Literal(x)
    }
    fn parse_partial(chars: &Vec<char>, idx: usize) -> Result<(SnailFishNum, usize), String> {

        match chars[idx] {
            '[' => { // inner pair
                let (left, idx_after_left) = Self::parse_partial(chars, idx + 1)?;
                if chars[idx_after_left] != ',' { 
                    return Err(format!("Expected ',', found {} at idx {}", chars[idx_after_left], idx_after_left)); 
                }
                let curr = idx_after_left + 1;
                let (right, idx_after_right) = Self::parse_partial(chars, curr)?;
                let fish = SnailFishNum::new_pair(left, right);

             
                let curr = idx_after_right + 1; // for ']'

             
                // println!("{:?}", fish);
                Ok((fish, curr))
            }
            c => {
                match c.to_digit(10) {
                    Some(d) => {
                        let fish = SnailFishNum::Literal(d as i32);
                        Ok((fish, idx + 1))
                    }
                    None => Err(format!("Expected digit, found '{}' at idx {}", c, idx))
                }
            }
         
        }
    }
    fn magnitude(&self) -> i32 {
        match self {
            SnailFishNum::Literal(x) => *x,
            SnailFishNum::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude()
        }
    }
    fn sum(a: SnailFishNum, b: SnailFishNum) -> SnailFishNum {
        let mut result = SnailFishNum::Pair{left: Box::new(a), right: Box::new(b)};
        
        loop {
            let exploded = result.explode();
            if !exploded {
                if !result.split() {
                    return result;
                }
            }
        }
    }
    
    fn explode(&mut self) -> bool {
        return self.explode_r(0).0
    }
    
    fn explode_r(&mut self, depth: i32) -> (bool, Option<i32>, Option<i32>) {
        match self {
            SnailFishNum::Pair { left, right } => {
                if depth >= 4 {
                    if let SnailFishNum::Literal(left_p) = left.as_ref() { 
                        if let SnailFishNum::Literal(right_p) = right.as_ref() {
                            let left_val = *left_p;
                            let right_val = *right_p;
                            *self = SnailFishNum::new_literal(0);
                            return (true, Some(left_val), Some(right_val))
                        }
                    }
                }
                // first explode left
                let (left_exploded, add_left, add_right) = left.explode_r(depth + 1);
                if left_exploded {
                    // since this is the left of the pair, we know we can add to the right
                    if let Some(r) = add_right {
                        right.add_to_leftmost(r);
                    }
                    // add_left is some
                    return (true, add_left, None)
                }
                
                // now try to explode right
                let (right_exploded, add_left, add_right) = right.explode_r(depth + 1);
                if right_exploded {
                    // since this is the right of the pair, we know we can add to the left
                    if let Some(l) = add_left {
                        left.add_to_rightmost(l);
                    }
                    return (true, None, add_right);
                }
                
                (false, None, None)
            },
            _ => (false, None, None)
        }
    }
    
    fn add_to_leftmost(&mut self, x: i32) {
        match self {
            SnailFishNum::Literal(p) => {*p += x; }
            SnailFishNum::Pair{left, ..} => {left.add_to_leftmost(x); }
        }
    }

    fn add_to_rightmost(&mut self, x: i32) {
        match self {
            SnailFishNum::Literal(p) => {*p += x; }
            SnailFishNum::Pair{right, ..} => {right.add_to_rightmost(x); }
        }
    }
    
    fn split(&mut self) -> bool {
        match self {
            SnailFishNum::Literal(x) => {
                if *x >= 10 {
                    let left = *x / 2;
                    let right = *x - left;
                    *self = SnailFishNum::new_pair(SnailFishNum::new_literal(left), SnailFishNum::new_literal(right));
                    return true;
                }
                return false;
            }
            SnailFishNum::Pair { left, right } => {
                if left.split() {
                    return true;
                }
                return right.split();
            }
        }
    }
}

impl FromStr for SnailFishNum {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
     
        let (snail, _) = Self::parse_partial(&chars, 0)?;
        return Ok(snail)
    }
}