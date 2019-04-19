pub struct Command<T> {
    pub key: T,
    pub pattern: regex::Regex,
    pub help: String,
}

impl<T> Command<T> {

    pub fn new(key: T, pattern: &str, help: String) -> Command<T> {
        Command {
            key: key,
            pattern: regex::Regex::new(pattern).unwrap(),
            help: help,
        }
    }

}
