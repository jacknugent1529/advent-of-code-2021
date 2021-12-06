use std::io;
use std::fs;
mod utils;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
}

// change day{n} to the desired day in the followign two lines
const DAY: &str = "day6";
use days::day6 as day;


fn main() -> io::Result<()> {
    let test_input = format!("data/{}/{}.test.in", DAY, DAY);
    let test_answer_a = format!("data/{}/{}.a.test.ans", DAY, DAY);
    let test_answer_b = format!("data/{}/{}.b.test.ans", DAY, DAY);
    let input = format!("data/{}/{}.in", DAY, DAY);

        
    let test_result_a = match day::soln_a(&test_input) {
        Err(e) => { println!("ERROR: {}", e.to_string()); return Ok(()) },
        Ok(n) => n
    };
    let test_expected_a = fs::read_to_string(test_answer_a)?.trim().parse().unwrap();
    assert_eq!(test_result_a, test_expected_a);

    match day::soln_a(&input) {
        Err(e) => println!("ERROR: {}", e.to_string()),
        Ok(n) => println!("Solution part A: {}", n)
    };

    let test_result_b = match day::soln_b(&test_input) {
        Err(e) => { println!("ERROR: {}", e.to_string()); return Ok(()) },
        Ok(n) => n
    };
    let test_expected_b = fs::read_to_string(test_answer_b)?.trim().parse().unwrap();
    assert_eq!(test_result_b, test_expected_b);

    match day::soln_b(&input) {
        Err(e) => println!("ERROR: {}", e.to_string()),
        Ok(n) => println!("Solution part B: {}", n)
    };
    
    Ok(())
}