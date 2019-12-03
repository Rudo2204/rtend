use std::io::{self, Write};
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

pub struct Alias {
    pub id: u32,
    pub name: String,
    pub updated: Timespec,
}

pub struct Snippet {
    pub id: u32,
    pub data: String,
    pub updated: Timespec,
}

impl Entity {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(sink, " {:<6} {:<28}", "ID", "Created on")?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<6} {:^28}",
            self.id,
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

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<4} {:<29} {:^8} {:^8} {:^28}",
            self.id,
            self.alias_list,
            self.alias_count,
            self.snippet_count,
            time::at(self.created).rfc3339()
        )
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

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        if self.data.lines().count() > 1 {
            let mut tmp = self.data.split('\n').nth(0).unwrap();
            if tmp.len() > 24 {
                tmp = &tmp[0..24];
                writeln!(
                    sink,
                    " {:<4} {:<4} {:<42} {:^28}",
                    self.id,
                    self.data_type,
                    format!("{}... ({} more lines)", tmp, self.data.lines().count() - 1),
                    time::at(self.last_modified).rfc3339()
                )
            } else {
                writeln!(
                    sink,
                    " {:<4} {:<4} {:<42} {:^28}",
                    self.id,
                    self.data_type,
                    format!("{} ({} more lines)", tmp, self.data.lines().count() - 1),
                    time::at(self.last_modified).rfc3339()
                )
            }
        } else {
            if self.data.len() > 39 {
                let mut tmp = self.data.clone();
                tmp.truncate(39);
                writeln!(
                    sink,
                    " {:<4} {:<4} {:<42} {:^28}",
                    self.id,
                    self.data_type,
                    format!("{}...", tmp),
                    time::at(self.last_modified).rfc3339()
                )
            } else {
                writeln!(
                    sink,
                    " {:<4} {:<4} {:<42} {:^28}",
                    self.id,
                    self.data_type,
                    self.data,
                    time::at(self.last_modified).rfc3339()
                )
            }
        }
    }
}

impl Alias {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(sink, " {:<44} {:<6} {:^28}", "Names", "ID", "Created on")?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<45} {:<6} {:^28}",
            self.name,
            self.id,
            time::at(self.updated).rfc3339()
        )
    }
}

impl Snippet {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(sink, " {:<44} {:<6} {:^28}", "Snippets", "ID", "Created on")?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let mut tmp = self.data.split('\n').nth(0).unwrap();
        if self.data.lines().count() > 1 {
            if tmp.len() > 27 {
                tmp = &tmp[0..27];
                writeln!(
                    sink,
                    " {:<45} {:<6} {:^28}",
                    format!("{}... ({} more lines)", tmp, self.data.lines().count() - 1),
                    self.id,
                    time::at(self.updated).rfc3339()
                )
            } else {
                writeln!(
                    sink,
                    " {:<45} {:<6} {:^28}",
                    format!("{} ({} more lines)", tmp, self.data.lines().count() - 1),
                    self.id,
                    time::at(self.updated).rfc3339()
                )
            }
        } else {
            if self.data.len() > 42 {
                let mut tmp = self.data.clone();
                tmp.truncate(42);
                writeln!(
                    sink,
                    " {:<45} {:<6} {:^28}",
                    format!("{}...", tmp),
                    self.id,
                    time::at(self.updated).rfc3339()
                )
            } else {
                writeln!(
                    sink,
                    " {:<45} {:<6} {:^28}",
                    self.data,
                    self.id,
                    time::at(self.updated).rfc3339()
                )
            }
        }
    }
}
