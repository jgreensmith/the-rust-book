use collections::*;

#[test]
fn median_odd_length() {
    let v = vec![3, 1, 2];
    assert_eq!(median(&v), 2.0);
}

#[test]
fn median_even_length() {
    let v = vec![4, 1, 2, 3];
    assert_eq!(median(&v), 2.5);
}

#[test]
fn median_single_element() {
    let v = vec![42];
    assert_eq!(median(&v), 42.0);
}

#[test]
fn mode_single_mode() {
    let v = vec![1, 2, 2, 3];
    assert_eq!(mode(&v), vec![2]);
}

#[test]
fn mode_multiple_modes() {
    let v = vec![1, 2, 2, 3, 3];
    let mut result = mode(&v);
    result.sort();
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn mode_all_unique() {
    let v = vec![1, 2, 3, 4];
    let mut result = mode(&v);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4]);
}
