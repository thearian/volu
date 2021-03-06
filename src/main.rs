use clap::Parser;
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::time::Instant;
use console::style;

mod types;
use types::{DirMap, Group, GroupList, ProgressOptional};

mod display_u64_as_file_size;
use display_u64_as_file_size::DisplayFileSize;

mod display_duration_as_hms;
use display_duration_as_hms::Hms;

mod display_letters_by_u8;
use display_letters_by_u8::{count_to_letter, produce_letter};

mod progress_bar;
use progress_bar::{init_progress_bar, new_progress, tick_progress};

static DEFAULT_PRINT_LIMMIT: u32 = 25;
static DEFAULT_PRINT_LIMMIT_STR: &str = "25";

/// Size of directory optic
#[derive(Parser, Debug)]
#[clap(
    author = "Arian Mirahmadi (thearian@github) (mirarianmir@gmail.com)",
    version = "0.2.1",
    about = "Prints the size of the given directory, featuring the largest dirs",
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
    /// Sort and limit the count of parent directories
    #[clap(short,long,default_value = DEFAULT_PRINT_LIMMIT_STR)]
    limit: u32,
    /// Print and limit the count of children directories
    #[clap(long,default_value = "9999")]
    child_limit: u32,
    /// Print all the parent directories, no limit
    #[clap(short,long)]
    all: bool,
    /// Print child of parent directories
    #[clap(short,long)]
    map: bool,
    /// Doesnt show progress which causes better performance
    #[clap(short,long)]
    fast: bool,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.dir);
    let mut dirs: GroupList = Vec::new();
    let mut count = 0u64;
    let mut size = 0u64;

    let progress = init_progress_bar(args.fast);

    println!("\n\tSize of {}", args.dir);

    let start_runtime = Instant::now();
    dir_size(path,&mut size, &mut count, &mut dirs, &progress);
    let runtime = start_runtime.elapsed();

    if args.print || args.sort || args.map || args.limit != DEFAULT_PRINT_LIMMIT {
        print_dirs(&mut dirs, &size, &args);
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
    progress: &ProgressOptional
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
        let mut children = Vec::new();
        let mut path_size = 0u64;

        let pb = new_progress(progress, &size.display_as_file_size());

        for child in read_dir(path).unwrap() {
            let child = child.unwrap();
            let child_path = child.path();
            tick_progress(&pb, &child_path);
            dir_size(
                &child_path,
                &mut path_size,
                count,
                &mut children,
                progress
            );
        }
        *size += path_size;
        dirs.push(Group {
            parent: DirMap {
                dirname: path.to_str().unwrap().to_owned(),
                size: path_size
            },
            children: children
        });
        return
    };
}

fn print_dirs(dirs: &mut GroupList, size: &u64, args: &Args) {
    if args.sort || args.map {
        dirs.sort_by(|a, b| b.parent.size.cmp(&a.parent.size))
    }

    let space_count = 10u8;
    println!(
        "SIZE   {}SUBS\tDIRECTORY",
        produce_letter(space_count, 4, ' ')
    );
    let mut index = 0u32;
    for group in dirs.iter() {
        if !args.all && index >= args.limit {
            break
        }
        index += 1;
        print_dir_children(group, space_count, size, 1, &mut index, args);
    }
    println!("");
}

fn print_dir_children(
    group: &Group,
    space_count: u8,
    max_size: &u64,
    generation: u8,
    index: &mut u32,
    args: &Args
) {
    let mut children = group.children.clone();

    if args.sort || args.map {
        children.sort_by(|a, b| b.parent.size.cmp(&a.parent.size))
    }

    let mut children_index = 0u32;
    for child in children {
        if !args.all && *index >= args.limit || children_index >= args.child_limit {
            println!(
                "\t{}... other child dirs are included",
                count_to_letter(2 * generation, ' '),
            );
            break
        }
        let child_dir_size = child.parent.size.to_owned()
            .display_as_file_size();
        let children_count = child.children.len();
        let children_count_string_len = format!("{}", children_count).len() as u8;
        *index += 1;
        children_index += 1;
        let styled_size = match 100 * child.parent.size / *max_size {
            0..=5      => style(child.parent.size.display_as_file_size()),
            6..=10     => style(child.parent.size.display_as_file_size()).green(),
            11..=15     => style(child.parent.size.display_as_file_size()).blue(),
            16..=20     => style(child.parent.size.display_as_file_size()).yellow(),
            21..=100    => style(child.parent.size.display_as_file_size()).red(),
            _           => style(child.parent.size.display_as_file_size())
        };
        println!(
            "{}{}  |{}{}|{}> {}",
            styled_size,
            produce_letter(10, child_dir_size.len() as u8 ,' '),
            children_count,
            produce_letter(4, children_count_string_len , ' '),
            count_to_letter(2 * generation, '-'),
            child.parent.dirname,
        );
        if args.map && child.children.len() > 0 {
            print_dir_children(&child, space_count, max_size, generation + 1, index, args);
        }
    };
}
