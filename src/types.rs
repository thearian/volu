#[derive(Clone)]
pub struct DirMap {
    pub dirname: String,
    pub size: u64
}

#[derive(Clone)]
pub struct Group {
    pub parent: DirMap,
    pub children: GroupList
}

pub type GroupList = Vec<Group>;
