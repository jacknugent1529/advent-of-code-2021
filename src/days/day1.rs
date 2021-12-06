use std::fs;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums_res: Result<Vec<i32>, std::num::ParseIntError> = file_str.lines().map(|s| s.parse()).collect();
    
    let nums = match nums_res {
        Err(e) => return Err(e.to_string()),
        Ok(n) => n,
    };

    return Ok(nums.iter().zip(nums[1..].iter()).fold(0, |acc, (d1, d2)| if d2 > d1 { acc + 1 } else { acc }))
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums_res: Result<Vec<i32>, std::num::ParseIntError> = file_str.lines().map(|s| s.parse()).collect();
    
    let nums = match nums_res {
        Err(e) => return Err(e.to_string()),
        Ok(n) => n,
    };
    let rolling_sum: Vec<i32> = nums.windows(3).map(|w| w.into_iter().sum()).collect();
    
    return Ok(rolling_sum.iter().zip(rolling_sum[1..].iter()).fold(0, |acc, (d1, d2)| if d2 > d1 { acc + 1 } else { acc }))
}