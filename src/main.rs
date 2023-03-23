use std::{
    fs::{
        Metadata,
        metadata,
        read_dir,
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
    DirMetaData
};

fn main() {
    let mut cache = MemoryCache::new();
    let path = Path::new(".");
    populate_cache_by_path(&path, &mut cache, None);

    viewer(cache);
}

fn populate_cache_by_path(
    path: &Path,
    cache: &mut MemoryCache,
    id: Option<u32>
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
            let dir_metadata = DirMetaData {
                name: dirname,
                cache: MemoryCache::new(),
                size: Option::None
            };
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
    command: &ViewCommand,
    cache: &mut MemoryCache,
    tail_spaces: u8
) {
    if hover {
        let spaces = spaces_by_count(tail_spaces);
        println!(" >{} {}", spaces, dir.name);

        match command {
            ViewCommand::Open => {
                let dir_path = Path::new(&dir.name);
                populate_cache_by_path(dir_path,
                    cache,
                    Some(dir.cache.id)
                );
            },
            ViewCommand::Size => {}
            ViewCommand::None => {}
        }
    }
    else {
        println!(" + {}", dir.name);
    }
}

enum ViewCommand { Open, Size, None }

fn viewer(mut cache: MemoryCache) {
    let mut cursor: u8 = 0;
    let mut index: u8;
    let mut command = ViewCommand::None;

    loop {
        // cleargin screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // moving index to the top
        index = 0;

        print_all_children(
            &mut cache,
            &mut index,
            &mut cursor,
            &command,
            0
       );

        println!("\n\r(j: down , k: up , open dir: o , size of dir: s , q: quit) Hit enter to execute");
        print_inline("COMMAND ");

        let direction = wait_for_readstd();

        command = ViewCommand::None;

        match direction {
            'k' => { cursor -= 1 },
            'j' => { cursor += 1 },
            'o' => { command = ViewCommand::Open }
            's' => { command = ViewCommand::Size }
            'q' => { break; }
            _ => {}
        }
    }
}

fn print_all_children(
    cache: &mut MemoryCache,
    index: &mut u8,
    cursor: &mut u8,
    command: &ViewCommand,
    tail_spaces: u8,
) {
    let mut old_cache = cache.clone();
    for dir in old_cache.dirs.iter_mut() {
        print_dir(
            &dir,
            index == cursor,
            command,
            cache,
            tail_spaces
        );
        *index += 1;

        if !dir.cache.is_empty() {
            print_all_children(
                &mut dir.cache,
                index,
                cursor,
                command,
                tail_spaces + 4
            );
        }
    }
    for file in cache.files.iter() {
        print_file_metadata(
            &file.name,
            &file.metadata,
            index == cursor,
            tail_spaces
        );
        *index += 1;
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
