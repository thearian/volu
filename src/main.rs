use clap::Parser;
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::time::{Instant, Duration};

/// Size of directory optic
#[derive(Parser, Debug)]
#[clap(
    author = "Arian Mirahmadi (thearian@github) (mirarianmir@gmail.com)",
    version = "0.0.1",
    about = "Maps the size of all the child directories",
    long_about = None
)]
struct Args {
    /// The parent directory
    #[clap(default_value = ".")]
    dir: String
}

trait Commas {
    fn add_commas(&self) -> String;
}
impl Commas for u64 {
    fn add_commas(&self) -> String {
        let mut s = self.to_string();
        let range = (1..s.len()-2).rev();
        for i in range.step_by(3) {
            s.insert(i, ',');
        }
        return s;
    }
}

trait Hms {
    fn to_hms(&self) -> String;
}
impl Hms for Duration {
    fn to_hms(&self) -> String {
        let millis: u32 = self.as_millis() as u32;
        if millis < 1000 { return format!("{}ms", millis) }
        let seconds: u32 = millis / 1_000;
        let secs: u32 = seconds % 60;
        let mins: u32 = seconds / 60;
        format!("{}:{}s {}ms", mins, secs, millis % 1000)
    }
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.dir);
    let now = Instant::now();
    
    let mut count = 0u64;
    let size = dir_size(path, &mut count);
    let time = now.elapsed();

    println!(
        "{} files indexed at {}\n\n {} bytes",
        count,
        time.to_hms(),
        size.add_commas()
    )
}


fn dir_size(path: &Path, count: &mut u64) -> u64 {
    if !path.exists() { return 0 }
    if path.is_file() {
        *count += 1;
        return metadata(path)
            .unwrap()
            .len();
    }
    if path.is_dir() {
        let mut size = 0;
        for child in read_dir(path).unwrap() {
            let child = child.unwrap();
            let child_path = child.path();
            size += dir_size(&child_path, count);
        }
        return size;
    }
    return 0u64;
}