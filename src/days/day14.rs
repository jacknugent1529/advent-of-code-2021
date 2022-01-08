use std::fs;
use std::collections::HashMap;
use std::str::Lines;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut lines = file_str.lines();
    let mut seq = lines.next().unwrap().to_string();
    lines.next();
    let rule_map = parse_rule_map(&mut lines);

    for _ in 0..10 {
        let mut offset = 0;
        for i in 0..(seq.len() - 1) {
            let i_o = i + offset;
            if let Some(&c) = rule_map.get(&seq[i_o..=(i_o+1)]) {
                seq.insert(i_o+1,c);
                offset += 1;
            }
        }
        // println!("seq: {}", seq);
    }
    
    Ok(diff_max_min_char_occurrences(&seq))
}

fn parse_rule_map(lines: &mut Lines) -> HashMap<String, char> {
    let mut rules_map = HashMap::new();
    for l in lines.into_iter() {
        let mut split = l.split(" -> ").map(|s| s.to_string());
        rules_map.insert(split.next().unwrap(), split.next().unwrap().chars().next().unwrap());
    }
    rules_map
}

fn diff_max_min_char_occurrences(s: &str) -> i32 {
    let mut freqs = HashMap::new();
    for c in s.chars() {
        let freq = freqs.entry(c).or_insert(0);
        *freq += 1;
    }
    
    let vals: Vec<i32> = freqs.iter().map(|(_, &v)| v).collect();
    return vals.iter().max().unwrap() - vals.iter().min().unwrap();
}

pub fn soln_b(file: &str) -> Result<u64, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let mut lines = file_str.lines();
    let seq = lines.next().unwrap();
    lines.next();
    let rule_map = parse_rule_map_b(&mut lines);
    
    let mut pairs_freq: HashMap<(char, char), u64> = HashMap::new();
    for (a,b) in seq.chars().zip(seq.chars().skip(1)) {
        let freq = pairs_freq.entry((a,b)).or_insert(0);
        *freq += 1
    }
    
    for _ in 0..40 {
        let mut new_pairs_freq: HashMap<(char, char), u64> = HashMap::new();
        for ((a,b), v) in pairs_freq {
            if let Some(&c) = rule_map.get(&(a,b)) {
                let freq_ac = new_pairs_freq.entry((a,c)).or_insert(0);
                *freq_ac += v;

                let freq_cb = new_pairs_freq.entry((c,b)).or_insert(0);
                *freq_cb += v;
            }
        }
        pairs_freq = new_pairs_freq;
    }

    Ok(diff_max_min_char_occurrences_pairs(&pairs_freq, seq.chars().nth(0).unwrap(), seq.chars().last().unwrap()))
}

fn parse_rule_map_b(lines: &mut Lines) -> HashMap<(char, char), char> {
    let mut rules_map = HashMap::new();
    for l in lines.into_iter() {
        let mut split = l.split(" -> ").map(|s| s.to_string());
        let chars: Vec<char> = split.next().unwrap().chars().collect();
        rules_map.insert((chars[0], chars[1]), split.next().unwrap().chars().next().unwrap());
    }
    rules_map
}

fn diff_max_min_char_occurrences_pairs(pairs_freq: &HashMap<(char, char), u64>, first: char, last: char) -> u64 {
    let mut freqs = HashMap::new();
    for ((a,b),count) in pairs_freq {
        let freq_a = freqs.entry(a).or_insert(0);
        *freq_a += count;

        let freq_b = freqs.entry(b).or_insert(0);
        *freq_b += count;
    }

    let vals: Vec<u64> = freqs.iter().map(|(_, &v)| v).collect();
    // double count because of pairs
    return ( vals.iter().max().unwrap() - vals.iter().min().unwrap() )/2 + 1;
}