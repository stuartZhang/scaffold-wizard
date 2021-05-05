pub mod error;

use ::std::{env, error::Error as StdError, path::PathBuf};
use ::log::debug;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

pub fn get_assets_dir() -> Result<PathBuf> {
    let mut json_file_path = env::current_exe()?;
    json_file_path.pop();
    json_file_path.push("assets");
    Ok(json_file_path)
}
pub fn initialize_log4rs() -> Result<()> {
    let mut log4rs_file = get_assets_dir()?;
    log4rs_file.push("log4rs.json");
    log4rs::init_file(log4rs_file, Default::default())?;
    debug!("初始化 log4rs 成功");
    Ok(())
}
