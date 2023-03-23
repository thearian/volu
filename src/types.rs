use std::fs::Metadata;

#[derive(Clone, Debug)]
pub struct MemoryCache {
    open: bool
    pub dirs:  Vec<DirMetaData>,
    pub files: Vec<FileMetaData>,
}

impl MemoryCache {
    pub fn new() -> MemoryCache {
        MemoryCache {
            open: false,
            dirs: Vec::new(),
            files: Vec::new()
        }
    }
    pub fn find(&mut self, id: &String) -> Option<&mut MemoryCache> {
        for dir in self.dirs.iter_mut() {
            if id.eq(&dir.id) {
                return Some(&mut dir.cache)
            }
            else {
                match dir.cache.find(&id) {
                    Some(value) => { return Some(value) },
                    None => { continue }
                }

            }
        }

        return None
    }
    pub fn is_empty(&self) -> bool {
        self.dirs.len() == 0 && self.files.len() == 0
    }
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}

#[derive(Clone, Debug)]
pub struct FileMetaData {
    pub name: String,
    pub metadata: Metadata
}

#[derive(Clone, Debug)]
pub struct DirMetaData {
    pub id: String,
    pub name: String,
    pub cache: MemoryCache,
    pub size: Option<u64>,
}

impl DirMetaData {
    pub fn new(id: String, name: String) -> DirMetaData {
        DirMetaData {
            id,
            name,
            cache: MemoryCache::new(),
            size: None
        }
    }
}


pub enum ViewCommand { Open, Size, None }

pub struct Cursor {
    pub pointer: u8,
    pub index: u8,
    pub command: ViewCommand
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor { pointer: 0, index: 0, command: ViewCommand::None }
    }
    pub fn reset_index(&mut self) {
        self.index = 0;
    }
    pub fn reset_command(&mut self) {
        self.command = ViewCommand::None;
    }
    pub fn is_hovered(&self) -> bool {
        self.index == self.pointer
    }
    pub fn increase_index(&mut self) {
        self.index += 1
    }
    pub fn move_down(&mut self) {
        if self.pointer < self.index {
            self.pointer += 1;
        }
    }
    pub fn move_up(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
    }
    pub fn set_command(&mut self, command: ViewCommand) {
        self.command = command;
    }
}

