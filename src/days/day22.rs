use std::fs;
use std::str::FromStr;
use std::cmp::{min, max};

pub fn soln_a(file: &str) -> Result<i64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let instructions_iter  = file_str.lines().filter(|s| s.len() > 1).map(|l| l.parse().unwrap());
    let instructions: Vec<Instruction> = instructions_iter.filter(|i: &Instruction| i.in_small_range()).collect();
    let on_areas = Instruction::execute_instructions(instructions);
    let volume = on_areas.iter().map(|area| area.volume()).sum();
    Ok(volume)
}

pub fn soln_b(file: &str) -> Result<i64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let instructions: Vec<Instruction> = file_str.lines().filter(|s| s.len() > 1).map(|l| l.parse().unwrap()).collect();
    let on_areas = Instruction::execute_instructions(instructions);
    let volume = on_areas.iter().map(|area| area.volume()).sum();
    Ok(volume)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    on: bool,
    min_p: (i64, i64, i64),
    max_p: (i64, i64, i64),
    subtracted: Vec<Instruction>
}

impl Instruction {
    fn execute_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut on_areas: Vec<Instruction> = vec![];
        for instr in instructions.iter() {
            for area in on_areas.iter_mut() {
                area.subtract(instr);
            }
            if instr.on {
                on_areas.push(instr.clone());
            }
        }
        on_areas
    }
    fn new(min_p: (i64, i64, i64), max_p: (i64, i64, i64), on: bool) -> Option<Instruction> {
        if min_p.0 > max_p.0 || min_p.1 > max_p.1 || min_p.2 > max_p.2 {
            return None
        }
        return Some(Instruction { min_p, max_p, subtracted: vec![], on });
    }
    fn intersect(&self, other: &Instruction, on: bool) -> Option<Instruction> {
        let (min_x_s, min_y_s, min_z_s) = self.min_p;
        let (max_x_s, max_y_s, max_z_s) = self.max_p;
        let (min_x_o, min_y_o, min_z_o) = other.min_p;
        let (max_x_o, max_y_o, max_z_o) = other.max_p;
        let min_p = (
            max(min_x_s, min_x_o),
            max(min_y_s, min_y_o),
            max(min_z_s, min_z_o)
        );
        let max_p = (
            min(max_x_s, max_x_o),
            min(max_y_s, max_y_o),
            min(max_z_s, max_z_o)
        );
        return Instruction::new(min_p, max_p, on)
    }
    fn subtract(&mut self, other: &Instruction) {
        if let Some(c) = self.intersect(other, other.on) {
            // subtract cube from each cube that is already subtracted
            for sub_c in self.subtracted.iter_mut() {
                sub_c.subtract(&c);
            }
            self.subtracted.push(c.clone());
        }
        
    }
    fn contains(&self, other: &Instruction) -> bool {
        return self.min_p.0 <= other.min_p.0 && 
               self.min_p.1 <= other.min_p.1 && 
               self.min_p.2 <= other.min_p.2 && 
               self.max_p.0 >= other.max_p.0 &&
               self.max_p.1 >= other.max_p.1 &&
               self.max_p.2 >= other.max_p.2;
    }
    fn volume(&self) -> i64 {
        let bound =  (self.max_p.0 - self.min_p.0 + 1) *
                         (self.max_p.1 - self.min_p.1 + 1) *
                         (self.max_p.2 - self.min_p.2 + 1);
        let subtracted: i64 = self.subtracted.iter().map(|area| area.volume()).sum();
        bound - subtracted
    }
    fn in_small_range(&self) -> bool {
        const MIN_BOUND: i64 = -50;
        const MAX_BOUND: i64 = 50;
        if self.min_p.0 < MIN_BOUND || self.max_p.0 > MAX_BOUND { return false; }
        if self.min_p.1 < MIN_BOUND || self.max_p.1 > MAX_BOUND { return false; }
        if self.min_p.2 < MIN_BOUND || self.max_p.2 > MAX_BOUND { return false; }
        return true;
    }
}



impl FromStr for Instruction {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let on = s.starts_with("on");
        let mut split = s[3..].split(',');
        let get_minmax = |s: &str| {
            let mut num_iter = s.split('=').skip(1).next().unwrap().split("..").map(|n| n.parse::<i64>().unwrap());
            (num_iter.next().unwrap(), num_iter.next().unwrap())
        };
        let (min_x, max_x) = get_minmax(split.next().unwrap());
        let (min_y, max_y) = get_minmax(split.next().unwrap());
        let (min_z, max_z) = get_minmax(split.next().unwrap());

        Ok(Instruction::new(
            (min_x, min_y, min_z),
            (max_x, max_y, max_z),
            on
        ).unwrap())
    }
}

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// struct Instruction {
//     on: bool,
//     min_x: i64,
//     max_x: i64,
//     min_y: i64,
//     max_y: i64,
//     min_z: i64,
//     max_z: i64,
// }

// impl Instruction {
//     fn in_small_range(&self) -> bool {
//         const MIN_BOUND: i64 = -50;
//         const MAX_BOUND: i64 = 50;
//         if self.min_x < MIN_BOUND || self.max_x > MAX_BOUND { return false; }
//         if self.min_y < MIN_BOUND || self.max_y > MAX_BOUND { return false; }
//         if self.min_z < MIN_BOUND || self.max_z > MAX_BOUND { return false; }
//         return true;
//     }
// }

// impl Instruction { 
//     fn transform(&mut self, a: i64, b: i64) {
//         self.min_x *= a;
//         self.min_x += b;

//         self.max_x *= a;
//         self.max_x += b;

//         self.min_y *= a;
//         self.min_y += b;

//         self.max_y *= a;
//         self.max_y += b;

//         self.min_z *= a;
//         self.min_z += b;

//         self.max_z *= a;
//         self.max_z += b;
//     }
//     fn calc_volume(&self, scale: i64) -> i64 {
//         let min_x = self.min_x/scale;
//         let max_x = self.max_x/scale;
//         let min_y = self.min_y/scale;
//         let max_y = self.max_y/scale;
//         let min_z = self.min_z/scale;
//         let max_z = self.max_z/scale;
//         println!("{}..{}, {}..{}, {}..{} are on", min_x, max_x, min_y, max_y, min_z, max_z);
//         return (max_x-min_x + 1) as i64 *
//                (max_y-min_y + 1) as i64 *
//                (max_z-min_z + 1) as i64;
//     }
//     fn apply_to_many(&self, others: &HashSet<Instruction>) -> HashSet<Instruction> {
//         if others.len() == 0 {
//             return HashSet::from([self.clone()]);
//         }
//         let sets = others.iter().map(|instr| {
//             let set = self.apply_to_one(instr);
//             // set.insert(instr.clone());
//             set
//         });
//         sets.reduce(|acc, s| acc.union(&s).map(|i| *i).collect::<HashSet<Instruction>>()).unwrap()
//     }
//     fn apply_to_one(&self, other: &Instruction) -> HashSet<Instruction> {
//         // apply self to other
//         if self.on && other.on {
//             return HashSet::from([self.clone()]);
//         }
//         if self == other {
//             if self.on {
//                 return HashSet::from([self.clone()])
//             }
//             return HashSet::new();
//         }
//         // cases
//         // contains other
//         if self.contains(other) {
//             if self.on {
//                 return HashSet::from([self.clone()])
//             }
//             return HashSet::new();
//         }
//         // other contains self
//         if other.contains(self) {
//             return HashSet::from([
//                 Instruction { // top
//                     min_z: other.max_z,
//                     ..self.clone()
//                 },
//                 Instruction { // bottom
//                     max_z: other.min_z,
//                     ..self.clone()
//                 },
//                 Instruction { // west
//                     min_z: other.min_z,
//                     max_z: other.max_z,
//                     max_x: other.min_x,
//                     ..self.clone()
//                 },
//                 Instruction { // east
//                     min_z: other.min_z,
//                     max_z: other.max_z,
//                     min_x: other.max_x,
//                     ..self.clone()
//                 },
//                 Instruction { // north
//                     min_z: other.min_z,
//                     max_z: other.max_z,
//                     min_x: other.min_x,
//                     max_x: other.max_x,
//                     min_y: other.max_y,
//                     ..self.clone()
//                 },
//                 Instruction { // south
//                     min_z: other.min_z,
//                     max_z: other.max_z,
//                     min_x: other.min_x,
//                     max_x: other.max_x,
//                     max_y: other.min_y,
//                     ..self.clone()
//                 },
//             ]);
//         }
//         if let Some(overlap) = self.overlap(other) {
//             println!("partial overlap between: {:?}, {:?}", self, other);
//             // overlapping
//             // create a set of the 4 boxes and compute overlaps with each one
//             // should find no intersection with 3 and a complete intersection with the last
//             let common_point = other.common_point_of_overlapping(&overlap);
//             println!("common point: {:?}", common_point);
//             return overlap.apply_to_many(&HashSet::from([
//                 Instruction {min_x: common_point.0, min_y: common_point.1, min_z: common_point.2, ..other.clone()},
//                 Instruction {min_x: common_point.0, min_y: common_point.1, max_z: common_point.2, ..other.clone()},
//                 Instruction {min_x: common_point.0, max_y: common_point.1, min_z: common_point.2, ..other.clone()},
//                 Instruction {min_x: common_point.0, max_y: common_point.1, max_z: common_point.2, ..other.clone()},
//                 Instruction {max_x: common_point.0, min_y: common_point.1, min_z: common_point.2, ..other.clone()},
//                 Instruction {max_x: common_point.0, min_y: common_point.1, max_z: common_point.2, ..other.clone()},
//                 Instruction {max_x: common_point.0, max_y: common_point.1, min_z: common_point.2, ..other.clone()},
//                 Instruction {max_x: common_point.0, max_y: common_point.1, max_z: common_point.2, ..other.clone()},
//             ]));
//         }
//         // no overlap
//         return HashSet::from([self.clone()]); 
//     }

//     fn contains(&self, other: &Instruction) -> bool {
//         // checks if other is inside self
//         return other.min_x >= self.min_x && other.max_x <= self.max_x
//             && other.min_y >= self.min_y && other.max_y <= self.max_y
//             && other.min_z >= self.min_z && other.max_z <= self.max_z
//     }

//     fn contains_point(&self, p: (i64, i64, i64)) -> bool {
//         return p.0 > self.min_x && p.0 < self.max_x
//             && p.1 > self.min_y && p.1 < self.max_y
//             && p.2 > self.min_z && p.2 < self.max_z
//     }

//     fn common_point_of_overlapping(&self, overlap: &Instruction) -> (i64, i64, i64) {
//         for p in overlap.get_corners().iter() {
//             println!("corner: {:?}", p);
//             if self.contains_point(*p) {
//                 return *p
//             }
//         }
//         unreachable!()
//     }

//     fn get_corners(&self) -> Vec<(i64,i64,i64)> {
//         return vec![
//             (self.min_x, self.min_y, self.min_z),
//             (self.min_x, self.min_y, self.max_z),
//             (self.min_x, self.max_y, self.min_z),
//             (self.min_x, self.max_y, self.max_z),
//             (self.max_x, self.min_y, self.min_z),
//             (self.max_x, self.min_y, self.max_z),
//             (self.max_x, self.max_y, self.min_z),
//             (self.max_x, self.max_y, self.max_z),
//         ]
//     }

//     fn overlap(&self, other: &Instruction) -> Option<Instruction> {
//         let min_x = max(self.min_x, other.min_x);
//         let max_x = min(self.max_x, other.max_x);
//         if max_x <= min_x {
//             return None;
//         }
//         let min_y = max(self.min_y, other.min_y);
//         let max_y = min(self.max_y, other.max_y);
//         if max_y <= min_y {
//             return None;
//         }
//         let min_z = max(self.min_z, other.min_z);
//         let max_z = min(self.max_z, other.max_z);
//         if max_z <= min_z {
//             return None;
//         }
//         return Some(Instruction {
//             on: other.on, // ?
//             min_x,
//             max_x,
//             min_y,
//             max_y,
//             min_z,
//             max_z
//         });
//     }
// }

// impl FromStr for Instruction {
//     type Err = String;
    
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let on = s.starts_with("on");
//         let mut split = s[3..].split(',');
//         let get_minmax = |s: &str| {
//             let mut num_iter = s.split('=').skip(1).next().unwrap().split("..").map(|n| n.parse::<i64>().unwrap());
//             (num_iter.next().unwrap(), num_iter.next().unwrap())
//         };
//         let (min_x, max_x) = get_minmax(split.next().unwrap());
//         let (min_y, max_y) = get_minmax(split.next().unwrap());
//         let (min_z, max_z) = get_minmax(split.next().unwrap());

//         Ok(Instruction {
//             on,
//             min_x,
//             max_x,
//             min_y,
//             max_y,
//             min_z,
//             max_z,
//         })
//     }
// }
