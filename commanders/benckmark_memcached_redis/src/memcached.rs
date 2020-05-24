#[cfg(test)]
mod test {
    use crate::memcached::BenckmarkMemcached;
    use crate::traits::BasicFunctions;
    use std::time::Duration;

    #[test]
    fn conn_ok() {
        BenckmarkMemcached::new(&"memcache://127.0.0.1:11211");
    }

    #[test]
    fn set_ok() {
        let mut conn = BenckmarkMemcached::new(&"memcache://127.0.0.1:11211");
        let r = conn.set(&"mario", &"aquele q ....", None);
        assert!(r.is_ok())
    }

    #[test]
    fn set_nok() {
        let mut conn = BenckmarkMemcached::new(&"memcache://127.0.0.1:11211");
        //simular queda no servico
        println!("pausando ...");
        std::thread::sleep(Duration::from_secs(10));
        println!("... retornando");
        let r = conn.set(&"mario", &"aquele q ....", None);
        assert!(r.is_err())
    }
}

use super::traits::BasicFunctions;
use memcache;
use memcache::Client;

struct BenckmarkMemcached {
    conn: Client,
}

impl BasicFunctions for BenckmarkMemcached {
    fn new(host: &str) -> Self {
        let instancy_memcache = BenckmarkMemcached {
            conn: memcache::Client::connect(host).expect("Erro ao recuperar conexao do memcached"),
        };

        instancy_memcache
    }

    fn set(&mut self, key: &str, value: &str, expiration: Option<u32>) -> Result<(), String> {
        let result = self.conn.set(key, value, expiration.unwrap_or(0u32));
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Erro ao setar valor {:?}", err)),
        }
    }

    fn get(&mut self, key: &str) {
        unimplemented!()
    }
}
