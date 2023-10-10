use core::panic;
use std::{error::Error, fs, io::Write, ops::Index};

#[derive(PartialEq)]
pub enum CommandType {
    CArithmetic = 1,
    CPush = 2,
    CPop = 3,
    CLabel = 4,
    CGoto = 5,
    CIf = 6,
    CFunction = 7,
    CReturn = 8,
    CCall = 9,
}

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

pub struct Parser {
    lines: Vec<String>,
    current_command: Vec<String>,
    current_line_idx: usize,
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
            current_command: Vec::new(),
            current_line_idx: 0,
        });
    }

    pub fn has_more_lines(&self) -> bool {
        return self.current_line_idx < self.lines.len();
    }

    pub fn advance(&mut self) {
        self.current_command =
            String::from(self.lines.index(self.current_line_idx).as_str())
                .split(" ")
                .map(|line| String::from(line))
                .collect();

        if self.current_command.is_empty() {
            panic!("current command cannot be empty!")
        }

        println!("{:?}", self.current_command);
        self.current_line_idx += 1;
    }

    pub fn command_type(&self) -> CommandType {
        return match self.current_command[0].as_str() {
            "push" => CommandType::CPush,
            "pop" => CommandType::CPop,
            "add" => CommandType::CArithmetic,
            _ => panic!("invalid command type"),
        };
    }

    pub fn arg1(&self) -> String {
        let command_type = self.command_type();
        if command_type == CommandType::CArithmetic {
            return self.current_command[0].clone();
        } else if command_type == CommandType::CReturn {
            panic!("arg1() should not be called for command type creturn!")
        }
        return self.current_command[1].clone();
    }
    pub fn arg2(&self) -> u16 {
        let command_type = self.command_type();
        if command_type != CommandType::CPush
            && command_type != CommandType::CPop
            && command_type != CommandType::CFunction
            && command_type != CommandType::CCall
        {
            panic!("arg2 should not be called on operations other than: cpush, cpop, cfunction, ccall!")
        }

        let arg: u16 = self.current_command[2].parse().unwrap();
        return arg;
    }
}

pub struct CodeWriter {
    file: fs::File,
}
impl CodeWriter {
    pub fn build(out_file: &str) -> CodeWriter {
        let file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(out_file)
            .unwrap();

        return CodeWriter { file };
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        if command == "add" {
            // SP--
            write!(self.file, "@SP\nM=M-1\n").unwrap();
            // D = RAM[SP]
            write!(self.file, "@SP\nA=M\nD=M\n").unwrap();
            // SP--
            write!(self.file, "@SP\nM=M-1\n").unwrap();
            // RAM[SP] = RAM[SP] + D
            write!(self.file, "@SP\nA=M\nM=M+D\n").unwrap();
        } else if command == "sub" {
        } else {
        }
    }

    pub fn write_push_pop(&mut self, command_type: CommandType, segment: &str, index: u16) {
        let segment_pointer = match segment {
            "constant" => "constant",
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => panic!("unknown segment label"),
        };
        if command_type == CommandType::CPush {
            if segment_pointer == "constant" {
                // D=i
                write!(self.file, "@{}\nD=A\n", index).unwrap();
                // RAM[SP] = D
                write!(self.file, "@SP\nA=M\nM=D\n").unwrap();
            } else {
                // addr <- segment + i
                write!(
                    self.file,
                    "@{}\nA=M+{}\nD=A\n@addr\nM=D\n",
                    segment_pointer, index
                )
                .unwrap();
                // RAM[SP] <- RAM[addr]
                write!(self.file, "@SP\nA=M\n@addr\nM=D\n").unwrap();
            }
            // SP++
            write!(self.file, "@SP\nM=M+1\n").unwrap();
        } else if command_type == CommandType::CPop {
            // addr <- segment + i
            write!(
                self.file,
                "@{}\nA=M+{}\nD=A\n@addr\nM=D\n",
                segment_pointer, index
            )
            .unwrap();
            // SP--
            write!(self.file, "@SP\nM=M-1\n").unwrap();
            // RAM[addr] <- RAM[SP]
            write!(self.file, "@SP\nD=M\n@addr\nM=D\n").unwrap();
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::build(config.in_file.as_str()).unwrap();
    let mut code_writer = CodeWriter::build(config.out_file.as_str());

    loop {
        if !parser.has_more_lines() {
            break;
        }
        parser.advance();

        if parser.command_type() == CommandType::CPush || parser.command_type() == CommandType::CPop
        {
            code_writer.write_push_pop(parser.command_type(), &parser.arg1(), parser.arg2());
        } else if parser.command_type() == CommandType::CArithmetic {
            code_writer.write_arithmetic(parser.arg1().as_str());
        }
    }

    Ok(())
}
