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
    dir: String,

    /// Print the main directories
    #[clap(short,long)]
    print: bool,

    /// Sort the main directories
    #[clap(short,long)]
    sort: bool
}

trait Commas {
    fn add_commas(&self) -> String;
}
impl Commas for u64 {
    fn add_commas(&self) -> String {
        let mut s = self.to_string();
        if s.len() < 2 { return s }
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

struct DirMap {
    dirname: String,
    size: u64
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.dir);
    let now = Instant::now();
    
    let mut dir_map: Vec<DirMap> = Vec::new();
    let mut count = 0u64;
    let size = dir_size(path, &mut count, &mut dir_map);
    let time = now.elapsed();

    if args.print {
        print_dir_map(dir_map, args.sort);
    }

    println!(
        "\t{} files indexed at {}\n\t{} bytes",
        count,
        time.to_hms(),
        size.add_commas()
    );
}

fn dir_size(path: &Path, count: &mut u64, dir_map: &mut Vec<DirMap>) -> u64 {
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
            size += dir_size(&child_path, count, dir_map);
        }
        dir_map.push(DirMap {
            dirname: path.to_str().unwrap().to_owned(),
            size: size
        });
        return size;
    }
    return 0u64;
}

struct Group<'a> {
    dir_map: &'a DirMap,
    children: Vec<Group<'a>>
}

fn print_dir_map(dir_map: Vec<DirMap>, sort: bool) {
    let mut grouped = group_dir_map(&dir_map);
    if sort {
        grouped.sort_by(|a, b| b.dir_map.size.cmp(&a.dir_map.size))
    }
    println!("\nSIZE\tCHILDREN\tDIRECTORY");
    for grp in grouped {
        println!(
            "{}\t({})\t{}",
            grp.dir_map.size.add_commas(),
            grp.children.len(),
            grp.dir_map.dirname
        );
    }
    println!("");
}

fn group_dir_map(dir_map: &Vec<DirMap>) -> Vec<Group> {
    let mut groupes: Vec<Group> = Vec::new();
    for dir in dir_map.iter() {
        let mut new_groupe = true;
        for grp in groupes.iter_mut() {
            if grp.dir_map.dirname.contains(&dir.dirname) {
                grp.children.push(Group {
                    dir_map: dir,
                    children: Vec::new()
                });
                new_groupe = false;
            }
        }
        if new_groupe {
            groupes.push(Group {
                dir_map: dir,
                children: Vec::new()
            })
        }
    };
    groupes
}