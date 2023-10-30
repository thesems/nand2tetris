pub struct Config {
    pub input: String,
    pub output: String,
    pub output_xml_tokenizer: String,
}
impl Config {
    pub fn build(input: String, output: String, output_xml_tokenizer: String) -> Config {
        Config { input, output, output_xml_tokenizer }
    }
}
