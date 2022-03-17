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
    version = "0.1.0",
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
    let mut dirs: GroupList = Vec::new();
    let mut count = 0u64;
    let mut size = 0u64;

    println!("\n\tSize of {}", args.dir);

    let start_runtime = Instant::now();
    dir_size(path,&mut size, &mut count, &mut dirs, &progress_steam,  &progress_style);
    let runtime = start_runtime.elapsed();

    if args.print || args.sort || args.limit != DEFAULT_PRINT_LIMMIT {
        print_dirs(&mut dirs, &args);
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
    dirs: &mut GroupList,
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
        let mut children = Vec::new();

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
                &mut children,
                progress_stream,
                progress_style
            );
        }
        dirs.push(Group {
            parent: DirMap {
                dirname: path.to_str().unwrap().to_owned(),
                size: *size
            },
            children: children
        });
        return
    };
}

// TODO: time display second is dead
// TODO: show unlisted dirs count for --limit and --map
// TODO: highlight the heaviest

fn print_dirs(dirs: &mut GroupList, args: &Args) {
    if args.sort || args.map {
        dirs.sort_by(|a, b| b.parent.size.cmp(&a.parent.size))
    }
    if args.all {
        println!("");
        return
    }

    let space_count = 9u8;
    println!(
        "SIZE {}SUBS\tDIRECTORY",
        produce_letter(space_count, 4, ' ')
    );
    let mut index = 0;
    for group in dirs.iter() {
        if index >= args.limit { break }
        index += 1;
        print_dir_children(group, space_count, 1, &mut index, args);
    }
    println!("");
}

fn print_dir_children(
    group: &Group,
    space_count: u8,
    generation: u8,
    index: &mut u8,
    args: &Args
) {
    let mut children = group.children.clone();

    if args.sort || args.map {
        children.sort_by(|a, b| b.parent.size.cmp(&a.parent.size))
    }

    for child in children {
        if *index >= args.limit { break }
        let child_dir_size = child.parent.size.display_as_file_size();
        *index += 1;
        println!(
            "{} {}{}|{}> {}",
            child_dir_size,
            produce_letter(space_count, child_dir_size.len() as u8, ' '),
            child.children.len(),
            count_to_letter(2 * generation, '-'),
            child.parent.dirname,
        );
        if args.map {
            print_dir_children(&child, space_count, generation + 1, index, args);
        }
    };
}