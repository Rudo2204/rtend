use std::io::{self, Write};
use textwrap;
use time::{self, Timespec};

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

pub struct Relation {
    pub id: u32,
    pub entity_id_a: u32,
    pub entity_id_b: u32,
    pub updated: Timespec,
}

pub struct RelationLong {
    pub id: u32,
    pub entity_id_a: u32,
    pub alias_list_a: String,
    pub entity_id_b: u32,
    pub alias_list_b: String,
    pub updated: Timespec,
}

pub struct RelationSnippet {
    pub id: u32,
    pub data: String,
    pub updated: Timespec,
}

pub struct EntityFound {
    pub id: u32,
    pub name: String,
    pub entity_id: u32,
    pub updated: Timespec,
}

pub struct EntityFoundLong {
    pub id: u32,
    pub name: String,
    pub entity_id: u32,
    pub other_alias: String,
    pub updated: Timespec,
}

pub struct SnippetFound {
    pub id: u32,
    pub data: String,
    pub entity_id: u32,
    pub updated: Timespec,
}

pub struct RelationSnippetFound {
    pub id: u32,
    pub data: String,
    pub relation_id: u32,
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
            "ID", "Type", "Data", "Last modified"
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
        writeln!(sink, "{}", "-".repeat(78)).unwrap();
    }
}

impl Alias {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(sink, " {:<44} {:<6} {:^28}", "Names", "ID", "Last modified")?;

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
        writeln!(
            sink,
            " {:<44} {:<6} {:^28}",
            "Snippets", "ID", "Last modified"
        )?;

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

impl Relation {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<6} {:^12} {:^12} {:^28}",
            "ID", "Entity ID A", "Entity ID B", "Last modified"
        )?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<6} {:^12} {:^12} {:^28}",
            self.id,
            self.entity_id_a,
            self.entity_id_b,
            time::at(self.updated).rfc3339()
        )
    }
}

impl RelationLong {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<3} {:<4} {:^18} {:<4} {:^18} {:^28}",
            "ID", "ID A", "Alias List A", "ID B", "Alias List B", "Last modified"
        )?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<4} {:<4} {:^18} {:<4} {:^18} {:^28}",
            self.id,
            self.entity_id_a,
            textwrap::wrap(&self.alias_list_a, 18)[0],
            self.entity_id_b,
            textwrap::wrap(&self.alias_list_b, 18)[0],
            time::at(self.updated).rfc3339()
        )
    }
}

impl RelationSnippet {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<44} {:<6} {:^28}",
            "Snippets", "ID", "Last modified"
        )?;

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

impl EntityFound {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<20} {:<6} {:^10} {:^28}",
            "Name", "ID", "Entity ID", "Updated"
        )?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<20} {:<6} {:^10} {:^28}",
            self.name,
            self.id,
            self.entity_id,
            time::at(self.updated).rfc3339()
        )
    }
}

impl EntityFoundLong {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<10} {:<6} {:^10} {:^22} {:^28}",
            "Name", "ID", "Entity ID", "Other Alias", "Updated"
        )?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        writeln!(
            sink,
            " {:<10} {:<6} {:^10} {:^22} {:^28}",
            self.name,
            self.id,
            self.entity_id,
            self.other_alias,
            time::at(self.updated).rfc3339()
        )
    }
}

impl SnippetFound {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<36} {:<3} {:^10} {:^28}",
            "Snippets", "ID", "Entity ID", "Last modified"
        )?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) {
        let wrapped_data = textwrap::wrap(&self.data, 37);

        if wrapped_data.len() > 1 {
            writeln!(
                sink,
                " {:<36} {:<4} {:^10} {:^28}",
                wrapped_data[0],
                self.id,
                self.entity_id,
                time::at(self.updated).rfc3339()
            )
            .unwrap();

            for i in 1..wrapped_data.len() {
                writeln!(sink, " {:<36}", wrapped_data[i]).unwrap();
            }
        } else {
            writeln!(
                sink,
                " {:<36} {:<4} {:^10} {:^28}",
                self.data,
                self.id,
                self.entity_id,
                time::at(self.updated).rfc3339()
            )
            .unwrap();
        }
        writeln!(sink, "{}", "-".repeat(80)).unwrap();
    }
}

impl RelationSnippetFound {
    pub fn print_header<W: Write>(&self, sink: &mut W, row: &str) -> io::Result<()> {
        writeln!(
            sink,
            " {:<34} {:<3} {:^12} {:^28}",
            "Snippets", "ID", "Relation ID", "Last modified"
        )?;

        writeln!(sink, "{}", row)
    }

    pub fn print_content<W: Write>(&self, sink: &mut W) {
        let wrapped_data = textwrap::wrap(&self.data, 35);

        if wrapped_data.len() > 1 {
            writeln!(
                sink,
                " {:<34} {:<4} {:^12} {:^28}",
                wrapped_data[0],
                self.id,
                self.relation_id,
                time::at(self.updated).rfc3339()
            )
            .unwrap();

            for i in 1..wrapped_data.len() {
                writeln!(sink, " {:<34}", wrapped_data[i]).unwrap();
            }
        } else {
            writeln!(
                sink,
                " {:<34} {:<4} {:^12} {:^28}",
                self.data,
                self.id,
                self.relation_id,
                time::at(self.updated).rfc3339()
            )
            .unwrap();
        }
        writeln!(sink, "{}", "-".repeat(80)).unwrap();
    }
}
