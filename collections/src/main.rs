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

    let tup = collections::add_user(String::from("Add Fart to Engineering"));
    println!("{tup:?}")
}



