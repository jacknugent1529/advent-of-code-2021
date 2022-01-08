use std::fs;
use std::hash::Hash;
use std::iter::Scan;
use std::ops::Add;
use std::ops::Sub;
use std::collections::HashSet;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut scanners = parse_scanners(&file_str);
    let (_, beacons) = calc_coords(&mut scanners);

    Ok(beacons.len() as i32)
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut scanners = parse_scanners(&file_str);
    let (scanner_locs, _) = calc_coords(&mut scanners);
    
    let mut max_dist = 0;
    let num_scanners = scanner_locs.len();
    for i in 0..num_scanners {
        for j in 0..num_scanners {
            if i == j { continue; }
            let dist = scanner_locs[i].manhattan_dist(&scanner_locs[j]);
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    Ok(max_dist)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
    fn abs(&self) -> Point {
        let Point{x,y,z } = self;
        Point {
            x: x.abs(),
            y: y.abs(),
            z: z.abs(),
        }
    }
    fn manhattan_dist(&self, other: &Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs();
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }   
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }   
}

fn calc_coords(scanners: &mut Vec<Scanner>) -> (Vec<Point>, HashSet<Point>) {
    let mut solved_scanner = scanners.remove(0);
    solved_scanner.location = Some(Point::new(0,0,0));
    let mut i = 0;
    let mut scanner_locs = vec![Point::new(0,0,0)];
    loop {
        let mut remove_i = false;
        let scanner = &mut scanners[i];
        if let Some((scanner_loc, common_points )) = solved_scanner.overlap(scanner) {
            scanner_locs.push(scanner_loc);
            solved_scanner.beacons.extend(common_points);
            println!("{} remaining", scanners.len());
            remove_i = true;
        }
        if remove_i {
            scanners.remove(i);
            i = 0;
        } else {
            i += 1;
        }
        if scanners.len() == 0 {
            break;
        }
    }
    return (scanner_locs, solved_scanner.beacons);
}


#[derive(Debug, Clone)]
struct Scanner {
    location: Option<Point>,
    // if location is none, then beacons are relative to the scanner. If location is specified, then beacon
    // locations are absolute
    beacons: HashSet<Point>,
}

impl Scanner {
    fn new(beacons: HashSet<Point>) -> Scanner {
        return Scanner {
            location: None,
            beacons
        }
    }
    
    fn overlap(&self, other: &Scanner) -> Option<(Point, HashSet<Point>)> {
        let mut max_common = 0;
        for b1 in self.beacons.iter() {
            for b2 in other.beacons.iter() {
                for t_x in [-1,1].iter() {
                    for t_y in [-1,1].iter() {
                        for t_z in [-1,1].iter() {
                            let permutations: [(usize, usize, usize); 6]  = [
                                (0,1,2),
                                (0,2,1),
                                (1,0,2),
                                (1,2,0),
                                (2,0,1),
                                (2,1,0),
                            ];
                            for perm in permutations.iter() {
                                let rel_b1 = &self.beacons;
                                let (scanner, rel_b2) = other.rel_coords_from_trans(*b2, *b1, *t_x, *t_y, *t_z, *perm);
                                let common_points: Vec<Point> = rel_b1.intersection(&rel_b2).map(|p| *p).collect();
                                let num_common_points = common_points.len();
                                if num_common_points > max_common {
                                    max_common = num_common_points;
                                }
                                if num_common_points >= 12 {
                                    return Some((scanner, rel_b2.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
    
    fn rel_coords_from_trans(&self, p1: Point, p2: Point, t_x: i32, t_y: i32, t_z: i32, perm: (usize,usize,usize)) -> (Point, HashSet<Point>) {
        let trans = |&Point{x,y,z}| {
            let vec = [(x - p1.x)*t_x, (y - p1.y)*t_y, (z - p1.z)*t_z];
            let x = vec[perm.0] + p2.x;
            let y = vec[perm.1] + p2.y;
            let z = vec[perm.2] + p2.z;
            Point{
                x, 
                y, 
                z, 
            }
        };
        let set: HashSet<Point> = self.beacons.iter().map(trans).collect();
        let origin = trans(&Point{x:0,y:0,z:0});
        (origin, set)
    }
}

fn parse_scanners(file_str: &str) -> Vec<Scanner> {
    let lines = file_str.lines();
    let mut beacons = HashSet::new();
    let mut scanners = vec![];
    
    for line in lines {
        if line.starts_with("--- scanner") {
            if beacons.len() > 0 {
                scanners.push(Scanner::new(beacons.clone()));
                beacons = HashSet::new();
            }
        } else {
            let tuple_vec: Vec<i32> = line.split(',').filter(|&s| !s.is_empty()).map(|s| s.parse().expect("should be a number")).collect();
            if tuple_vec.len() > 0 {
                beacons.insert(Point::new(tuple_vec[0], tuple_vec[1], tuple_vec[2]));
            }
        }
    }
    if beacons.len() > 0 {
        scanners.push(Scanner::new(beacons.clone()));
    }
    scanners
}






