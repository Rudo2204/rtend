use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use time::{Format, OffsetDateTime};

use crate::utils;

pub struct Entity {
    pub id: u32,
    pub created: OffsetDateTime,
}

pub struct EntityLong {
    pub id: u32,
    pub alias_list: String,
    pub alias_count: u32,
    pub snippet_count: u32,
    pub created: OffsetDateTime,
}

pub struct EntityLongLong {
    pub id: u32,
    pub data_type: String,
    pub data: String,
    pub last_modified: OffsetDateTime,
}

pub struct Alias {
    pub id: u32,
    pub name: String,
    pub updated: OffsetDateTime,
}

pub struct Snippet {
    pub id: u32,
    pub data: String,
    pub updated: OffsetDateTime,
}

pub struct Relation {
    pub id: u32,
    pub entity_id_a: u32,
    pub entity_id_b: u32,
    pub updated: OffsetDateTime,
}

pub struct RelationLong {
    pub id: u32,
    pub entity_id_a: u32,
    pub alias_list_a: String,
    pub entity_id_b: u32,
    pub alias_list_b: String,
    pub updated: OffsetDateTime,
}

pub struct RelationSnippet {
    pub id: u32,
    pub data: String,
    pub updated: OffsetDateTime,
}

pub struct EntityFound {
    pub id: u32,
    pub name: String,
    pub entity_id: u32,
    pub updated: OffsetDateTime,
}

pub struct EntityFoundLong {
    pub id: u32,
    pub name: String,
    pub entity_id: u32,
    pub other_alias: String,
    pub updated: OffsetDateTime,
}

pub struct SnippetFound {
    pub id: u32,
    pub data: String,
    pub entity_id: u32,
    pub updated: OffsetDateTime,
}

pub struct RelationSnippetFound {
    pub id: u32,
    pub data: String,
    pub relation_id: u32,
    pub updated: OffsetDateTime,
}

pub struct Stats {
    pub stat_type: String,
    pub count: u32,
}

pub struct ComfyStruct<T> {
    pub data: Vec<T>,
}

pub trait ComfyTable {
    fn print_comfy_table(&self);
}

impl ComfyTable for ComfyStruct<Entity> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![Cell::new("ID"), Cell::new("Created on")]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.id),
                    Cell::new(&entity.created.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<EntityLong> {
    fn print_comfy_table(&self) {
        let mut table = Table::new();
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("ID"),
                    Cell::new("Alias List"),
                    Cell::new("Aliases"),
                    Cell::new("Snippets"),
                    Cell::new("Created on"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.id),
                    Cell::new(&entity.alias_list),
                    Cell::new(&entity.alias_count),
                    Cell::new(&entity.snippet_count),
                    Cell::new(&entity.created.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<EntityLongLong> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("ID"),
                    Cell::new("Type"),
                    Cell::new("Data"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.id),
                    Cell::new(&entity.data_type),
                    Cell::new(&entity.data),
                    Cell::new(&entity.last_modified.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<Alias> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Names"),
                    Cell::new("ID"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.name),
                    Cell::new(&entity.id),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<Snippet> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Snippets"),
                    Cell::new("ID"),
                    Cell::new("Last modified"),
                ]);

            for snippet in &self.data {
                table.add_row(vec![
                    Cell::new(&snippet.data),
                    Cell::new(&snippet.id),
                    Cell::new(&snippet.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<Relation> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("ID"),
                    Cell::new("Entity ID A"),
                    Cell::new("Entity ID B"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.id),
                    Cell::new(&entity.entity_id_a),
                    Cell::new(&entity.entity_id_b),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<RelationLong> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("ID"),
                    Cell::new("ID A"),
                    Cell::new("Alias List A"),
                    Cell::new("ID B"),
                    Cell::new("Alias List B"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.id),
                    Cell::new(&entity.entity_id_a),
                    Cell::new(&entity.alias_list_a),
                    Cell::new(&entity.entity_id_b),
                    Cell::new(&entity.alias_list_b),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<RelationSnippet> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Snippets"),
                    Cell::new("ID"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.data),
                    Cell::new(&entity.id),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<EntityFound> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Name"),
                    Cell::new("ID"),
                    Cell::new("Entity ID"),
                    Cell::new("Updated"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.name),
                    Cell::new(&entity.id),
                    Cell::new(&entity.entity_id),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<EntityFoundLong> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Name"),
                    Cell::new("ID"),
                    Cell::new("Entity ID"),
                    Cell::new("Other Aliases"),
                    Cell::new("Updated"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.name),
                    Cell::new(&entity.id),
                    Cell::new(&entity.entity_id),
                    Cell::new(&entity.other_alias),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<SnippetFound> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Snippets"),
                    Cell::new("ID"),
                    Cell::new("Entity ID"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.data),
                    Cell::new(&entity.id),
                    Cell::new(&entity.entity_id),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<RelationSnippetFound> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![
                    Cell::new("Snippets"),
                    Cell::new("ID"),
                    Cell::new("Relation ID"),
                    Cell::new("Last modified"),
                ]);

            for entity in &self.data {
                table.add_row(vec![
                    Cell::new(&entity.data),
                    Cell::new(&entity.id),
                    Cell::new(&entity.relation_id),
                    Cell::new(&entity.updated.format(Format::Rfc3339)),
                ]);
            }

            println!("{}", table);
        }
    }
}

impl ComfyTable for ComfyStruct<Stats> {
    fn print_comfy_table(&self) {
        if self.data.len() == 0 {
            println!("Found nothing.");
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_table_width(utils::get_term_width())
                .set_header(vec![Cell::new("Type"), Cell::new("Count")]);

            for entity in &self.data {
                table.add_row(vec![Cell::new(&entity.stat_type), Cell::new(&entity.count)]);
            }

            println!("{}", table);
        }
    }
}
