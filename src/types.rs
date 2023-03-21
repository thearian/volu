use std::fs::Metadata;

pub struct MemoryCache {
    pub dirs:  Vec<String>,
    pub files: Vec<FileMetaData>,
}

pub struct FileMetaData {
    pub name: String,
    pub metadata: Metadata
}

struct DirMetaData {
    name: String,
    cache: MemoryCache,
    size: Option<u64>
}
