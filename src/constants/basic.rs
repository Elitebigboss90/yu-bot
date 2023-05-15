use std::env;

lazy_static! {
    pub static ref BASE_API_URL: String = env::var("BASE_API_URL").unwrap();
    pub static ref TOKEN: String = env::var("TOKEN").unwrap();
    pub static ref VERIFY_TOKEN: String = env::var("VERIFY_TOKEN").unwrap();
    pub static ref ENCRYPT_KEY: String = env::var("ENCRYPT_KEY").unwrap();
    pub static ref BOT_ID: String = env::var("BOT_ID").unwrap();
}