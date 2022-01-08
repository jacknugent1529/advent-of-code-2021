use std::fs;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    Ok(file_str.lines().map(|line| score_line_a(line)).sum())
}

fn score_line_a(line: &str) -> i32 {
    let mut delims: Vec<char> = vec![];
    let open_delims = ['(', '[', '{', '<'];
    let delim_cost = |c| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    };
    
    let get_closing = |c| match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        x => x
    };
    
    for c in line.chars() {
        if open_delims.contains(&c) {
            delims.push(c);
        } else { // closing
            let open = delims.pop().unwrap();
            if get_closing(open) != c {
                return delim_cost(c);
            }
        }
    }
    return 0;
}

pub fn soln_b(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut scores: Vec<u64> = file_str.lines().map(|line| score_line_b(line)).filter(|&x| x != 0).collect();
    scores.sort();
    // println!("scores: {:?}", scores);
    Ok(scores[scores.len() / 2])
}

fn score_line_b(line: &str) -> u64 {
    let mut delims: Vec<char> = vec![];
    let open_delims = ['(', '[', '{', '<'];
    let delim_cost = |c| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    };
    
    let get_closing = |c| match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        x => x
    };
    
    for c in line.chars() {
        if open_delims.contains(&c) {
            delims.push(c);
        } else { // closing
            let open = delims.pop().unwrap();
            if get_closing(open) != c {
                return 0;
            }
        }
    }
    return delims.iter().rev().fold(0, |tot, &c| tot * 5 + delim_cost(get_closing(c)))
}



