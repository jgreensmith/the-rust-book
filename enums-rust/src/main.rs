fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) if i >= 10 => Some(i),
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let ten = plus_one(Some(10));

    assert_eq!(six, Some(6));
    assert_eq!(none, None);
    assert_eq!(ten, Some(10));

    println!("{}", six.is_some_and(|x| x > 2))
}
