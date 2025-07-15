use std::{collections::HashMap};

pub fn median (vec: &Vec<i32>) -> f64 {
    let mut v2: Vec<i32> = vec.iter().copied().collect();
    v2.sort();
    let l = v2.len() - 1;
    if l % 2 == 1 {
        let fi: f64 = l as f64 / 2.0;
        let higher = v2[fi.ceil() as usize] as f64;
        let lower = v2[fi.floor() as usize] as f64;
        let median = (higher + lower) / 2.0;
        return median
    } else {
        let i = l / 2;
        return v2[i] as f64;
    }
}

pub fn mode (list: &Vec<i32>) -> Vec<i32> {
    let mut h = HashMap::new();
    for i in list {
        let val_count = h.entry(i).or_insert(0);
        *val_count += 1;
    };
    let top_count = h.values().max().unwrap();
    let mut vec = Vec::new();
    for (k, v) in &h {
        if v == top_count {
            vec.push(**k);
        };
    };
    vec
}

pub fn word_to_pig_latin(s: &str) -> Option<String>{
    if s.is_empty() {
        return Some(String::new());
    }
    for b in s.bytes() {
        if !b.is_ascii_alphabetic() {
            return None;
        }
    }
    let vowels = b"aeiou";
    let b1 = s.as_bytes()[0];

    if vowels.contains(&b1.to_ascii_lowercase()) {
        return Some(format!("{s}-hay"));
    }
    let (c1, r) = s.split_at(1);
    Some(format!("{}-{}ay", r, c1.to_lowercase()))
}
