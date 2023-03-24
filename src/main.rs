use std::{
    fs::{
        Metadata,
        metadata,
        read_dir, canonicalize,
    },
    path::Path,
    io,
    io::{
        stdin,
        stdout,
        Write,
        Read
    }
};

mod display_u64_as_file_size;
use display_u64_as_file_size::DisplayFileSize;

mod types;
use types::{
    MemoryCache,
    FileMetaData,
    DirMetaData,
    Cursor
};

use crate::types::ViewCommand;

fn main() {
    let mut cache = MemoryCache::new();
    let path = Path::new(".");
    populate_cache_by_path(&path, &mut cache, None);

    viewer(cache);
}

fn populate_cache_by_path(
    path: &Path,
    cache: &mut MemoryCache,
    id: Option<&String>
) {
    let readed_path = read_dir(path).unwrap();
    let selected_cache = match id {
        None => cache,
        Some(id) => match cache.find(id) {
            None => cache,
            Some(c) => c
        }
    };
    for child in readed_path {
        let child = child.unwrap();
        let child_path = child.path();

        if child_path.is_file() {
            let child_metadata = metadata(&child_path).unwrap();
            let filename = child_path.file_name()
                .ok_or("-uknown-")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let file_metadata = FileMetaData {
                name: filename,
                metadata: child_metadata
            };
            selected_cache.files.push(file_metadata);
        }
        else if child_path.is_dir() {
            let dirname = child_path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let fullpath = canonicalize(child_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let dir_metadata = DirMetaData::new(fullpath, dirname);

            selected_cache.dirs.push(dir_metadata);
        }
    }
}


fn print_file_metadata(
    filename: &String,
    metadata: &Metadata,
    hover: bool,
    tail_spaces: u8
) {
    // let permissions = match metadata.permissions().readonly() {
        // true  => " R ",
        // false => "R&W"
    // };
    let size = DisplayFileSize::display_as_file_size(
        &metadata.len()
    );
    // let created_time = metadata.created()
            // .unwrap()
            // .elapsed()
            // .unwrap()
            // .to_hms();
    // let modified_time = metadata.modified()
            // .unwrap()
            // .elapsed()
            // .unwrap()
            // .to_hms();
    let status = if hover { ">" } else { " " };

    print_neatly(
        &format!("{}{} {}", 
            status,
            spaces_by_count(tail_spaces),
            filename
            // permissions,
            // created_time,
            // modified_time
        ),
        &size
    );
}

fn print_neatly(tail: &str, body: &str) {
    let spaces_needed = (
        90 - tail.len() - body.len()
    ) as u8;
    
    println!("{}{}{}", tail, spaces_by_count(spaces_needed), body);
}


fn print_dir(
    dir: &DirMetaData,
    hover: bool,
    cursor: &Cursor,
    cache: &mut MemoryCache,
    tail_spaces: u8
) {
    let spaces = spaces_by_count(tail_spaces);
    if hover {
        println!(" {}> {}", spaces, dir.name);

        match cursor.command {
            ViewCommand::Open => {
                cache.find(&dir.id).unwrap().toggle();

                if dir.cache.is_empty() {
                    let dir_path = Path::new(&dir.id);
                    populate_cache_by_path(
                        dir_path,
                        cache,
                        Some(&dir.id)
                    );
                }
                
            },
            ViewCommand::Size => {}
            ViewCommand::None => {}
        }
    }
    else {
        println!(" {}+ {}", spaces, dir.name);
    }
}

fn viewer(mut cache: MemoryCache) {
    let mut cursor = Cursor::new();

    loop {
        // cleargin screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        cursor.reset_index();

        print_all_children( &mut cache, &mut cursor, 0);

        println!("\n\r {}/{} (j: down , k: up , open dir: o , size of dir: s , q: quit) Hit enter to execute",
            cursor.pointer,
            cursor.index
        );
        print_inline("COMMAND ");

        let direction = wait_for_readstd();
        cursor.reset_command();
        match direction {
            'k' => { cursor.move_up() },
            'j' => { cursor.move_down() },
            'o' => { cursor.set_command(ViewCommand::Open) }
            's' => { cursor.set_command(ViewCommand::Size) }
            'q' => { break; }
            _ => {}
        }
    }
}

fn print_all_children(
    cache: &mut MemoryCache,
    cursor: &mut Cursor,
    tail_spaces: u8,
) {
    let mut old_cache = cache.clone();
    for dir in old_cache.dirs.iter_mut() {
        print_dir(
            &dir,
            cursor.is_hovered(),
            &cursor,
            cache,
            tail_spaces
        );
        cursor.increase_index();

        if !dir.cache.is_empty() && dir.cache.is_open() {
            print_all_children(
                &mut dir.cache,
                cursor,
                tail_spaces + 2
            );
        }
    }
    for file in cache.files.iter() {
        print_file_metadata(
            &file.name,
            &file.metadata,
            cursor.is_hovered(),
            tail_spaces
        );
        cursor.increase_index();
    }
}


fn wait_for_readstd() -> char {
    let mut character = [0];
    while let Ok(_) = stdin().read(&mut character) {
        return character[0] as char
    }

    let _ = stdout().flush();

    let ch = stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32)
        .expect("Not a char");

    return std::char::from_u32(ch as u32)
        .expect("Not a valid char")
}


fn print_inline(text: &str) {
    print!("{}", text);
    io::stdout().flush().unwrap();
}

fn spaces_by_count(count: u8) -> String {
    let mut spaces = String::new();
    for _ in 0..count { spaces.push(' ') }
    return spaces
}
