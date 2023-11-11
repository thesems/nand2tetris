use std::collections::HashMap;

#[derive(PartialEq, Clone)]
enum KindType {
    STATIC,
    FIELD,
    ARG,
    VAR,
}

struct SymbolTable {
    vars: HashMap<String, (String, KindType, u16)>,
}

impl SymbolTable {
    fn build() -> SymbolTable {
        return SymbolTable{
            vars: HashMap::new(),
        };
    }
    fn start_subroutine(&mut self) {
        self.vars.clear();
    }
    fn define(&mut self, name: &str, typ: &str, kind: KindType) {
        let index: u16 = self.var_count(&kind);
        self.vars.insert(String::from(name), (String::from(typ), kind, index)); 
    }
    fn var_count(&self, kind: &KindType) -> u16 {
        let mut size: u16 = 0;
        for (_, data) in &self.vars {
            if data.1 == *kind {
                size = size + 1;
            }
        }
        return size;
    }
    fn kind_of(&self, name: &str) -> KindType {
        let data = self.vars.get(name).unwrap_or_else(|| {
            panic!("Could not find the symbol {}.", name);
        });
        return data.1.clone();
    }
    fn type_of(&self, name: &str) -> &str {
        let data = self.vars.get(name).unwrap_or_else(|| {
            panic!("Could not find the symbol {}.", name);
        });
        return data.0.as_str();
    }
    fn index_of(&self, name: &str) -> u16 {
        let data = self.vars.get(name).unwrap_or_else(|| {
            panic!("Could not find the symbol {}.", name);
        });
        return data.2;
    }
}
