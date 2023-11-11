pub struct Config {
    pub input: String,
}
impl Config {
    pub fn build(input: String) -> Config {
        Config { input }
    }
}
