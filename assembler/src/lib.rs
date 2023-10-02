use std::{collections::HashMap, error::Error, fs, ops::Index, process};

pub struct Config {
    pub in_file: String,
    pub out_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not correct number of arguments!");
        }

        let in_file = args[1].clone();
        let out_file = args[2].clone();

        Ok(Config { in_file, out_file })
    }
}

#[derive(Debug)]
struct SymbolTable {
    pub symbols: HashMap<String, u16>,
    pub next_free_variable: u16,
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbols: HashMap<String, u16> = HashMap::new();
        for x in 0..16 {
            symbols.insert(format!("R{x}"), x);
        }
        symbols.insert(String::from("SCREEN"), 16384);
        symbols.insert(String::from("KBD"), 24576);
        symbols.insert(String::from("SP"), 0);
        symbols.insert(String::from("LCL"), 1);
        symbols.insert(String::from("ARG"), 2);
        symbols.insert(String::from("THIS"), 3);
        symbols.insert(String::from("THAT"), 4);
        // dbg!(symbols);

        SymbolTable {
            symbols: symbols,
            next_free_variable: 16,
        }
    }

    pub fn add_entry(&mut self, symbol: &String, address: u16) {
        self.symbols.insert(symbol.clone(), address);
    }

    pub fn contains(&self, symbol: &String) -> bool {
        return self.symbols.contains_key(symbol);
    }

    pub fn get_address(&self, symbol: &String) -> Option<&u16> {
        return self.symbols.get(symbol.as_str());
    }
}

#[derive(PartialEq)]
enum CommandType {
    ACommand = 1,
    CComand = 2,
    LCommand = 3,
}

struct Parser {
    lines: Vec<String>,
    current_line: i32,
    instr_line: i32,
    instructions: HashMap<i32, String>,
}
impl Parser {
    pub fn build(in_file: &str) -> Result<Parser, Box<dyn Error>> {
        let contents = fs::read_to_string(in_file)?;
        let lines = contents.split("\n");
        return Ok(Parser {
            lines: lines
                .into_iter()
                .map(|line| String::from(line.trim()))
                .filter(|line| !line.starts_with("//") && !line.is_empty())
                .collect(),
            current_line: -1,
            instr_line: -1,
            instructions: HashMap::new(),
        });
    }

    pub fn reset_to_start(&mut self) {
        self.current_line = -1;
        self.instr_line = -1;
    }

    pub fn command_type(&self) -> CommandType {
        let instr = self.instructions.get(&self.instr_line).unwrap();

        if instr.starts_with("@") {
            return CommandType::ACommand;
        } else if instr.contains(";") || instr.contains("=") {
            return CommandType::CComand;
        } else if instr.contains("(") {
            return CommandType::LCommand;
        }

        panic!("invalid instruction type {instr}");
    }

    pub fn has_more_commands(&self, first_pass: bool) -> bool {
        if (self.current_line + 1) as usize == self.lines.len() && first_pass {
            return false;
        } else if (self.instr_line + 1) as usize == self.instructions.len() && !first_pass {
            return false;
        }
        return true;
    }

    pub fn advance(&mut self, st: &mut SymbolTable, first_pass: bool) {
        if !self.has_more_commands(first_pass) {
            return;
        }

        self.current_line += 1;
        let mut line = self.lines.index(self.current_line as usize).trim();

        if line.starts_with("//") || line.is_empty() {
            self.advance(st, first_pass);
            return;
        }

        let binding = self.clean_line(line);
        line = binding.as_str();

        if self.instr_line == -1 {
            self.instr_line = 0;
        } else {
            self.instr_line += 1;
        }

        if first_pass {
            self.instructions
                .insert(self.instr_line, String::from(line));
        }

        let command_type = self.command_type();
        if first_pass && command_type == CommandType::LCommand {
            let symbol = self.symbol();
            if !st.contains(&symbol) {
                st.add_entry(&symbol, self.instr_line as u16);
            }
            self.instr_line -= 1;
        } else if !first_pass && command_type == CommandType::ACommand {
            let symbol = self.symbol();
            let mut is_num = true;
            let num = symbol.trim().parse::<u16>().unwrap_or_else(|_| {
                is_num = false;
                0
            });
            if is_num {
                st.add_entry(&symbol, num);
            } else if !st.contains(&symbol) {
                st.add_entry(&symbol, st.next_free_variable);
                st.next_free_variable += 1;
            }
        }
        // println!("{line}")
    }

    pub fn symbol(&self) -> String {
        let instr = self.instructions.get(&self.instr_line).unwrap();
        let size = instr.len();
        if instr.contains("@") {
            let value = &instr[1..size];
            return String::from(value.trim());
        } else if instr.contains("(") {
            let value = &instr[1..size - 1];
            return String::from(value.trim());
        }

        panic!("invalid instruction, not a symbol: {instr}")
    }

    pub fn dest(&self) -> String {
        let instr = self.instructions.get(&self.instr_line).unwrap();
        let tokens: Vec<&str> = instr.split("=").map(|val| val.trim()).collect();
        if tokens.len() != 1 {
            return String::from(tokens[0]);
        }
        return String::from("null");
    }

    pub fn comp(&self) -> String {
        let instr = self.instructions.get(&self.instr_line).unwrap();

        let tokens: Vec<&str>;
        if instr.contains(";") {
            // jump operation
            tokens = instr.split(";").map(|val| val.trim()).collect();
            return String::from(tokens[0]);
        } else {
            // assignment
            tokens = instr.split("=").map(|val| val.trim()).collect();
            return String::from(tokens[1]);
        }
    }

    pub fn jump(&self) -> String {
        let instr = self.instructions.get(&self.instr_line).unwrap();
        // jump operation
        let tokens: Vec<&str> = instr.split(";").map(|val| val.trim()).collect();

        if tokens.len() == 1 {
            // semi-colon not found
            return String::from("null");
        }

        return String::from(tokens[1]);
    }

    pub fn clean_line(&self, line: &str) -> String {
        let mut clean = line.trim();

        let idx = match clean.find("//") {
            Some(x) => x,
            None => 123456,
        };

        if idx != 123456 {
            clean = &clean[0..idx];
        }

        return String::from(clean);
    }
}

struct Code {}
impl Code {
    fn new() -> Code {
        return Code {};
    }
    pub fn comp(&self, mnemonic: &str) -> Result<String, Box<dyn Error>> {
        let comp = match mnemonic {
            "0" => "101010",
            "1" => "111111",
            "-1" => "111010",
            "D" => "001100",
            "A" => "110000",
            "M" => "110000",
            "!D" => "001101",
            "!A" => "110001",
            "!M" => "110001",
            "-D" => "001111",
            "-A" => "110011",
            "-M" => "110011",
            "D+1" => "011111",
            "A+1" => "110111",
            "M+1" => "110111",
            "D-1" => "001110",
            "A-1" => "110010",
            "M-1" => "110010",
            "D+A" => "000010",
            "D+M" => "000010",
            "D-A" => "010011",
            "D-M" => "010011",
            "A-D" => "000111",
            "M-D" => "000111",
            "D&A" => "000000",
            "D&M" => "000000",
            "D|A" => "010101",
            "D|M" => "010101",
            _ => "invalid",
        };

        if comp == "invalid" {
            return Err("invalid computation {mnemonic}".into());
        }

        let mut result = String::from(comp);
        if mnemonic.contains("M") {
            result.insert(0, '1');
        } else {
            result.insert(0, '0');
        }
        return Ok(result);
    }

    pub fn dest(&self, mnemonic: &str) -> Result<String, Box<dyn Error>> {
        let dest = match mnemonic {
            "null" => "000",
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => "invalid",
        };

        if dest == "invalid" {
            return Err(format!("invalid destination: {mnemonic}").into());
        }

        return Ok(String::from(dest));
    }

    fn jump(&self, mnemonic: &str) -> Result<String, Box<dyn Error>> {
        let jump = match mnemonic {
            "null" => "000",
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => "invalid",
        };
        if jump == "invalid" {
            return Err(format!("invalid jump: {mnemonic}").into());
        }
        return Ok(String::from(jump));
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::build(config.in_file.as_str()).unwrap();
    let mut st = SymbolTable::new();

    // Build symbol table
    loop {
        if !parser.has_more_commands(true) {
            break;
        }
        parser.advance(&mut st, true);
    }

    // Reset parser
    parser.reset_to_start();

    // Compile program
    let code = Code::new();
    let mut out_lines: Vec<String> = Vec::new();
    loop {
        if !parser.has_more_commands(false) {
            break;
        }
        parser.advance(&mut st, false);

        if parser.command_type() == CommandType::CComand {
            let comp = code.comp(&parser.comp()).unwrap();
            let dest = code.dest(&parser.dest()).unwrap();
            let jump = code.jump(&parser.jump()).unwrap();
            out_lines.push(format!("111{:#7}{:#3}{:#3}", comp, dest, jump));
        } else if parser.command_type() == CommandType::ACommand {
            let symbol = parser.symbol();
            let addr = st.get_address(&symbol).unwrap_or_else(|| {
                println!("Error: undefined variable {symbol}");
                process::exit(1);
            });
            out_lines.push(format!("{addr:016b}"));
        } else if parser.command_type() == CommandType::LCommand {
            continue;
        }
    }

    // Write .hack file
    fs::write(
        config.out_file,
        out_lines
            .iter()
            .map(|line| format!("{}\n", line))
            .collect::<Vec<String>>()
            .concat(),
    )?;

    Ok(())
}
