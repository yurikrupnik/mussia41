use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Singleton>> = Arc::new(Mutex::new(Singleton::new()));
}

struct Singleton {
    value: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton { value: 0 }
    }

    fn get_instance() -> Arc<Mutex<Singleton>> {
        Arc::clone(&INSTANCE)
    }
}
