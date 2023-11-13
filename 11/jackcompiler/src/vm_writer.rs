use std::io::Write;
use std::{error::Error, fs};

pub enum Segment {
    CONSTANT,
    ARGUMENT,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
}

pub enum Operation {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
    // Special
    MULT,
    DIV,
}

pub struct VmWriter {
    file: fs::File,
}

impl VmWriter {
    pub fn build(out_path: &str) -> Result<VmWriter, Box<dyn Error>> {
        let file = std::fs::File::options()
            .create(true)
            .append(false)
            .write(true)
            .open(out_path)?;
        Ok(VmWriter { file })
    }

    pub fn write_push(&mut self, segment: Segment, index: u16) {
        let segment_str = match segment {
            Segment::CONSTANT => "constant",
            Segment::ARGUMENT => "argument",
            Segment::LOCAL => "local",
            Segment::STATIC => "static",
            Segment::THIS => "this",
            Segment::THAT => "that",
            Segment::POINTER => "pointer",
            Segment::TEMP => "temp",
        };
        write!(self.file, "push {} {}\n", segment_str, index).unwrap();
    }

    pub fn write_pop(&mut self, segment: Segment, index: u16) {
        let segment_str = match segment {
            Segment::CONSTANT => "constant",
            Segment::ARGUMENT => "argument",
            Segment::LOCAL => "local",
            Segment::STATIC => "static",
            Segment::THIS => "this",
            Segment::THAT => "that",
            Segment::POINTER => "pointer",
            Segment::TEMP => "temp",
        };

        write!(self.file, "pop {} {}\n", segment_str, index).unwrap();
    }

    pub fn write_arithmetic(&mut self, op: Operation) {
        let op_str = match op {
            Operation::ADD => "add",
            Operation::SUB => "sub",
            Operation::NEG => "neg",
            Operation::EQ => "eq",
            Operation::GT => "gt",
            Operation::LT => "lt",
            Operation::AND => "and",
            Operation::OR => "or",
            Operation::NOT => "not",
            Operation::MULT => {
                self.write_call("Math.multiply", 2);
                ""
            }
            Operation::DIV => {
                self.write_call("Math.divide", 2);
                ""
            }
        };

        if op_str != "" {
            write!(self.file, "{}\n", op_str).unwrap();
        }
    }

    pub fn write_label(&mut self, label: &str) {
        write!(self.file, "label {}\n", label).unwrap();
    }

    pub fn write_goto(&mut self, label: &str) {
        write!(self.file, "goto {}\n", label).unwrap();
    }

    pub fn write_if(&mut self, label: &str) {
        write!(self.file, "if-goto {}\n", label).unwrap();
    }

    pub fn write_call(&mut self, name: &str, nargs: u16) {
        write!(self.file, "call {} {}\n", name, nargs).unwrap();
    }

    pub fn write_function(&mut self, name: &str, nvars: u16) {
        write!(self.file, "function {} {}\n", name, nvars).unwrap();
    }

    pub fn write_return(&mut self) {
        write!(self.file, "return\n").unwrap();
    }
}
