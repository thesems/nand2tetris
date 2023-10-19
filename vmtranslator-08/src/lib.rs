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
        self.current_command = String::from(self.lines.index(self.current_line_idx).as_str())
            .split(" ")
            .map(|line| String::from(line.trim()))
            .collect();

        if self.current_command.is_empty() {
            panic!("current command cannot be empty!")
        }

        self.current_line_idx += 1;
    }

    pub fn command_type(&self) -> CommandType {
        return match self.current_command[0].as_str() {
            "push" => CommandType::CPush,
            "pop" => CommandType::CPop,
            "add" => CommandType::CArithmetic,
            "sub" => CommandType::CArithmetic,
            "neg" => CommandType::CArithmetic,
            "eq" => CommandType::CArithmetic,
            "gt" => CommandType::CArithmetic,
            "lt" => CommandType::CArithmetic,
            "and" => CommandType::CArithmetic,
            "or" => CommandType::CArithmetic,
            "not" => CommandType::CArithmetic,
            "label" => CommandType::CLabel,
            "if-goto" => CommandType::CIf,
            "goto" => CommandType::CGoto,
            "function" => CommandType::CFunction,
            "call" => CommandType::CCall,
            "return" => CommandType::CReturn,
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
            && command_type != CommandType::CLabel
            && command_type != CommandType::CIf
            && command_type != CommandType::CGoto
        {
            panic!("arg2 should not be called on operations other than: cpush, cpop, cfunction, ccall!")
        }
        let arg: u16 = self.current_command[2].parse().unwrap();
        return arg;
    }
}

pub struct CodeWriter {
    file: fs::File,
    current_line: u16,
}
impl CodeWriter {
    pub fn build(out_file: &str) -> CodeWriter {
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(out_file)
            .unwrap();

        return CodeWriter {
            file,
            current_line: 0,
        };
    }

    fn write_line(&mut self, line: &str) {
        self.file.write(line.as_bytes()).unwrap();
        self.current_line += 1;
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        // SP--
        self.write_line("@SP\n");
        self.write_line("M=M-1\n");

        // D = RAM[SP]
        self.write_line("@SP\n");
        self.write_line("A=M\n");
        self.write_line("D=M\n");

        if command == "neg" || command == "not" {
            let cmd = match command {
                "neg" => "-",
                "not" => "!",
                _ => panic!("not implemented"),
            };
            // RAM[SP] = -/! RAM[SP]
            self.write_line(format!("M={}M\n", cmd).as_str());
        } else {
            // SP--
            self.write_line("@SP\n");
            self.write_line("M=M-1\n");

            if command == "add" || command == "sub" {
                let cmd = match command {
                    "add" => "+",
                    "sub" => "-",
                    _ => panic!("not implemented"),
                };

                // RAM[SP] = RAM[SP] +/- D
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line(format!("M=M{}D\n", cmd).as_str());
            } else if command == "and" || command == "or" {
                let cmd = match command {
                    "and" => "&",
                    "or" => "|",
                    _ => panic!("not implemented"),
                };

                // RAM[SP] = RAM[SP] &/| D
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line(format!("M=M{}D\n", cmd).as_str());
            } else if command == "eq" || command == "gt" || command == "lt" {
                let cmd = match command {
                    "eq" => "JEQ",
                    "gt" => "JGT",
                    "lt" => "JLT",
                    _ => panic!("not implemented"),
                };

                // RAM[SP] = RAM[SP] - D
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line("D=M-D\n");

                // @line_true, RAM[SP];JEQ/JGT/JLT
                self.write_line(format!("@{}\n", self.current_line + 7).as_str());
                self.write_line(format!("D;{}\n", cmd).as_str());

                // RAM[SP] = 0, @line_skip_true, jump
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line("M=0\n");
                self.write_line(format!("@{}\n", self.current_line + 5).as_str());
                self.write_line("0;JMP\n");

                // RAM[SP] = -1
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line("M=-1\n");
            }
        }

        // SP++
        self.write_line("@SP\n");
        self.write_line("M=M+1\n");
    }

    pub fn write_push_pop(&mut self, command_type: CommandType, segment: &str, index: u16) {
        let segment_pointer = match segment {
            "constant" => "constant",
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            "temp" => "temp",
            "static" => "static",
            "pointer" => "pointer",
            _ => panic!("unknown segment label"),
        };
        if command_type == CommandType::CPush {
            if segment_pointer == "constant" {
                // D=i
                self.write_line(format!("@{}\n", index).as_str());
                self.write_line("D=A\n");

                // RAM[SP] = D
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line("M=D\n");
            } else {
                if segment_pointer == "temp"
                    || segment_pointer == "static"
                    || segment_pointer == "pointer"
                {
                    let segment = match segment_pointer {
                        "temp" => format!("{}", 5 + index),
                        "static" => format!("{}", 16 + index),
                        "pointer" => {
                            if index == 0 {
                                String::from("THIS")
                            } else {
                                String::from("THAT")
                            }
                        }
                        _ => panic!("should not happen!"),
                    };

                    // D <- temp/static + i
                    self.write_line(format!("@{}\n", segment).as_str());
                    self.write_line("D=M\n");
                } else {
                    // D <- segmentPointer + i
                    self.write_line(format!("@{}\n", index).as_str());
                    self.write_line("D=A\n");

                    // @segmentPointer
                    self.write_line(format!("@{}\n", segment_pointer).as_str());

                    // D <- M[segmentPointer + i]
                    self.write_line("A=M+D\n");
                    self.write_line("D=M\n");
                }

                // RAM[SP] <- RAM[D]
                self.write_line("@SP\n");
                self.write_line("A=M\n");
                self.write_line("M=D\n");
            }
            // SP++
            self.write_line("@SP\n");
            self.write_line("M=M+1\n");
        } else if command_type == CommandType::CPop {
            if segment_pointer == "temp"
                || segment_pointer == "static"
                || segment_pointer == "pointer"
            {
                // addr <- temp/static + i
                let segment = match segment_pointer {
                    "temp" => format!("{}", 5 + index),
                    "static" => format!("{}", 16 + index),
                    "pointer" => {
                        if index == 0 {
                            String::from("THIS")
                        } else {
                            String::from("THAT")
                        }
                    }
                    _ => panic!("should not happen!"),
                };

                // addr <- temp/static + i
                self.write_line(format!("@{}\n", segment).as_str());
                self.write_line("D=A\n");
                self.write_line("@13\n");
                self.write_line("M=D\n");
            } else {
                // addr <- segmentPointer + i
                self.write_line(format!("@{}\n", index).as_str());
                self.write_line("D=A\n");

                // @segmentPointer
                self.write_line(format!("@{}\n", segment_pointer).as_str());

                // addr <- M + D
                self.write_line("D=M+D\n");
                self.write_line("@13\n");
                self.write_line("M=D\n");
            }

            // SP--
            self.write_line("@SP\n");
            self.write_line("M=M-1\n");

            // RAM[addr] <- RAM[SP]
            self.write_line("@SP\n");
            self.write_line("A=M\n");
            self.write_line("D=M\n");
            self.write_line("@13\n");
            self.write_line("A=M\n");
            self.write_line("M=D\n");
        }
    }

    pub fn set_file_name(&self, file_name: &str) {}
    pub fn write_init(&self) {}

    pub fn write_label(&mut self, label: &str) {
        self.write_line(format!("({})\n", label).as_str());
    }

    pub fn write_goto(&mut self, label: &str) {
        self.write_line(format!("@{}\n", label).as_str());
        self.write_line("0;JMP\n");
    }

    pub fn write_if(&mut self, label: &str) {
        // SP--
        self.write_line("@SP\n");
        self.write_line("M=M-1\n");
        // D = RAM[SP]
        self.write_line("A=M\n");
        self.write_line("D=M\n");
        // if D != 0 then jump
        self.write_line(format!("@{}\n", label).as_str());
        self.write_line("D;JNE\n");
    }

    pub fn write_function(&mut self, function_name: &str, num_args: u16) {
        // TODO: fix me by adding a counter
        self.write_line(format!("({}${})\n", function_name, 0).as_str());

        let mut i = 0;
        while i < num_args {
            self.write_line("@SP\n");
            self.write_line("A=M\n");
            self.write_line("M=0\n");
            self.write_line("@SP\n");
            self.write_line("M=M-1\n");
            i += 1;
        }
    }

    pub fn write_call(&mut self, function_name: &str, num_args: u16) {
        // Define label for return address
        let label = ""; // TODO: randomly generate a variable name?

        // Push return address
        self.write_line("@SP\n");
        self.write_line("A=M\n");
        self.write_line("M=X\n"); // change X to return address

        // Increment stack
        self.write_line("@SP\n");
        self.write_line("M=M+1\n");

        // Push pointers
        let mut save_pointers = vec!["THAT", "THIS", "ARG", "LCL"];
        while let Some(pointer) = save_pointers.pop() {
            // Push LCL
            self.write_line(format!("@{}\n", pointer).as_str());
            self.write_line("D=M\n");
            self.write_line("@SP\n");
            self.write_line("M=D\n");

            // Increment stack
            self.write_line("@SP\n");
            self.write_line("M=M+1\n");
        }

        // Reposition ARG pointer
        // ARG = SP
        self.write_line("@SP\n");
        self.write_line("D=A\n");
        self.write_line("@ARG\n");
        self.write_line("M=D\n");

        // ARG -= 5
        self.write_line("@5\n");
        self.write_line("D=A\n");
        self.write_line("@ARG\n");
        self.write_line("M=M-D\n");

        // ARG -= num_args
        self.write_line(format!("@{}\n", num_args).as_str());
        self.write_line("D=A\n");
        self.write_line("@ARG\n");
        self.write_line("M=M-D\n");

        // LCL = SP
        self.write_line("@SP\n");
        self.write_line("D=A\n");
        self.write_line("@LCL\n");
        self.write_line("M=D\n");

        // Goto function
        self.write_line(format!("@{}\n", function_name).as_str());
        self.write_line("0;JMP\n");

        // Define label (e.g. Foo$ret.1)
        self.write_line(format!("({})\n", label).as_str());
    }

    pub fn write_return(&mut self) {
        // TODO: perhaps use @13 (general purpose registers) instead of @endFrame

        // endFrame = LCL
        self.write_line("@LCL\n");
        self.write_line("D=M\n");
        self.write_line("@endFrame\n");
        self.write_line("M=D\n");

        // retAddr = *(endFrame - 5)
        self.write_line("@5\n");
        self.write_line("D=A\n");
        self.write_line("@endFrame\n");
        self.write_line("D=A-D\n");
        self.write_line("M=A\n");
        self.write_line("D=M\n");
        self.write_line("@retAddr\n");
        self.write_line("M=D\n");

        // *ARG = pop()
        self.write_line("@SP\n");
        self.write_line("D=M-1\n");
        self.write_line("@ARG\n");
        self.write_line("A=M\n");
        self.write_line("M=D\n");

        // SP = ARG + 1
        self.write_line("@ARG\n");
        self.write_line("D=A\n");
        self.write_line("@SP\n");
        self.write_line("M=D+1\n");

        let pointers = vec!["THAT", "THIS", "ARG", "LCL"];
        let mut i = 1;
        while i < 5 {
            // SEGMENT = *(endFrame - 1)
            self.write_line(format!("@{}\n", i).as_str());
            self.write_line("D=A\n");
            self.write_line("@endFrame\n");
            self.write_line("D=A-D\n");
            self.write_line("M=A\n");
            self.write_line("D=M\n");
            self.write_line(format!("@{}\n", pointers[i-1]).as_str());
            self.write_line("M=D\n");
            i += 1
        }

        // Jump to return address
        self.write_line("A=D\n");
        self.write_line("0;JMP\n");
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::build(config.in_file.as_str()).unwrap();
    let mut code_writer = CodeWriter::build(config.out_file.as_str());

    // // RAM[SP] = 256
    // code_writer.write_line("@256\n");
    // code_writer.write_line("D=A\n");
    // code_writer.write_line("@0\n");
    // code_writer.write_line("M=D\n");
    //
    // // RAM[LCL] = 300
    // code_writer.write_line("@300\n");
    // code_writer.write_line("D=A\n");
    // code_writer.write_line("@1\n");
    // code_writer.write_line("M=D\n");
    //
    // // RAM[ARG] = 310
    // code_writer.write_line("@310\n");
    // code_writer.write_line("D=A\n");
    // code_writer.write_line("@2\n");
    // code_writer.write_line("M=D\n");

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
        } else if parser.command_type() == CommandType::CLabel {
            code_writer.write_label(parser.arg1().as_str());
        } else if parser.command_type() == CommandType::CIf {
            code_writer.write_if(parser.arg1().as_str());
        } else if parser.command_type() == CommandType::CGoto {
            code_writer.write_goto(parser.arg1().as_str());
        } else if parser.command_type() == CommandType::CFunction {
            code_writer.write_function(parser.arg1().as_str(), parser.arg2());
        } else if parser.command_type() == CommandType::CReturn {
            code_writer.write_return();
        } else if parser.command_type() == CommandType::CCall {
            code_writer.write_call(parser.arg1().as_str(), parser.arg2());
        } else {
            panic!("not implemented yet!");
        }
    }

    Ok(())
}
