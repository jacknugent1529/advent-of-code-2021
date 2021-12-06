use std::fs;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums_res: Result<Vec<i32>, std::num::ParseIntError> = file_str.lines().map(|s| s.parse()).collect();
    
    Err("ERROR: Not implemented".to_string())
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let nums_res: Result<Vec<i32>, std::num::ParseIntError> = file_str.lines().map(|s| s.parse()).collect();
    
    Err("ERROR: Not implemented".to_string())
}