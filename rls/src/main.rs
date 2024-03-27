use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs;
use std::str;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)] 
    /// Display hidden files 
    all: bool,

    #[arg(short)]
    /// Print [DIRECTORIES] as a long listing
    long_list: bool,

    #[arg(short, long)]
    /// Recursively list any subdirectories
    recursive: bool,

    #[arg(default_value = ".")]
    directories: Option<Vec<PathBuf>>,
}


fn list_directory(dir: &PathBuf, recur: bool, all: bool, long: bool) {
    // Needs to handle three cases:
    // 1. The directory exists and is accessible, display each item
    // 2. The directory exists but we don't have permission, display warning and continue
    // 3. The directory doesn't exist, display warning and continue

    let contents = fs::read_dir(dir);

    match contents {
        Ok(entries) => {
            // TODO: Finish this or something idk
            for entry in entries {
                
            }
        },
        Err(error) => { 
            let msg = match error.kind() {
                ErrorKind::NotFound => format!("Unable to find directory '{}'", dir.to_str().unwrap()),
                ErrorKind::PermissionDenied => format!("Invalid permissions to open directory '{}'", dir.to_str().unwrap()),
                _ => format!(""),
            };
            eprintln!("rls: {msg}");
        }
    }
}


fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");

    if let Some(ref dirs) = cli.directories {
        for dir in dirs {
            list_directory(
                dir, 
                cli.recursive,
                cli.all,
                cli.long_list,
            );
        }
    };
}
