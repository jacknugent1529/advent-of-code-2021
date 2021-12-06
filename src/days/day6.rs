use std::fs;

pub fn soln_a(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums: Vec<usize> = file_str.lines().next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    let mut fish: [u64; 9] = [0; 9];
    for n in nums {
        fish[n] += 1;
    }
    
    for _ in 0..80 {
        sim_step(&mut fish);
    }
    
    Ok(fish.iter().sum())
}

pub fn soln_b(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums: Vec<usize> = file_str.lines().next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    let mut fish: [u64; 9] = [0; 9];
    for n in nums {
        fish[n] += 1;
    }
    
    for _ in 0..256 {
        sim_step(&mut fish);
    }
    
    Ok(fish.iter().sum())
}

fn sim_step(fish: &mut [u64]) {
    let num_zero = fish[0];
    for i in 1..=8 {
        fish[i-1] = fish[i];
    }
    fish[6] += num_zero;
    fish[8] = num_zero;
}