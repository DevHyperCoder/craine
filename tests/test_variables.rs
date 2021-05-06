use craine::var_parser::*;
use std::collections::HashMap;

#[test]
fn test_get_variables() {
    let content = r#"
    {color||blue}
    {border_color||black}
    {image_url||https://devhypercoder.com}
    {10-line||asdf}
    {stringed-value||"hey there"}
    "#;

    let content_vec: Vec<&str> = content.split("\n").collect();
    let variables = get_variables(&content_vec);
    assert!(variables.is_ok());

    let mut expected = HashMap::new();
    expected.insert("color".to_string(), "blue".to_string());
    expected.insert("border_color".to_string(), "black".to_string());
    expected.insert(
        "image_url".to_string(),
        "https://devhypercoder.com".to_string(),
    );
    expected.insert("10-line".to_string(), "asdf".to_string());
    expected.insert("stringed-value".to_string(), "\"hey there\"".to_string());

    assert_eq!(variables.unwrap(), expected);
}

#[test]
fn test_empty_var() {
    let content = r#"
    {||blue}
    "#;

    let content_vec: Vec<&str> = content.split("\n").collect();
    let variables = get_variables(&content_vec);
    assert!(variables.is_err());
}

#[test]
fn test_replace_var() {
    let content = r#"
id=(color)
lorem(color) ipsum "(color)"
    "#;

    let mut variables = HashMap::new();
    variables.insert("color".to_string(), "blue".to_string());
    let new_content = replace_variables(content, variables);

    assert!(new_content.is_ok());
    assert_eq!(
        r#"
id=blue
loremblue ipsum "blue"
    "#,
        new_content.unwrap()
    );
}
