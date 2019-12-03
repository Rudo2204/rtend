use std::io::{self, Write};
use textwrap;
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

    pub fn print_content<W: Write>(&self, sink: &mut W) {
        let wrapped_data = textwrap::wrap(&self.alias_list, 29);
        writeln!(
            sink,
            " {:<4} {:<29} {:^8} {:^8} {:^28}",
            self.id,
            wrapped_data[0],
            self.alias_count,
            self.snippet_count,
            time::at(self.created).rfc3339()
        )
        .unwrap();

        for i in 1..wrapped_data.len() {
            writeln!(sink, " {:<4} {:<29}", "", wrapped_data[i]).unwrap();
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

    pub fn print_content<W: Write>(&self, sink: &mut W) {
        let wrapped_data = textwrap::wrap(&self.data, 40);
        if wrapped_data.len() > 1 {
            writeln!(
                sink,
                " {:<4} {:<4} {:<40} {:^28}",
                self.id,
                self.data_type,
                wrapped_data[0],
                time::at(self.last_modified).rfc3339()
            )
            .unwrap();

            for i in 1..wrapped_data.len() {
                writeln!(
                    sink,
                    " {:<4} {:<4} {:<40} {:^28}",
                    "", "", wrapped_data[i], ""
                )
                .unwrap();
            }
        } else {
            writeln!(
                sink,
                " {:<4} {:<4} {:<40} {:^28}",
                self.id,
                self.data_type,
                self.data,
                time::at(self.last_modified).rfc3339()
            )
            .unwrap();
        }
        writeln!(sink, "{}", "-".repeat(80)).unwrap();
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

    pub fn print_content<W: Write>(&self, sink: &mut W) {
        let wrapped_data = textwrap::wrap(&self.data, 45);

        if wrapped_data.len() > 1 {
            writeln!(
                sink,
                " {:<45} {:<6} {:^28}",
                wrapped_data[0],
                self.id,
                time::at(self.updated).rfc3339()
            )
            .unwrap();

            for i in 1..wrapped_data.len() {
                writeln!(sink, " {:<45}", wrapped_data[i]).unwrap();
            }
        } else {
            writeln!(
                sink,
                " {:<45} {:<6} {:^28}",
                self.data,
                self.id,
                time::at(self.updated).rfc3339()
            )
            .unwrap();
        }
        writeln!(sink, "{}", "-".repeat(80)).unwrap();
    }
}
