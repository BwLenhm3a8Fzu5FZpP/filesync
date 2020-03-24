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
    if args.len() > 1 {
        if let Some(pathstr) = args.get(1) {
            let relpath = Path::new(pathstr);
            if let Ok(abspath) = fs::canonicalize(&relpath) {
                println!("{:?}", abspath);
                list_files(pathstr);
            } else {
                eprintln!("Could not resolve path")
            }
        }
    } else {
        eprintln!("Missing PATH parameter")
    }
}

fn list_files(path: &str) {
    let mut modified_time: SystemTime;
    let mut datetime: DateTime<Utc>;
    for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_meta = e.metadata().unwrap();
        if file_meta.is_file() {
            modified_time = file_meta.modified().unwrap();
            datetime = modified_time.into();
            println!("{};{};{}", e.path().display(), datetime.to_rfc2822(), file_meta.len());
        }
    }
}