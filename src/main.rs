use clap::Parser;
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::time::{Instant, Duration};
use indicatif::{ProgressStyle, ProgressBar, MultiProgress};

/// Size of directory optic
#[derive(Parser, Debug)]
#[clap(
    author = "Arian Mirahmadi (thearian@github) (mirarianmir@gmail.com)",
    version = "0.0.1",
    about = "Prints the size of the given directory, featuring the heaviest dirs",
    long_about = None
)]
struct Args {
    /// The parent directory
    #[clap(default_value = ".")]
    dir: String,
    /// Print the parent directories
    #[clap(short,long)]
    print: bool,
    /// Sort the parent directories
    #[clap(short,long)]
    sort: bool,
    /// Sort and limit the parent directories
    #[clap(short,long,default_value = "25")]
    limit: u8,
    /// Print all the parent directories, no limit
    #[clap(short,long)]
    all: bool,
    /// Print child of parent directories
    #[clap(short,long)]
    map: bool,
}

trait ByteSize {
    fn add_commas(&self) -> String;
    fn byte_format(&self) -> String;
}
impl ByteSize for u64 {
    fn add_commas(&self) -> String {
        let mut s = self.to_string();
        if s.len() < 2 { return s }
        let range = (1..s.len()-2).rev();
        for i in range.step_by(3) {
            s.insert(i, ',');
        }
        return s;
    }
    fn byte_format(&self) -> String {
        let u64_comma = self.add_commas();
        let commas = u64_comma
            .split(',')
            .collect::<Vec<&str>>();
        let suffix = match commas.len() {
            1 => "bytes",
            2 => "KB",
            3 => "MB",
            4 => "G",
            5 => "T",
            _ => ""
        };
        if commas.len() > 1 {
            return format!("{}.{} {}", commas[0], commas[1], suffix);
        }
        else {
            return format!("{} {}", commas[0], suffix);
        }
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
    let progress_style = ProgressStyle::with_template(
        "{prefix:.bold.dim} {spinner}   {wide_msg}"
    ).unwrap()
        .tick_chars("⠁⠁⠂⠂⠄⠄⡀⡀⢀⢀⠠⠠⠐⠐⠈⠈");
    let progress_steam = MultiProgress::new();
    let now = Instant::now();
    
    let mut dir_map: Vec<DirMap> = Vec::new();
    let mut count = 0u64;
    let mut size = 0u64;
    dir_size(path,&mut size, &mut count, &mut dir_map, &progress_steam,  &progress_style);
    let time = now.elapsed();

    // this is for print design dont touch it. thanks :)
    println!("");

    if args.print || args.sort || args.limit != 25 {
        print_dir_map(dir_map, size, &args);
    }

    println!(
        "\t{} files indexed at {}\n\t{} ({} bytes)",
        count,
        time.to_hms(),
        size.byte_format(),
        size.add_commas()
    );
}

fn dir_size(
    path: &Path,
    size: &mut u64,
    count: &mut u64,
    dir_map: &mut Vec<DirMap>,
    progress_stream: &MultiProgress,
    progress_style: &ProgressStyle
) {
    if !path.exists() { return }
    if path.is_file() {
        *count += 1;
        *size += metadata(path)
            .unwrap()
            .len();
    }
    if path.is_dir() {
        let pb = progress_stream.add(ProgressBar::new_spinner());
        pb.set_style(progress_style.clone());
        pb.set_prefix(format!("[{}]", size.byte_format()));
        for child in read_dir(path).unwrap() {
            let child = child.unwrap();
            let child_path = child.path();
            pb.set_message(
                child_path.to_owned()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            );
            pb.inc(1);
            dir_size(
                &child_path,
                size,
                count,
                dir_map,
                progress_stream,
                progress_style
            );
        }
        dir_map.push(DirMap {
            dirname: path.to_str().unwrap().to_owned(),
            size: *size
        });
    };
}

struct Group<'a> {
    dir_map: &'a DirMap,
    children: Vec<Group<'a>>
}

fn print_dir_map(dir_map: Vec<DirMap>, size: u64, args: &Args) {
    let mut grouped = group_dir_map(&dir_map);
    if args.sort {
        grouped.sort_by(|a, b| b.dir_map.size.cmp(&a.dir_map.size))
    }

    let space_count = size.byte_format().len() as u8;
    println!(
        "SIZE {}SUBS\tDIRECTORY",
        produce_letter(space_count, 4, ' ')
    );
    for (index, grp) in grouped.iter().enumerate() {
        if args.all || index as u8 > args.limit { break }
        let parent_dir_size = grp.dir_map.size.byte_format();
        println!(
            "{} {}({}) {}",
            parent_dir_size,
            produce_letter(space_count, parent_dir_size.len() as u8, ' '),
            grp.children.len(),
            grp.dir_map.dirname
        );

        if args.map {
            print_dir_children(&grp.children, space_count, 1);
        }
    }
    println!("");
}

fn print_dir_children(children: &Vec<Group>, space_count: u8, generation: u8) {
    for child in children {
        let child_dir_size = child.dir_map.size.byte_format();
        println!(
            "{}  {}{}> {}",
            child_dir_size,
            produce_letter(space_count, child_dir_size.len() as u8, ' '),
            count_to_letter(4 * generation, '-'),
            child.dir_map.dirname,
        );
        print_dir_children(&child.children, space_count, generation + 1);
    };
}

fn group_dir_map(dir_map: &Vec<DirMap>) -> Vec<Group> {
    let mut groupes: Vec<Group> = Vec::new();
    for dir in dir_map.iter().rev() {
        let mut new_groupe = true;
        for grp in groupes.iter_mut() {
            if dir.dirname.contains(&grp.dir_map.dirname) {
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

fn count_to_letter(count: u8, letter: char) -> String {
    (0..count)
        .map(|_| letter)
        .collect::<String>()
}

fn produce_letter(space_count: u8, occupied: u8, letter: char) -> String {
    count_to_letter(
        if space_count > occupied {space_count - occupied}
        else {0},
        letter
    )
}