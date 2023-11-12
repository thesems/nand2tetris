use std::{collections::HashMap, fmt};

#[derive(PartialEq, Clone)]
pub enum KindType {
    STATIC,
    FIELD,
    ARG,
    VAR,
}

impl fmt::Display for KindType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           KindType::STATIC => write!(f, "static"),
           KindType::FIELD => write!(f, "field"),
           KindType::ARG => write!(f, "arg"),
           KindType::VAR => write!(f, "var"),
       }
    }
}

pub struct SymbolTable {
    vars: HashMap<String, (String, KindType, u16)>,
}

impl SymbolTable {
    pub fn build() -> SymbolTable {
        return SymbolTable{
            vars: HashMap::new(),
        };
    }
    
    pub fn start_subroutine(&mut self) {
        self.vars.clear();
    }
    
    pub fn define(&mut self, name: &str, typ: &str, kind: KindType) {
        let index: u16 = self.var_count(&kind);
        self.vars.insert(String::from(name), (String::from(typ), kind, index)); 
    }
    
    pub fn var_count(&self, kind: &KindType) -> u16 {
        let mut size: u16 = 0;
        for (_, data) in &self.vars {
            if data.1 == *kind {
                size = size + 1;
            }
        }
        return size;
    }

    pub fn kind_of(&self, name: &str) -> Option<KindType> {
        return match self.vars.get(name) {
            Some(data) => Some(data.1.clone()),
            None => None
        }
    }

    pub fn type_of(&self, name: &str) -> Option<&str> {
        return match self.vars.get(name) {
            Some(data) => Some(data.0.as_str()),
            None => None
        }
    }
    
    pub fn index_of(&self, name: &str) -> Option<u16> {
        return match self.vars.get(name) {
            Some(data) => Some(data.2),
            None => None
        }
    }
}
