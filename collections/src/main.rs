use std::collections::HashMap;


fn median (vec: &Vec<i32>) -> i32 {
    // let mut median: Vec<_> = Vec::new();

    // I think this is better than clone ()
    let mut v2: Vec<i32> = vec.iter().copied().collect();
    v2.sort();
    let l = v2.len() - 1;

    // check if even number of items in vec
    if l % 2 == 1 {
        let fi: f64 = l as f64 / 2.0;
        let higher = v2[fi.ceil() as usize] as f64;
        let lower = v2[fi.floor() as usize] as f64;
        // return mean of two medians
        let median = ((higher + lower) / 2.0).ceil() as i32;
        return median
    } else {
        let i = l / 2;
        return v2[i];

    }
}

fn mode (list: &Vec<i32>) -> Vec<i32> {
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


fn main() {

    let v: Vec<i32> = vec![1, 1, 6, 4, 5, 3];
    let median = median(&v);
    let mode = mode(&v);

    println!("Median: {median:?}");
    println!("Mode: {mode:?}");

}
   


    // let v = vec![100, 32, 57];
    // for &i in &v {
    //     let scoop: u8 = &i - 5;
    //     println!("{}", scoop);
    // }
    // for c in "scoops".chars() {
    //     println!("{c}");
    // }
    // for c in "scoops".bytes() {
    //     println!("{c}");
    // }
    // let fart = "fart";

    // println!("{}", &fart[0]);


    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     count += 1;
    // }

    // println!("{map:?}");
