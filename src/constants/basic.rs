use std::env;
use std::process;

lazy_static! {
    pub static ref BASE_API_URL: String = get_env("BASE_API_URL");
    pub static ref TOKEN: String = get_env("TOKEN");
    pub static ref VERIFY_TOKEN: String = get_env("VERIFY_TOKEN");
    pub static ref ENCRYPT_KEY: String = get_env("ENCRYPT_KEY");
    pub static ref BOT_ID: String = get_env("BOT_ID");
}

fn get_env(var: &str) -> String {
    match env::var(var) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{} environment variable not set", var);
            process::exit(1);
        }
    }
}