use std::fs;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums: Vec<i32> = file_str.lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let fuel_spent = |arr: &Vec<i32>, i: i32| arr.iter().map(|n| (n - i).abs()).sum();
    
    let max: i32 = *nums.iter().max().unwrap();
    let min: i32 = *nums.iter().min().unwrap();
    Ok((min..max).map(|i| fuel_spent(&nums, i)).min().unwrap())
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums: Vec<i32> = file_str.lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let fuel_move = |a: i32, b: i32| { let d = (a-b).abs(); return (d*(d+1))/2 };
    let fuel_spent = |arr: &Vec<i32>, i: i32| arr.iter().map(|n| fuel_move(*n,i)).sum();
    
    let max: i32 = *nums.iter().max().unwrap();
    let min: i32 = *nums.iter().min().unwrap();
    Ok((min..max).map(|i| fuel_spent(&nums, i)).min().unwrap())
}