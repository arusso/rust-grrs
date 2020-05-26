#[test]
fn test_find_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolar sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
