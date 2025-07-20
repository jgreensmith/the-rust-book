use std::{collections::HashMap};
use std::io;

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


pub fn add_user(hm: &mut HashMap<String, String>, a: &String) -> Option<String> {

    let v: Vec<&str> = a.split_whitespace().collect();

    let departments = ["Engineering", "Sales", "Finance"];


    if departments.contains(&v[3]) {
        hm.insert(v[1].to_string(), v[3].to_string());
    } else {
        return None
    }
    Some(format!("{} added to {}", v[1], v[3]))
}

pub fn get_users_by_department(hm: &HashMap<String, String>, l: &str) -> Vec<String> {
    let mut vec = Vec::new();
    for (k, v) in hm {
        if v == l {
            vec.push(k.to_string());
        }
    }
    vec
}

// Using a hash map and vectors, create a text interface to allow a
// user to add employee names to a department in a company. For
// example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.


pub fn start() {
    println!("Add employee names to each deparment!");

    // We are using HashMap<String, Vec<String>> instead of
    // HashMap<String, &mut Vec<String>> because if the hashmap owns
    // the values, you can always get mutable references from it
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("what to do?");
        println!("a) add a dept");
        println!("b) add employee to an existing dept");
        println!("c) view all people in a department");
        println!("d) view all people in the company by dept");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: char = match input.trim().parse() {
            Ok(c) => c,
            // The underscore, _, is a catchall value; in this
            // example, we’re saying we want to match all Err values,
            // no matter what information they have inside them.
            Err(_) => continue,
        };

        match input {
            // create a new department with an empty vector on the map:
            'a' => add_dept(&mut map),
            'b' => add_employee_to_dept(&mut map),
            'c' => print_employees_for_dept(&map),
            'd' => println!("the company directory: {:?}", map),
            _ => panic!("invalid input, goodbye!"),
        }
    }
}

// create a new department with an empty vector on the map:
fn add_dept(map: &mut HashMap<String, Vec<String>>) {
    println!("enter the name of the dept");
    let mut dept = String::new();
    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");
    let dept: String = match dept.trim().parse() {
        Ok(s) => s,
        Err(_) => panic!("invalid dept input!"),
    };
    map.entry(dept).or_insert(Vec::new());
    println!("company depts: {:?}", map);
    println!();
}

// asks for the departments name, and the employee's name, and inserts
// the employee's name into the deptarment
fn add_employee_to_dept(map: &mut HashMap<String, Vec<String>>) {
    println!("enter the name of the dept");
    let mut dept = String::new();
    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");
    let dept: String = match dept.trim().parse() {
        Ok(s) => s,
        Err(_) => panic!("invalid dept input"),
    };
    let employees: Option<&mut Vec<String>> = map.get_mut(&dept);

    let mut employee = String::new();
    println!("enter the name of the employee");
    io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read line");

    let employee: String = match employee.trim().parse() {
        Ok(s) => s,
        Err(_) => return,
    };
    if let Some(employees_vector) = employees {
        employees_vector.push(employee.to_string());
    } else {
        panic!("invalid input");
    }
    println!("company depts: {:?}", map);
    println!();
}

fn print_employees_for_dept(map: &HashMap<String, Vec<String>>) {
    println!("enter the name of the dept");
    let mut dept = String::new();
    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");
    let dept: String = match dept.trim().parse() {
        Ok(s) => s,
        Err(_) => panic!("invalid dept input"),
    };
    let employees = map.get(&dept);
    println!("employees for dept: {}, are: {:?}", dept, employees);
}