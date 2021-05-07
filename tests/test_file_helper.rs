use craine::{get_name, is_component};
use std::path::PathBuf;

#[test]
fn test_get_name() {
    assert_eq!(
        get_name(&PathBuf::new().join("/tmp/test/index.html")),
        Some("index".to_string())
    )
}

#[test]
fn test_is_component() {
    assert_eq!(is_component("index".to_owned()), false);
    assert_eq!(is_component("FancyButton".to_owned()), true);
    assert_eq!(is_component("F_A_N".to_owned()), true);
    assert_eq!(is_component("a_S_d_f".to_owned()), false);
    assert_eq!(is_component("0-sd".to_owned()), false);
    assert_eq!(is_component("3de".to_owned()), false);
}
