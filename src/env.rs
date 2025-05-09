use dotenv::dotenv;

use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn read_env(key: &str) -> String {
    INIT.call_once(|| {
        dotenv().ok();
    });

    let value = env::var(key).unwrap();
    return value;
}
