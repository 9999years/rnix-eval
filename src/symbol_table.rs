use std::collections::HashSet;

pub type Symbol<'arena> = &'arena str;

pub struct SymbolTable(HashSet<String>);

impl SymbolTable {
    pub fn create(&mut self, s: String) -> Symbol<'_> {
        self.0.insert(s);
        self.0.get(&s).unwrap()
    }
}
