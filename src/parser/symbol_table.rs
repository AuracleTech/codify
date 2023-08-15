use super::kinds::Lexemes;

pub struct SymbolTable {
    pub table: Vec<SymbolEntry>,
}

pub struct SymbolEntry {
    pub name: String,
    pub kind: Lexemes,
    pub byte_size: u64,
    pub dimensions: u64,

    pub line_declaration: Option<u32>,
    pub line_last_use: Option<u32>,
    pub line_address: Option<u32>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { table: Vec::new() }
    }

    //     pub async fn insert(&mut self, entry: SymbolEntry) {
    //         self.table.push(entry);
    //     }

    //     pub async fn lookup(&self, name: &str) -> Option<&SymbolEntry> {
    //         self.table.iter().find(|&x| x.name == name)
    //     }
}
