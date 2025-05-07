#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    println!("Hello, world!");
    median_and_mode();

    convert_to_pig_latin("first hello world apple FiRst HELLO WOrld APPLe");

    add_and_retrieve_employees();

    // let nums = vec![3, 2, 4];
    //
    // let mut v: Vec<(i32, usize)> = nums
    //     .clone()
    //     .iter()
    //     .enumerate()
    //     .map(|(i, &item)| (item, i))
    //     .collect();
    //
    // v.sort();
}

fn add_and_retrieve_employees() {
    // assumes single word names and single word departments
    // assumes input as "Add <name> to <dept>"
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        let line_vector: Vec<&str> = line.split_whitespace().collect();
        let name = line_vector[1].to_string();
        let department = line_vector[3].to_string();

        let value_of_entry = map.entry(department).or_insert(vec![]);
        value_of_entry.push(name);
    }

    // retrieve list of all people in a deaprtment or all people in the company by department
    // sorted alphabetilcally

    let mut keys: Vec<String> = map.keys().into_iter().map(|s| s.clone()).collect();
    keys.sort();

    for dept in &keys {
        println!("Department = {dept}");
        let mut names = map.get(dept).unwrap().clone();
        names.sort();

        for name in &names {
            println!("Name = {name}");
        }
        println!("");
    }
}

fn check_if_vowel(w: &str) -> bool {
    let ans: bool;
    match w {
        "a" | "e" | "i" | "o" | "u" => ans = true,
        "A" | "E" | "I" | "O" | "U" => ans = true,
        _ => ans = false,
    }

    ans
}

fn convert_to_pig_latin(s: &str) {
    let mut ans = String::new();

    for word in s.split_whitespace() {
        if word.is_ascii() {
            let first_ch = &word[0..1];
            if check_if_vowel(first_ch) {
                ans.push_str(&format!("{word}-hay"));
            } else {
                let add_string = &format!("{}-ay{}", &word[1..], first_ch);
                ans.push_str(&add_string);
            }
        } else {
            ans.push_str(word);
        }
        ans.push(' ');
    }

    println!("{ans}");
}

fn median_and_mode() {
    let v_actual = vec![5, 3, 3, 3, 2, 4, 1, 5, 6, 2, 3];
    let v_actual = dbg!(&v_actual);

    let mut v = v_actual.clone();
    v.sort();

    let mut v = dbg!(&v);

    let n = v.len();
    let mid_index = (n as f64 / 2.0).floor() as usize;
    println!("mid_index = {mid_index}");
    println!("Median = {}", v[mid_index]);

    let mut counter = HashMap::new();

    for item in v {
        let value = counter.entry(item).or_insert(0);
        *value += 1;
    }

    let mut max_value = i32::MIN;
    let mut mode: Option<i32> = None;

    for (key, value) in &counter {
        if *value > max_value {
            max_value = *value;
            mode = Some(**key);
        }
    }

    let mode = mode.unwrap();
    println!("Mode = {mode}");
}
