fn longest_good<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_bad(x: &str, y: &str) -> &'static str {
    if x.len() > y.len() { "fart" } else { "nips" }
}

fn longest_meh<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{y}");
    x
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string3;
    {
        let string2 = String::from("xyz");
        string3 = string2
    }
    result = longest_good(string1.as_str(), string3.as_str());
    println!("The longest string is {result}");

    let scoops = "scoops";
    let ahoy = "ahoy";
    println!("{}", longest_bad(scoops, ahoy));
    let result2;

    let scoops1 = "scoops1";
    {
        let ahoy2: &'static str = "ahoy2";
        result2 = longest_meh(scoops1, ahoy2);
    }

    println!("{}", result2);

}
