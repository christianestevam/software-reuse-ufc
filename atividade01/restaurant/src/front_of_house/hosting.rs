use std::collections::HashMap;

pub struct Table {
    pub id: u32,
    pub is_occupied: bool,
}

pub struct Host {
    pub tables: HashMap<u32, Table>,
}

impl Host {
    pub fn new() -> Self {
        let mut tables = HashMap::new();
        for id in 1..=10 {
            tables.insert(id, Table { id, is_occupied: false });
        }
        Self { tables }
    }

    pub fn seat_table(&mut self, table_id: u32) -> Result<(), String> {
        match self.tables.get_mut(&table_id) {
            Some(table) if !table.is_occupied => {
                table.is_occupied = true;
                println!("Table {} is now occupied.", table_id);
                Ok(())
            }
            Some(_) => Err(format!("Table {} is already occupied.", table_id)),
            None => Err(format!("Table {} does not exist.", table_id)),
        }
    }

    pub fn free_table(&mut self, table_id: u32) -> Result<(), String> {
        match self.tables.get_mut(&table_id) {
            Some(table) if table.is_occupied => {
                table.is_occupied = false;
                println!("Table {} is now free.", table_id);
                Ok(())
            }
            Some(_) => Err(format!("Table {} is already free.", table_id)),
            None => Err(format!("Table {} does not exist.", table_id)),
        }
    }
}
