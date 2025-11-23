use comfy_table::{Table, presets::UTF8_FULL};

pub fn create_table() -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table
}
