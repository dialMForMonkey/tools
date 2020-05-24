//implementar composer no get e set
pub trait BasicFunctions {
    fn new(host: &str) -> Self;
    fn set(&mut self, key: &str, value: &str, expiration: Option<u32>) -> Result<(), String>;
    fn get(&mut self, key: &str);
}
