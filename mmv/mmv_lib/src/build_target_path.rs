//!Returns a path matching target_pattern

use regex::Captures;

///Returns a String containing path matching target_pattern
/// # Errors
/// This function won't return any Errors, as it's guaranteed that cap_iter is not empty
/// # Examples
///
/// ```no_run
/// use regex::{Regex, Captures};
/// use mmv_lib::mass_move::build_target_path;
///
/// fn main() {
///     let re = Regex::new(r"a(.*)c1(.*)3").unwrap();
///     let caps = re.captures("abc123").unwrap();
///     let target = build_target_path(caps, &"a#2c1#13".to_string());
///     assert_eq!("a2c1b3", target);
/// }
/// ```
pub fn build_target_path(cap_iter: Captures, target_pattern: &String) -> String {
    let mut target_path = target_pattern.clone();

    for x in 1..cap_iter.len() {
        let part = cap_iter.get(x).unwrap().as_str();
        target_path = str::replace(&target_path, &format!("#{}", x), part);
    }
    target_path
}

#[cfg(test)]
mod tests_build {
    use super::*;
    use crate::search_by_pattern::search_by_pattern;
    use regex::Regex;
    use std::path::PathBuf;

    #[test]
    fn test_one_replace() {
        let re = Regex::new(r"a(.*)").unwrap();
        let caps = re.captures("abc123").unwrap();
        let target = build_target_path(caps, &"#1a".to_string());
        assert_eq!("bc123a", target);
    }
    #[test]
    fn test_several_replaces() {
        let re = Regex::new(r"a(.*)c1(.*)3").unwrap();
        let caps = re.captures("abc123").unwrap();
        let target = build_target_path(caps, &"#1#2".to_string());
        assert_eq!("b2", target);
    }
    #[test]
    fn test_absolute_path() {
        let source =
            "/Users/vladpolyakov/rust-hse-2023-poliakovva/problems/projects/mmv/mmv_lib/src/*.rs"
                .to_string();
        let paths = search_by_pattern(&source);

        assert!(paths.contains(&PathBuf::from("/Users/vladpolyakov/rust-hse-2023-poliakovva/problems/projects/mmv/mmv_lib/src/build_target_path.rs")));
        assert!(paths.contains(&PathBuf::from(
            "/Users/vladpolyakov/rust-hse-2023-poliakovva/problems/projects/mmv/mmv_lib/src/lib.rs"
        )));
        assert!(paths.contains(&PathBuf::from("/Users/vladpolyakov/rust-hse-2023-poliakovva/problems/projects/mmv/mmv_lib/src/mass_move.rs")));
        assert!(paths.contains(&PathBuf::from("/Users/vladpolyakov/rust-hse-2023-poliakovva/problems/projects/mmv/mmv_lib/src/search_by_pattern.rs")));
        assert!(!paths.contains(&PathBuf::from("/Users/vladpolyakov/rust-hse-2023-poliakovva/problems/projects/mmv/mmv_lib/src/random_file.rs")));
    }
}
