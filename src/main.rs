extern crate walkdir;
extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use walkdir::WalkDir;

use std::env;
use std::path::Path;
use std::fs;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Expected a file path as argument. Found none.");
        std::process::exit(1);
    }

    let pathstr = args.get(1).unwrap();
    let relpath = Path::new(pathstr);

    let abspath_res = fs::canonicalize(&relpath);
    if abspath_res.is_err() {
        eprintln!("Could not resolve path.");
        std::process::exit(1);
    }

    let abspath = abspath_res.unwrap();
    list_files(abspath.to_str().unwrap());
}

fn list_files(path: &str) {
    let mut modified_time: SystemTime;
    let mut datetime: DateTime<Utc>;
    let mut scan_path: String;
    for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_meta = e.metadata().unwrap();
        if file_meta.is_file() {
            modified_time = file_meta.modified().unwrap();
            datetime = modified_time.into();
            scan_path = e.path().display().to_string();
            scan_path = scan_path.replace(path, "");
            println!("{};{};{}", scan_path, datetime.timestamp(), file_meta.len());
        }
    }
}