use std::{
    fmt,
    io::{self, Write},
};
use time::{self, Timespec};

pub const FALLBACK_ROW_LEN: usize = 79;

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

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " {:<6} {:>28}",
            self.id,
            time::at(self.created).rfc3339()
        )
    }
}

impl Entity {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(sink, " {:<6} {:<28}", "ID", "Created on")?;

        writeln!(sink, "{}", row)
    }
}

impl fmt::Display for EntityLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " {:<4} {:<29} {:^8} {:^8} {:<28}",
            self.id,
            self.alias_list,
            self.alias_count,
            self.snippet_count,
            time::at(self.created).rfc3339()
        )
    }
}

impl EntityLong {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<4} {:<28} {:<8} {:<8} {:^28}",
            "ID", "Alias List", "Aliases", "Snippets", "Created on"
        )?;

        writeln!(sink, "{}", row)
    }
}

impl fmt::Display for EntityLongLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.data.lines().count() > 1 {
            let mut tmp = self.data.split('\n').nth(0).unwrap();
            if tmp.len() > 23 {
                tmp = &tmp[0..23];
                write!(
                    f,
                    " {:<4} {:<4} {:<42} {:>28}",
                    self.id,
                    self.data_type,
                    format!("{}... ({} more lines)", tmp, self.data.lines().count() - 1),
                    time::at(self.last_modified).rfc3339()
                )
            } else {
                write!(
                    f,
                    " {:<4} {:<4} {:<42} {:>28}",
                    self.id,
                    self.data_type,
                    format!("{} ({} more lines)", tmp, self.data.lines().count() - 1),
                    time::at(self.last_modified).rfc3339()
                )
            }
        } else {
            if self.data.len() > 23 {
                let mut tmp = self.data.clone();
                tmp.truncate(39);
                write!(
                    f,
                    " {:<4} {:<4} {:<42} {:<28}",
                    self.id,
                    self.data_type,
                    format!("{}...", tmp),
                    time::at(self.last_modified).rfc3339()
                )
            } else {
                write!(
                    f,
                    " {:<4} {:<4} {:<42} {:<28}",
                    self.id,
                    self.data_type,
                    self.data,
                    time::at(self.last_modified).rfc3339()
                )
            }
        }
    }
}

impl EntityLongLong {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<4} {:<4} {:<40} {:^28}",
            "ID", "Type", "Data", "Created on"
        )?;

        writeln!(sink, "{}", row)
    }
}
