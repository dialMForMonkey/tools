
use super::traits::basic_functions;
use memcache;

struct BenckmarkMemcached;

impl basic_functions for BenckmarkMemcached {
    fn new(host: &str) -> Self {
        unimplemented!()
    }

    fn set(self, key: &str, value: &str) -> Self {
        unimplemented!()
    }

    fn get(self, key: &str) -> Self {
        unimplemented!()
    }
}