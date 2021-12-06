use std::fs;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let n = file_str.lines().by_ref().next().unwrap().len();
    let mut sums = vec![0; n];
    let mut n_lines: u32 = 0;
    for line in file_str.lines() {
        for (i, c) in line.chars().enumerate() {
            sums[i] += c.to_digit(10).unwrap();
        }
        n_lines += 1;
    }
    

    let common_digit: Vec<bool> = sums.into_iter().map(|s| s > n_lines / 2).collect();

    let mut gamma = 0;
    let mut epsilon = 0;
    let base: u32 = 2;
    for (i, digit) in common_digit.into_iter().rev().enumerate() {
        if digit {
            gamma += base.pow(i as u32); 
        } else {
            epsilon += base.pow(i as u32);
        }
    }
    
    Ok((gamma * epsilon) as i32)
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let n = file_str.lines().by_ref().next().unwrap().len();
    let nums: Vec<Vec<u32>> = file_str.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).into_iter().collect()).collect();
    
    let o2 = bit_criteria_r(nums.clone(), 0, true);
    let co2 = bit_criteria_r(nums, 0, false);
    // println!("{:?}", remaining_o2);
    Ok((binary_to_dec(o2) * binary_to_dec(co2)) as i32 )
}

fn bit_criteria_r(nums: Vec<Vec<u32>>, i: usize, above: bool) -> Vec<u32> {
    // println!("nums: {:?}", nums);
    let num_len = nums.len();
    let count: u32 = nums.clone().into_iter().map(|num| num[i]).sum();
    let most_common_bit = if 2 * count >= num_len as u32 { 1 } else { 0 };
    // println!("most common: {:?}", most_common_bit);
    let bit_criteria_val;
    if above {
        bit_criteria_val = most_common_bit;
    } else {
        bit_criteria_val = if most_common_bit == 1 { 0 } else { 1 };
    }
    
    let new_nums: Vec<Vec<u32>> = nums.into_iter().filter(|num| num[i] == bit_criteria_val).collect();
    if new_nums.len() > 1 {
        return bit_criteria_r(new_nums, i+1, above)
    } 
    return new_nums[0].clone()       
}

fn binary_to_dec(num: Vec<u32>) -> u32 {
    let mut sum = 0; 
    let base: u32 = 2;
    for (i, digit) in num.into_iter().rev().enumerate() {
        sum += digit * base.pow(i as u32); 
    }
    sum
}