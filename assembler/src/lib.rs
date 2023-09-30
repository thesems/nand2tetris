use std::{error::Error, fs, str::FromStr};

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.in_file)?;
    let lines = contents.split("\n");

    let mut out_lines: Vec<String> = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        } else if line.starts_with("//") {
            continue;
        }

        println!("{line}");

        if line.starts_with("@") {
            // A-Instruction
            out_lines.push(handle_instruction_a(line.to_string().clone())?);
        } else if line.contains("(") {
            // Label
        } else {
            // C-Instruction
            out_lines.push(handle_instruction_c(line.to_string().clone())?);
        }
    }

    fs::write(
        config.out_file,
        out_lines
            .iter()
            .map(|line| format!("{}\n", line))
            .collect::<Vec<String>>()
            .concat(),
    )?;

    println!("Output hack binary:");
    for line in out_lines {
        println!("{line}")
    }

    Ok(())
}

fn handle_instruction_a(instr: String) -> Result<String, Box<dyn Error>> {
    let size = instr.len();
    let value = &instr[1..size];
    let mut variable = "";

    let mut is_num = true;
    let num = value.trim().parse::<u16>().unwrap_or_else(|_| {
        variable = value;
        is_num = false;
        0
    });

    if !is_num {
        // variable
        println!("Variable: {num}!");
    } else {
        // integer value
        return Ok(format!("{num:016b}"));
    }

    // if number => binary
    // else => save as variable
    return Ok(String::from_str(value)?);
}

fn handle_instruction_c(instr: String) -> Result<String, Box<dyn Error>> {
    if instr.contains(";") {
        // jump operation
        Ok(String::from("to-do jump"))
    } else {
        // assignment operation
        let tokens: Vec<&str> = instr.split("=").map(|val| val.trim()).collect();

        let dest = match tokens[0] {
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
            return Err("invalid destination".into());
        }

        let comp = match tokens[1] {
            "0" => "101010",
            "1" => "111111",
            "-1" => "111010",
            "D" => "001100",
            "A" => "110000",
            "!D" => "001101",
            "!A" => "110001",
            "-D" => "001111",
            "-A" => "110011",
            "D+1" => "011111",
            "A+1" => "110111",
            "D-1" => "001110",
            "A-1" => "110010",
            "D+A" => "000010",
            "D-A" => "010011",
            "A-D" => "000111",
            "D&A" => "000000",
            "D|A" => "010101",
            _ => "invalid",
        };

        if comp == "invalid" {
            dbg!(tokens);
            return Err("invalid computation".into());
        }

        let out = format!("1110{:#6}{:#3}000", comp, dest);
        Ok(String::from(out))
    }
}

fn handle_label() {}
