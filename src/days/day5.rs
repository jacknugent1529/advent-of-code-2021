use std::fs;
use std::str::FromStr;
use std::cmp;
use std::collections::HashMap;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let lines: Vec<Line> = file_str.lines().map(|s| s.parse().unwrap()).collect();
    
    let mut max_x = lines[0].start.0;
    let mut min_x = lines[0].start.0;
    for l in &lines {
        max_x = cmp::max(max_x, cmp::max(l.start.0, l.end.0));
        min_x = cmp::min(min_x, cmp::min(l.start.0, l.end.0));
    }
    
    let mut counts: HashMap<(i32,i32), i32> = HashMap::new();
    
    for l in &lines {
        let mut points: Vec<(i32,i32)> = vec![];
        l.get_points(&mut points, false);
        for p in points.iter() {
            let count = counts.entry(*p).or_insert(0);
            *count += 1;
        }
    }
    Ok(counts.values().filter(|&x| *x > 1).count() as i32)
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let lines: Vec<Line> = file_str.lines().map(|s| s.parse().unwrap()).collect();
    
    let mut max_x = lines[0].start.0;
    let mut min_x = lines[0].start.0;
    for l in &lines {
        max_x = cmp::max(max_x, cmp::max(l.start.0, l.end.0));
        min_x = cmp::min(min_x, cmp::min(l.start.0, l.end.0));
    }
    
    let mut counts: HashMap<(i32,i32), i32> = HashMap::new();
    
    for l in &lines {
        let mut points: Vec<(i32,i32)> = vec![];
        l.get_points(&mut points, true);
        for p in points.iter() {
            let count = counts.entry(*p).or_insert(0);
            *count += 1;
        }
    }
    Ok(counts.values().filter(|&x| *x > 1).count() as i32)
}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn get_points(&self, points: &mut Vec<(i32,i32)>, allow_diagonals: bool) {
        if self.start.0 == self.end.0 { // vertical line
            for y in self.start.1..=self.end.1 {
                let x = self.start.0; 
                points.push((x, y));
            }
            return;
        }
        if !allow_diagonals && self.start.1 != self.end.1 {
            return;
        }
        let slope = (self.end.1 - self.start.1) / (self.end.0 - self.start.0);
        for x in self.start.0..=self.end.0 {
            points.push((x, self.start.1 + slope * (x-self.start.0)));
        }
    }
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split(" -> ");
        let parse_pair = |pair_str: &str| {
            let nums: Vec<i32> = pair_str.split(",").map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1])
        };
        let p1 = parse_pair(pairs.next().unwrap());
        let p2 = parse_pair(pairs.next().unwrap());
        let start = cmp::min(p1, p2); 
        let end = cmp::max(p1, p2);

        Ok(Line {
            start,
            end
        })
    }
}



