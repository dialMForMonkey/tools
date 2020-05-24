

pub trait basic_functions {
    fn new(host: &str) -> Self;
    fn set(self, key: &str, value: &str) -> Self;
    fn get(self, key: &str) -> Self;
}
