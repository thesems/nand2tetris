use std::{fs, error::Error};

enum Segment {
    UNKNOWN,
    CONSTANT,
    ARGUMENT,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
}

enum Operation {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

struct VmWriter {
    file: fs::File
}

impl VmWriter {
    pub fn build(out_path: &str) -> Result<VmWriter, Box<dyn Error>> {
        let file = std::fs::File::options().create(true).append(false).write(true).open(out_path)?;
        Ok(VmWriter { file })
    }

    pub fn write_push(segment: Segment, index: u16) {}
    pub fn write_pop(segment: Segment, index: u16) {}
    pub fn write_arithmetic(op: Operation) {}
    pub fn write_label(label: &str) {}
    pub fn write_goto(label: &str) {}
    pub fn write_if(label: &str) {}
    pub fn write_call(name: &str, nargs: u16) {}
    pub fn write_function(name: &str, nvars: u16) {}
    pub fn write_return() {}
}
