use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let displays: Vec<Display> = file_str.lines().map(|s| s.parse().unwrap()).collect();
    // length -> frequency
    let mut frequencies: HashMap<usize, usize> = HashMap::new(); 
    for d in displays {
        for s in d.outputs {
            let freq = frequencies.entry(s.len()).or_insert(0);
            *freq += 1;

            // let stat = player_stats.entry("attack").or_insert(100);
            // *stat += random_stat_buff();
        }
    }
    let count: usize = frequencies.iter().filter(|(&key, _)| key == 2 || key == 3 || key == 4 || key == 7).map(|(_, val)| val).sum();
    return Ok(count as i32);
}
    

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let displays: Vec<Display> = file_str.lines().map(|s| s.parse().unwrap()).collect();

    Ok(displays.iter().map(|d| calc_display(d)).sum())
}

fn calc_display(display: &Display) -> i32 {
    let mut inputs_iter = display.inputs.iter();
    // first create a set of the segments in numbers 1, 4, and 7, which we identify by the number of segments
    let mut digits_mapping: HashMap<i32, HashSet<char>> = HashMap::new();
    
    while digits_mapping.len() < 3 {
        let d = inputs_iter.next().unwrap();
        match d.len() {
            2 => digits_mapping.insert(1,d.chars().collect::<HashSet<char>>()), // convert to hashset
            4 => digits_mapping.insert(4,d.chars().collect::<HashSet<char>>()),
            3 => digits_mapping.insert(7,d.chars().collect::<HashSet<char>>()),
            _ => None
        };
    }
    
    let mut num = 0;
    let mut power = 1;
    for d in display.outputs.iter().rev() {
        let digit = classify_digit(d, &digits_mapping);
        num += power * digit;
        power *= 10;
    }
    
    num
}

pub fn classify_digit(digit: &str, digits_mapping: &HashMap<i32, HashSet<char>>) -> i32 {
    let d = match digit.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        5 => classify_digits_5_lines(digit, digits_mapping),
        6 => classify_digits_6_lines(digit, digits_mapping),
        _ => 8 // will always be 7
    };
    return d;
}

pub fn classify_digits_5_lines(digit: &str, digits_mapping: &HashMap<i32, HashSet<char>>) -> i32 {
    let chars_set: HashSet<char> = digit.chars().collect();
    let set_1 = digits_mapping.get(&1).unwrap();
    let set_4 = digits_mapping.get(&4).unwrap();
    let set_7 = digits_mapping.get(&7).unwrap();
    
    if chars_set.intersection(set_4).count() == 3 && chars_set.intersection(set_7).count() == 2 {
        return 5;
    }
    
    let set_1_i_digit: HashSet<char> = (*set_1).intersection(&chars_set).map(|&c| c).collect();
    if set_1_i_digit == *set_1 {
        return 3;
    }

    return 2;
}

pub fn classify_digits_6_lines(digit: &str, digits_mapping: &HashMap<i32, HashSet<char>>) -> i32 {
    let chars_set: HashSet<char> = digit.chars().collect();
    let set_1 = digits_mapping.get(&1).unwrap();
    let set_4 = digits_mapping.get(&4).unwrap();
    
    
    if (*set_4).intersection(&chars_set).map(|&c| c).count() == 4 {
        return 9;
    }
    
    let set_1_i_digit: HashSet<char> = (*set_1).intersection(&chars_set).map(|&c| c).collect();
    if set_1_i_digit == *set_1 {
        return 0;
    }

    return 6;
}


#[derive(Debug)]
struct Display {
    inputs: Vec<String>,
    outputs: Vec<String>
}

impl FromStr for Display {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" | ");
        let inputs: Vec<String> = split.next().unwrap().split(" ").map(|s| s.to_string()).collect();
        let outputs: Vec<String> = split.next().unwrap().split(" ").map(|s| s.to_string()).collect();
        
        Ok(Display {
            inputs,
            outputs
        })
    }
}