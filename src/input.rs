pub struct Input<T> {
    pub raw: String,
    pub key: T,
}

impl<T> Input<T> {

    pub fn new(key: T, raw_input: String) -> Input<T> {
        Input {
            key: key,
            raw: raw_input,
        }
    }

    pub fn args(&self) -> Vec<&str> {
        self.raw.trim().split_whitespace().collect()
    }

}