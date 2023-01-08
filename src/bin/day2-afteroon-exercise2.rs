pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefixes = prefix.split('/');
    let request_paths = request_path
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    for (prefix_part, request_path_part_option) in prefixes.zip(request_paths) {
        match request_path_part_option {
            Some(request_path_part) => {
                if prefix_part != "*" && prefix_part != request_path_part {
                    return false
                }
            },
            None => return false
        }
    }

    true
}

#[test]
fn test_matches_without_wildcard_perfect_match() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
}

#[test]
fn test_matches_without_wildcard_with_additional_path_and_match() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
}

#[test]
fn test_matches_without_wildcard_with_additional_paths_and_match() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));
}

#[test]
fn test_matches_without_wildcard_with_only_first_path_matches_and_match() {
    assert!(!prefix_matches("/v1/publishers", "/v1"));
}

#[test]
fn test_matches_without_wildcard_with_additional_string_in_path_and_match() {
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
}

#[test]
fn test_matches_without_wildcard_and_path_does_not_match_at_all() {
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard_and_match_foo() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
}

#[test]
fn test_matches_with_wildcard_and_match_bar() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
}

#[test]
fn test_matches_with_wildcard_serveral_additional_paths_and_match() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));
}

#[test]
fn test_matches_with_wildcard_path_is_short_and_should_not_match() {
    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
}

#[test]
fn test_matches_with_wildcard_and_path_does_not_include_final_path_and_should_not_match() {
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

#[allow(dead_code)]
fn main() {}