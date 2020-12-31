use std::collections::HashMap;

pub struct SymbolTable {
    map: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: usize) {
        self.map.insert(symbol, address);
    }

    pub fn contains(&self, symbol: String) -> bool {
        self.map.contains_key(&symbol)
    }

    pub fn get_address(&self, symbol: String) -> usize {
        *self.map.get(&symbol).unwrap()
    }
}
