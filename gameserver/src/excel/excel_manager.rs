use super::Item;

pub struct ExcelManager {
    pub item: Item
}

impl ExcelManager {
    pub fn new() -> Self {
        Self {
            item: Item::new()
        }
    }
}