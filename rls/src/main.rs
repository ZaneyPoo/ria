use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs::{self, DirEntry, FileType};
use clap::{Parser, ValueEnum};

mod colors {
    pub const WHITE: &str = "\x1b[37m"; // regular files
    pub const BLUE: &str = "\x1b[34m"; // directories
    pub const CYAN: &str = "\x1b[36m";
    pub const PURPLE: &str = "\x1b[35m"; // symlinks
    pub const _RED: &str = "\x1b[31m"; // files with executable bit set
    pub const _GREEN: &str = "\x1b[32m"; 
    pub const RESET: &str = "\x1b[39m";
}

mod markers {
    pub const DIR: char = '/';
    pub const EXE: char = '*';
    pub const SYMLINK: char = '@';
    pub const FIFO: char = '|';
    pub const SOCKET: char = '=';
    pub const DOOR: char = '>';
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)] 
    /// Display hidden files 
    all: bool,

    #[arg(short)]
    /// Print [DIRECTORIES] as a long listing
    long_list: bool,

    #[arg(long, value_enum, value_name = "WHEN")]
    /// Color files depending on their type
    color: Option<When>,

    #[arg(short, long)]
    /// Recursively list any subdirectories
    recursive: bool,

    #[arg(default_value = ".")]
    directories: Option<Vec<PathBuf>>,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum When {
    Always, 
    Auto,
    Never,
}


fn format_file(entry: &DirEntry, opts: &Cli) -> String {
    let filetype = entry.file_type().unwrap();
    let (color, classifier) = match filetype {
        _ if filetype.is_dir() => (colors::BLUE, Some(markers::DIR)),
        _ if filetype.is_symlink() => (colors::CYAN, Some(markers::SYMLINK)),
        _ => (colors::WHITE, None),
    };
    format!("{color}{name}{reset}{classifier}", 
        name = entry.file_name().into_string().unwrap(), 
        reset = colors::RESET,
        classifier = classifier.unwrap_or_default())
}


fn format_line(entries: Vec<DirEntry>, opts: &Cli) -> String {
    let mut line = String::new();
    for entry in entries {
        let filename = entry.file_name().into_string().unwrap();
        if !opts.all && filename.starts_with(".") { continue }

        let formatted = format_file(&entry, opts);

        line.push_str(&formatted);
        line.push_str("  ");
    }

    line
}


#[allow(dead_code, unused_variables)]
fn format_list(entries: Vec<DirEntry>, opts: &Cli) -> String {
    todo!("Not implemented yet :(");
}


#[allow(unused)]
fn list_directory(dir: &PathBuf, opts: &Cli) {
    // Needs to handle three cases:
    // 1. The directory exists and is accessible, display each item
    // 2. The directory exists but we don't have permission, display warning and continue
    // 3. The directory doesn't exist, display warning and continue

    let contents = fs::read_dir(dir);
    let alpha = true;

    match contents {
        Ok(entries) => {
            let mut contents: Vec<DirEntry> = entries
                .filter_map(|e| Some(e.unwrap()))
                .collect();
            let line = if opts.long_list { 
                format_list(contents, opts) 
            } else { 
                format_line(contents, opts) 
            };
            println!("{line}");
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
        let use_labels = dirs.len() > 1;
        for dir in dirs {
            if use_labels { println!("{}:", dir.display()) }
            list_directory(
                dir, 
                &cli
            );
        }
    };
}
