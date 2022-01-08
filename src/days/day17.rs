use std::fs;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut nums_res = file_str.lines().next().expect("no first line")[15..].split(", y=").map(|p| p.split("..").map(|s| s.parse::<i32>().expect("integer") ));
    let xs: Vec<i32> = nums_res.next().expect("target area incorrect format").collect();
    let ys: Vec<i32> = nums_res.next().expect("target area incorrect format").collect();
    let target = Rect {
        x_min: xs[0],
        x_max: xs[1],
        y_min: ys[0],
        y_max: ys[1]
    };
    let mut highest_y = 0;
    for vy in 0..1000 {
        for vx in 0..100 {
            if trajectory_successful((0,0), (vx, vy), &target) {
                let max_y = (vy * (vy + 1)) / 2;
                if max_y > highest_y { highest_y = max_y };
            }
        }
    }
    Ok(highest_y)
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut nums_res = file_str.lines().next().expect("no first line")[15..].split(", y=").map(|p| p.split("..").map(|s| s.parse::<i32>().expect("integer") ));
    let xs: Vec<i32> = nums_res.next().expect("target area incorrect format").collect();
    let ys: Vec<i32> = nums_res.next().expect("target area incorrect format").collect();
    let target = Rect {
        x_min: xs[0],
        x_max: xs[1],
        y_min: ys[0],
        y_max: ys[1]
    };
    let mut count = 0;
    for vy in -1000..1000 {
        for vx in 0..100 {
            if trajectory_successful((0,0), (vx, vy), &target) { count += 1; }
        }
    }
    Ok(count)
}

fn trajectory_successful(p: (i32, i32), v: (i32, i32), target: &Rect) -> bool {
    let (mut x,mut y) = p;
    let (vx, vy) = v;
    x += vx;
    y += vy;
    
    if target.inside((x,y)) { return true; } 
    if target.right((x,y)) { return false; }
    if target.below((x,y)) { return false; }
    let vx_new = if vx > 0 { vx - 1 } else { 0 }; 
    let vy_new = vy - 1;
    return trajectory_successful((x,y), (vx_new, vy_new), target);
}


struct Rect {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32
}

impl Rect {
    fn inside(&self, p: (i32, i32)) -> bool {
        return self.x_min <= p.0 
            && self.x_max >= p.0
            && self.y_min <= p.1
            && self.y_max >= p.1
    }
    
    fn right(&self, p: (i32, i32)) -> bool {
        return p.0 > self.x_max;
    }

    fn below(&self, p: (i32, i32)) -> bool {
        return p.1 < self.y_min;
    }
}