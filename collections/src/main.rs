use std::collections::HashMap;

use collections::{median, mode, word_to_pig_latin};

fn main() {
    let v: Vec<i32> = vec![1, 1, 6, 4, 5, 3];
    let median = median(&v);
    let mode = mode(&v);

    println!("Median: {median:?}");
    println!("Mode: {mode:?}");

    let w1 = String::from("Здравствуйте");
    let w2 = String::from("Sствуйте");
    let w3 = String::from("Scoops");
    let w4 = String::from("");

    word_to_pig_latin(&w1);
    word_to_pig_latin(&w2);
    word_to_pig_latin(&w3);
    word_to_pig_latin(&w4);

    // Using a hash map and vectors, create a text interface to allow 
    // a user to add employee names to a department in a company; for example, 
    // “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve 
    // a list of all people in a department or all people in the 
    // company by department, sorted alphabetically.

    let v1 = [
        String::from("Add Sally to Engineering"),
        String::from("Add Scoop to Finance"),
        String::from("Add Ahoy to Finoonce"),
        String::from("Add Amir to Sales."),
        String::from("Add Amir1 to Sales"),
        String::from("Add Amir2 to Sales"),
        String::from("Add Amir3 to Sales"),
        String::from("Add Amir4 to Sales"),
        String::from("Add Amir5 to Sales"),
        String::from("Add Amir6 to Sales"),
        String::from("Add Amir7 to Sales")

    ];

    let mut hm: HashMap<String, String> = HashMap::new();
    for add in v1 {
        match collections::add_user(&mut hm, &add) {
            Some(s) => println!("{s}"),
            None => println!("{} is invalid", add)
        }
    }

    println!("{:?}", &hm);
    let lookup_value = "Sales";

    
    let ubd = collections::get_users_by_department(&hm, lookup_value);
    if ubd.len() == 0 {
        println!("No one in department {lookup_value}")
    } else {
        println!("================ {lookup_value} ===============");
        for user in ubd {
            println!("{user}")
        }
    }

}





