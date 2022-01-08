use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let heights: HeightMap = file_str.parse().unwrap();
    let is_low_point = |i: i8, j: i8| { let h = heights.get(i, j).unwrap(); return heights.get_adjacent(i, j).iter().filter(|maybe_h| maybe_h.is_some()).all(|&h_adj| h_adj.unwrap() > h); };
    
    let mut sum = 0;
    for i in 0..heights.nrows {
        for j in 0..heights.ncols {
            if is_low_point(i as i8, j as i8) {
                sum += heights.get(i as i8, j as i8).unwrap() + 1;
            }
        }
    }
    Ok(sum as i32)
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let heights: HeightMap = file_str.parse().unwrap();
    let is_low_point = |i: i8, j: i8| { let h = heights.get(i, j).unwrap(); return heights.get_adjacent(i, j).iter().filter(|maybe_h| maybe_h.is_some()).all(|&h_adj| h_adj.unwrap() > h); };
    
    let mut largest_3 = [0,0,0];
    for i in 0..heights.nrows {
        for j in 0..heights.ncols {
            if is_low_point(i as i8, j as i8) {
                let size = heights.get_basin_size(i as i8,j as i8);
                let (i, &min) = largest_3.iter().enumerate().min_by(|(_,&v), (_,&q)| v.cmp(&q)).unwrap();
                // println!("size: {}", size);
                // println!("i: {}, min: {}", i, min);
                if min < size {
                    largest_3[i] = size;
                }
            }
        }
    }
    Ok(largest_3.iter().fold(1, |acc, &x| acc * x) as i32)
    
    // Err("ERROR: Not implemented".to_string())
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<u32>,
    nrows: usize,
    ncols: usize,
}

impl HeightMap {
    fn get(&self, i: i8, j: i8) -> Option<u32> {
        if ( i as usize ) >= self.nrows || ( j as usize ) >= self.ncols || i < 0 || j < 0 {
            return None;
        }
        return Some(self.heights[( i as usize ) * self.ncols + ( j as usize )]);
    }
    fn get_adjacent(&self, i: i8, j: i8) -> Vec<Option<u32>> {
        // right, above, left, below
        let mut adjacent = vec![];
        adjacent.push(self.get(i, j + 1)); // right
        adjacent.push(self.get(i-1, j)); // above
        adjacent.push(self.get(i, j - 1)); // right
        adjacent.push(self.get(i+1, j)); // below
        adjacent
    }
    
    fn get_basin_size(&self, i: i8, j: i8) -> i32 {
        let mut size = 0;
        let mut q: Vec<(i8, i8)> = vec![(i,j)];
        let mut visited: HashSet<(i8, i8)> = HashSet::new();
        while q.len() > 0 {
            let curr = q.pop();
            if let Some((i,j)) = curr {
                if visited.contains(&(i,j)) {
                    continue;
                }
                if let Some(val) = self.get(i, j) {
                    if val < 9 {
                        size += 1;
                        visited.insert((i,j));
                        q.extend([(i, j + 1), (i - 1, j), (i, j - 1), (i + 1, j)].iter())
                    }
                }
            }
        }
        // println!("size: {}", size);
        size
    }
}

impl FromStr for HeightMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arr: Vec<u32> = vec![];
        let mut nrows = 0;
        let mut ncols = 0;
        for line in s.lines() {
            ncols = 0;
            for d in line.chars().map(|s| s.to_digit(10).unwrap()) {
                arr.push(d);
                ncols += 1;
            }
            nrows += 1;
        }
        
        Ok(HeightMap {
            heights: arr,
            nrows,
            ncols,
        })
    }
}


