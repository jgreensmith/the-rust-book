use collections;

#[test]
fn word_to_pig_latin_empty() {
    assert_eq!(collections::word_to_pig_latin(""), Some(String::from("")));
}

#[test]
fn word_to_pig_latin_ascii_vowel() {
    assert_eq!(collections::word_to_pig_latin("apple"), Some(String::from("apple-hay")));
    assert_eq!(collections::word_to_pig_latin("Orange"), Some(String::from("Orange-hay")));
}

#[test]
fn word_to_pig_latin_ascii_consonant() {
    assert_eq!(collections::word_to_pig_latin("scoops"), Some(String::from("coops-say")));
    assert_eq!(collections::word_to_pig_latin("Rust"), Some(String::from("ust-ray")));
}

#[test]
fn word_to_pig_latin_non_ascii() {
    assert_eq!(collections::word_to_pig_latin("Здравствуйте"), None);
    assert_eq!(collections::word_to_pig_latin(" "), None);
    assert_eq!(collections::word_to_pig_latin(" Fart"), None);
}
