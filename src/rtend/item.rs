use time::Timespec;

pub struct Entity {
    pub id: u32,
    pub created: Timespec,
}

pub struct EntityLong {
    pub id: u32,
    pub alias_list: String,
    pub alias_count: u32,
    pub snippet_count: u32,
    pub created: Timespec,
}

pub struct EntityLongLong {
    pub id: u32,
    pub data_type: String,
    pub data: String,
    pub last_modified: Timespec,
}
