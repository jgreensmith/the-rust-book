use collections::word_to_pig_latin;

#[test]
fn word_to_pig_latin_empty() {
    assert_eq!(word_to_pig_latin(""), Some(String::from("")));
}

#[test]
fn word_to_pig_latin_ascii_vowel() {
    assert_eq!(word_to_pig_latin("apple"), Some(String::from("apple-hay")));
    assert_eq!(word_to_pig_latin("Orange"), Some(String::from("Orange-hay")));
}

#[test]
fn word_to_pig_latin_ascii_consonant() {
    assert_eq!(word_to_pig_latin("scoops"), Some(String::from("coops-say")));
    assert_eq!(word_to_pig_latin("Rust"), Some(String::from("ust-ray")));
}

#[test]
fn word_to_pig_latin_non_ascii() {
    assert_eq!(word_to_pig_latin("Здравствуйте"), None);
    assert_eq!(word_to_pig_latin(" "), None);
    assert_eq!(word_to_pig_latin(" Fart"), None);

}

#[test]
fn median_odd_length() {
    let v = vec![3, 1, 2];
    assert_eq!(collections::median(&v), 2.0);
}

#[test]
fn median_even_length() {
    let v = vec![4, 1, 2, 3];
    assert_eq!(collections::median(&v), 2.5);
}

#[test]
fn median_single_element() {
    let v = vec![42];
    assert_eq!(collections::median(&v), 42.0);
}

#[test]
fn mode_single_mode() {
    let v = vec![1, 2, 2, 3];
    assert_eq!(collections::mode(&v), vec![2]);
}

#[test]
fn mode_multiple_modes() {
    let v = vec![1, 2, 2, 3, 3];
    let mut result = collections::mode(&v);
    result.sort();
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn mode_all_unique() {
    let v = vec![1, 2, 3, 4];
    let mut result = collections::mode(&v);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4]);
}
