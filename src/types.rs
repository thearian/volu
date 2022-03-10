pub struct DirMap {
    pub dirname: String,
    pub size: u64
}

pub struct Group<'a> {
    pub parent: &'a DirMap,
    pub children: GroupList<'a>
}

pub type GroupList<'a> = Vec<Group<'a>>;
