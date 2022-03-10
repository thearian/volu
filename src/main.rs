use clap::Parser;
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::time::Instant;
use indicatif::{ProgressStyle, ProgressBar, MultiProgress};

mod types;
use types::{DirMap, Group, GroupList};

mod display_u64_as_file_size;
use display_u64_as_file_size::DisplayFileSize;

mod display_duration_as_hms;
use display_duration_as_hms::Hms;

mod display_letters_by_u8;
use display_letters_by_u8::{count_to_letter, produce_letter};

static DEFAULT_PRINT_LIMMIT: u8 = 25;
static DEFAULT_PRINT_LIMMIT_STR: &str = "25";

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
    #[clap(short,long,default_value = DEFAULT_PRINT_LIMMIT_STR)]
    limit: u8,
    /// Print all the parent directories, no limit
    #[clap(short,long)]
    all: bool,
    /// Print child of parent directories
    #[clap(short,long)]
    map: bool,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.dir);
    let progress_style = ProgressStyle::with_template(
        "{prefix:.bold.dim} {spinner}   {wide_msg}"
    ).unwrap()
        .tick_chars("⠁⠁⠂⠂⠄⠄⡀⡀⢀⢀⠠⠠⠐⠐⠈⠈");
    let progress_steam = MultiProgress::new();
    let mut dirs: Vec<DirMap> = Vec::new();
    let mut count = 0u64;
    let mut size = 0u64;

    println!("\n\tSize of {}", args.dir);

    let start_runtime = Instant::now();
    dir_size(path,&mut size, &mut count, &mut dirs, &progress_steam,  &progress_style);
    let runtime = start_runtime.elapsed();

    if args.print || args.sort || args.limit != DEFAULT_PRINT_LIMMIT {
        print_dirs(dirs, size, &args);
    }

    println!(
        "\t{} files indexed at {}\n\t{} ({} bytes)",
        count,
        runtime.to_hms(),
        size.display_as_file_size(),
        size.add_commas()
    );
}

fn dir_size(
    path: &Path,
    size: &mut u64,
    count: &mut u64,
    dirs: &mut Vec<DirMap>,
    progress_stream: &MultiProgress,
    progress_style: &ProgressStyle
) {
    if !path.exists() { return }
    if path.is_file() {
        *count += 1;
        *size += metadata(path)
            .unwrap()
            .len();
        return
    }
    if path.is_dir() {
        let pb = progress_stream.add(ProgressBar::new_spinner());
        pb.set_style(progress_style.clone());
        pb.set_prefix(format!("[{}]", size.display_as_file_size()));
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
                dirs,
                progress_stream,
                progress_style
            );
        }
        dirs.push(DirMap {
            dirname: path.to_str().unwrap().to_owned(),
            size: *size
        });
        return
    };
}

fn print_dirs(dirs: Vec<DirMap>, size: u64, args: &Args) {
    let mut grouped = group_dirs(&dirs);
    if args.sort {
        grouped.sort_by(|a, b| b.parent.size.cmp(&a.parent.size))
    }

    let space_count = size.display_as_file_size().len() as u8;
    println!(
        "SIZE {}SUBS\tDIRECTORY",
        produce_letter(space_count, 4, ' ')
    );
    for (index, grp) in grouped.iter().enumerate() {
        if args.all || index as u8 > args.limit { break }
        let parent_dir_size = grp.parent.size.display_as_file_size();
        println!(
            "{} {}({}) {}",
            parent_dir_size,
            produce_letter(space_count, parent_dir_size.len() as u8, ' '),
            grp.children.len(),
            grp.parent.dirname
        );

        if args.map {
            print_dir_children(&grp.children, space_count, 1);
        }
    }
    println!("");
}

fn print_dir_children(children: &GroupList, space_count: u8, generation: u8) {
    for child in children {
        let child_dir_size = child.parent.size.display_as_file_size();
        println!(
            "{}  {}{}> {}",
            child_dir_size,
            produce_letter(space_count, child_dir_size.len() as u8, ' '),
            count_to_letter(4 * generation, '-'),
            child.parent.dirname,
        );
        print_dir_children(&child.children, space_count, generation + 1);
    };
}

fn group_dirs(ungrouped_dirs: &Vec<DirMap>) -> GroupList {
    let mut groupes: GroupList = Vec::new();
    for dir in ungrouped_dirs.iter().rev() {
        let mut new_groupe = true;
        for grp in groupes.iter_mut() {
            if dir.dirname.contains(&grp.parent.dirname) {
                grp.children.push(Group {
                    parent: dir,
                    children: Vec::new()
                });
                new_groupe = false;
            }
        }
        if new_groupe {
            groupes.push(Group {
                parent: dir,
                children: Vec::new()
            })
        }
    };
    groupes
}
