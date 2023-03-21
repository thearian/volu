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

mod display_duration_as_hms;
use display_duration_as_hms::Hms;

mod types;
use types::{
    MemoryCache,
    FileMetaData
};

fn main() {
    let mut cache = MemoryCache {
        dirs: Vec::new(),
        files: Vec::new()
    };

    let path = Path::new(".");
    let readed_path = read_dir(path)
        .unwrap();
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
            cache.files.push(file_metadata);
        }
        else if child_path.is_dir() {
            let dirname = child_path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            cache.dirs.push(dirname);
        }
    }

    viewer(cache);
}


fn print_file_metadata(filename: &String, metadata: &Metadata, hover: bool) {
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

    println!(" {} {}  {}",
        status,
        filename,
        size,
        // permissions,
        // created_time,
        // modified_time
    );
}


fn print_dir(dir: &String, hover: bool) {
    if hover {
        println!(" > {}", dir);
    }
    else {
        println!(" + {}", dir);
    }
}


fn viewer(cache: MemoryCache) {
    let mut cursor: u8 = 0;
    let mut index: u8 = 0;

    loop {
        print_all_children(&cache, &mut index, &mut cursor);

        println!("\n\r(j: down , k: up , q: quit) Hit enter to execute");
        print!("COMMAND ");

        io::stdout().flush().unwrap();

        let direction = wait_for_readstd();
        match direction {
            'k' => { cursor -= 1 },
            'j' => { cursor += 1 },
            'q' => { break; }
            _ => {}
        }
    }
}

fn print_all_children(cache: &MemoryCache, index: &mut u8, cursor: &mut u8) {
    // cleargin screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for dir in cache.dirs.iter() {
        print_dir(&dir, index == cursor);
        *index += 1;
    }
    for file in cache.files.iter() {
        print_file_metadata(&file.name, &file.metadata, index == cursor);
        *index += 1;
    }
    *index = 0;
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

