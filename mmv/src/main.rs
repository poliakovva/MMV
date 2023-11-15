extern crate clap;

use clap::Parser;
use mmv_lib::mass_move::mass_move;

#[derive(Parser, Debug)]
#[command(
    author = "Vladyslav Poliakov",
    version = "0.1.0",
    about = "Terminal application for mass move or rename of files by pattern",
    long_about = "mmv moves or renames each source file matching a 'from'
    pattern to the target name specified by the 'to' pattern. "
)]
pub struct Args {
    #[arg(
        help = "A pattern for selecting files",
        long_help = "The template for selecting files consists of a path, a name, and a * symbol inside the name
         denoting a substring of any length (including an empty one)."
    )]
    pub source_pattern: String,
    #[arg(
        help = "A pattern for moving/renaming files",
        long_help = "The template for the final path uses special markers like `#1`, `#2' etc. These markers indicate
        which fragments, indicated by asterisks in the original template, should be inserted into the new file name."
    )]
    pub target_pattern: String,
    #[arg(short, long, help = "Overrides target files if they exist")]
    pub force: bool,
}

fn main() {
    let args = Args::parse();

    let source_pattern = args.source_pattern;
    let target_pattern = args.target_pattern;
    let force = args.force;

    mass_move(source_pattern, target_pattern, force);
}
