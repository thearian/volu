use std::fs::Metadata;

#[derive(Clone)]
pub struct MemoryCache {
    pub id: u32,
    pub dirs:  Vec<DirMetaData>,
    pub files: Vec<FileMetaData>,
}

impl MemoryCache {
    pub fn new() -> MemoryCache {
        let something = true;
        let address = &something as *const bool;
        let bad_rand =  address as u32;
        MemoryCache {
            id: bad_rand,
            dirs: Vec::new(),
            files: Vec::new()
        }
    }
    pub fn find(&mut self, id: u32) -> Option<&mut MemoryCache> {
        if id == self.id { return Some(self) }

        for dir in self.dirs.iter_mut() {
            if id == dir.cache.id {
                return Some(&mut dir.cache)
            }
            else {
                match dir.cache.find(id) {
                    Some(value) => { return Some(value) },
                    None => { continue }
                }

            }
        }

        return None
    }
    pub fn is_empty(&self) -> bool {
        if self.dirs.len() > 0 { return false }
        if self.files.len() > 0 { return false }
        return true;
    }
}

#[derive(Clone)]
pub struct FileMetaData {
    pub name: String,
    pub metadata: Metadata
}

#[derive(Clone)]
pub struct DirMetaData {
    pub name: String,
    pub cache: MemoryCache,
    pub size: Option<u64>
}
