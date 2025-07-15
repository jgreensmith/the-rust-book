use collections::add_user;

#[test]
fn add_user_returns_none_for_empty_string() {
    let result = add_user(String::new());
    assert_eq!(result, None);
}

#[test]
fn add_user_returns_engineering() {
    let result = add_user("Add Sally to Engineering".to_string());
    assert_eq!(result, Some(("Sally".to_string(), "to".to_string())));
}

#[test]
fn add_user_returns_marketing() {
    let result = add_user("Add Bob to Marketing".to_string());
    assert_eq!(result, Some(("Bob".to_string(), "to".to_string())));
}

#[test]
fn add_user_returns_finance() {
    let result = add_user("Add Alice to Finance".to_string());
    assert_eq!(result, Some(("Alice".to_string(), "to".to_string())));
}
