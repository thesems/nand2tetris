pub struct Config {
    pub input: String,
    pub output: String,
}
impl Config {
    pub fn build(input: String, output: String) -> Config {
        Config { input, output }
    }
}
