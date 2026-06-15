use crate::config;
use once_cell::sync::OnceCell;

pub static UNDERSCORE_CONFIG: OnceCell<config::Config> = OnceCell::new();

pub fn init_globals() -> Result<(), std::io::Error>{
    UNDERSCORE_CONFIG.get_or_init(|| config::get_config_from_file());

    Ok(())
}