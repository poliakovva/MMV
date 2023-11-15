//! Moves or renames files by 'from' and 'to' patterns

extern crate regex;
pub use crate::build_target_path::build_target_path;
pub use crate::search_by_pattern::search_by_pattern;
use regex::Regex;
use std::{
    fs::{create_dir_all, rename},
    path::Path,
};

fn build_regex(file_name: &str) -> Regex {
    let mut regex = str::replace(&file_name, '.', r"\.");
    regex = str::replace(&regex, '*', r"(.*)");
    Regex::new(&regex).unwrap()
}
/// Moves or renames all files matching String source_pattern to files matching String target_pattern. If moving is susuccessfull, prints all changes made
/// # Errors
/// Function will exit with exitcode 1 if:
/// * Files matching source pattern not found
/// * File that matches target pattern exists and flag -f wasn't given
/// # Examples
///
/// ```no_run
/// use mmv_lib::mass_move::mass_move;
/// fn main() {
///     let input = "path/to/some_*_filename.*".to_string();
///     let target = "path2/to/changed_#1_filename.#2".to_string();
///     //mass_move(input, target, true);
///     }
///  //path/to/some_A_filename.bin -> path2/to/changed_A_filename.bin
///  //path/to/some_A_filename.jpg -> path2/to/changed_A_filename.jpg
///  //path/to/some_B_filename.bin -> path2/to/changed_B_filename.bin
///  //path/to/some_B_filename.jpg -> path2/to/changed_B_filename.jpg
/// ```

pub fn mass_move(source_pattern: String, target_pattern: String, force: bool) {
    let file_names = search_by_pattern(&source_pattern);
    if file_names.is_empty() {
        println!("Files for pattern {} not found", source_pattern);
        std::process::exit(1);
    }

    let regex = build_regex(&source_pattern);
    let mut changed_names: Vec<String> = Vec::new();

    for i in &file_names {
        let captures = regex.captures(&i.as_os_str().to_str().unwrap()).unwrap();
        changed_names.push(build_target_path(captures, &target_pattern))
    }

    let target_path = Path::new(&target_pattern);
    let parent_path = match target_path.parent() {
        Some(_) => target_path.parent().unwrap(),
        None => Path::new(&"."),
    };
    let _ = create_dir_all(parent_path);

    for i in 0..file_names.len() {
        let changed_path = Path::new(&changed_names[i]);
        if !changed_path.exists() || force {
            let rename_result = rename(file_names[i].as_path(), &changed_path);
            match rename_result {
                Ok(_) => (),
                Err(error) => println!("mmv: {}", error),
            }
        } else {
            println!("Not able to replace existing file {} ", changed_names[i]);
            std::process::exit(1);
        }
    }
    for i in 0..file_names.len() {
        println!(
            "{} -> {}",
            file_names[i].to_str().unwrap(),
            changed_names[i].as_str()
        );
    }
}

#[cfg(test)]
mod test_mmv {
    use super::*;
    use std::fs::{metadata, read_to_string, File};
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_rename() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let _ = File::create(&file_path).unwrap();
        let source = tmp_dir.path().join("*.txt").to_str().unwrap().to_string();
        let target = tmp_dir
            .path()
            .join("changed_#1.txt")
            .to_str()
            .unwrap()
            .to_string();
        mass_move(source, target, false);
        let right_target_path = tmp_dir.path().join("changed_my-temporary-note.txt");
        assert_eq!(true, metadata(right_target_path).is_ok());
    }
    #[test]
    fn test_move() {
        let tmp_dir1 = TempDir::new("src").unwrap();
        let file_path = tmp_dir1.path().join("my-temporary-note.txt");

        let _ = File::create(&file_path).unwrap();
        let source = tmp_dir1.path().join("*.txt").to_str().unwrap().to_string();
        let tmp_dir2 = TempDir::new("target").unwrap();
        let target = tmp_dir2
            .path()
            .join("changed_#1.txt")
            .to_str()
            .unwrap()
            .to_string();
        mass_move(source, target, false);
        let right_target_path = tmp_dir2.path().join("changed_my-temporary-note.txt");
        assert_eq!(true, metadata(right_target_path).is_ok());
    }
    #[test]
    fn test_force_move() {
        let tmp_dir1 = TempDir::new("src").unwrap();
        let file_path = tmp_dir1.path().join("my-temporary-note.txt");

        let mut file = File::create(&file_path).unwrap();
        let _ = file.write_all(b"Brian was here. Briefly.");

        let source = tmp_dir1.path().join("*.txt").to_str().unwrap().to_string();
        let tmp_dir2 = TempDir::new("target").unwrap();
        let exist_file_path = tmp_dir2.path().join("changed_my-temporary-note.txt");
        let _ = File::create(&exist_file_path).unwrap();

        let target = tmp_dir2
            .path()
            .join("changed_#1.txt")
            .to_str()
            .unwrap()
            .to_string();
        mass_move(source, target, true);
        let right_target_path = tmp_dir2.path().join("changed_my-temporary-note.txt");
        assert_eq!(true, metadata(&right_target_path).is_ok());
        let contents = read_to_string(&right_target_path).unwrap();
        assert_eq!(contents, "Brian was here. Briefly.");
    }
    #[test]
    fn test_mass_move() {
        let tmp_dir1 = TempDir::new("src").unwrap();
        let note1_path = tmp_dir1.path().join("note1.txt");
        let note2_path = tmp_dir1.path().join("note2.txt");
        let note3_path = tmp_dir1.path().join("note3.txt");

        let _ = File::create(&note1_path).unwrap();
        let _ = File::create(&note2_path).unwrap();
        let _ = File::create(&note3_path).unwrap();

        let source = tmp_dir1.path().join("*.txt").to_str().unwrap().to_string();
        let tmp_dir2 = TempDir::new("target").unwrap();
        let target = tmp_dir2
            .path()
            .join("changed_#1.txt")
            .to_str()
            .unwrap()
            .to_string();
        mass_move(source, target, false);
        let right_note1_path = tmp_dir2.path().join("changed_note1.txt");
        assert_eq!(true, metadata(right_note1_path).is_ok());
        let right_note2_path = tmp_dir2.path().join("changed_note2.txt");
        assert_eq!(true, metadata(right_note2_path).is_ok());
        let right_note3_path = tmp_dir2.path().join("changed_note3.txt");
        assert_eq!(true, metadata(right_note3_path).is_ok());
    }
}
