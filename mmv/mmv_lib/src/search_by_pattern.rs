use glob::glob;

use std::path::PathBuf;

///Returns a vector of PathBuf containing paths matching source pattern
/// # Errors
/// This function won't return any Errors
/// # Examples
///
/// ```no_run
///use mmv_lib::search_by_pattern::search_by_pattern;
///
/// fn main() {
///     let source = "mmv_lib/src/*.rs".to_string();
///     let paths = search_by_pattern(&source);
///     assert_eq!(paths.len(), 4);
/// }
/// ```

pub fn search_by_pattern(source_pattern: &String) -> Vec<PathBuf> {
    let mut file_names: Vec<PathBuf> = Vec::new();
    for entry in glob(&source_pattern).unwrap() {
        if entry.is_ok() {
            file_names.push(entry.unwrap());
        }
    }
    file_names
}

#[cfg(test)]
mod tests_search {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn test_lib_files() {
        let source = "src/*.rs".to_string();
        let paths = search_by_pattern(&source);

        assert!(paths.contains(&PathBuf::from("src/build_target_path.rs")));
        assert!(paths.contains(&PathBuf::from("src/lib.rs")));
        assert!(paths.contains(&PathBuf::from("src/mass_move.rs")));
        assert!(paths.contains(&PathBuf::from("src/search_by_pattern.rs")));
        assert!(!paths.contains(&PathBuf::from("src/random_file.rs")));
    }
    #[test]
    fn test_temp_dir() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let _ = File::create(&file_path).unwrap();
        let source = tmp_dir.path().join("*.txt").to_str().unwrap().to_string();
        let paths = search_by_pattern(&source);
        assert!(paths.contains(&file_path));
    }
}
