//! # Mass move
//! mmv moves or renames each source file matching a 'from' pattern to the target name specified by the 'to' pattern.

pub mod build_target_path;
pub mod mass_move;
/// Returns a vector of file names matching a 'from' pattern
pub mod search_by_pattern;
