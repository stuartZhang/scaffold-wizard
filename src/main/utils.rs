pub mod error;

use ::std::{env, error::Error as StdError, path::{Path, PathBuf}};
use ::log::debug;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

pub fn find_input_file(input_file: &str) -> Result<(PathBuf, PathBuf)> {
    let input_file_path = Path::new(input_file);
    if input_file_path.is_absolute() { // 绝对路径
        if input_file_path.exists() && input_file_path.is_file() {
            let input_file = input_file_path.to_path_buf();
            let mut output_dir = input_file_path.to_path_buf();
            output_dir.pop();
            return Ok((input_file, output_dir));
        }
        return Err(Box::new(Error::new("【问卷】输入不是一个文件", None)));
    }
    // 相对路径，先找 exe 文件所在的目录
    let mut input_file_path = env::current_exe()?;
    input_file_path.pop();
    input_file_path.push(input_file);
    if input_file_path.exists() && input_file_path.is_file() {
        let mut output_dir = env::current_exe()?;
        output_dir.pop();
        return Ok((input_file_path, output_dir));
    }
    // 相对路径，再找命令执行的工作目录
    let mut input_file_path = env::current_dir()?;
    input_file_path.push(input_file);
    if input_file_path.exists() && input_file_path.is_file() {
        return Ok((input_file_path, env::current_dir()?));
    }
    Err(Box::new(Error::new("【问卷】输入不是一个文件", None)))
}
pub fn find_output_file(output_file: &str, output_dir: &PathBuf) -> PathBuf {
    let output_file_path = Path::new(output_file);
    if output_file_path.is_absolute() {
        return output_file_path.to_path_buf();
    }
    let mut output_dir = output_dir.clone();
    output_dir.push(output_file);
    output_dir
}
pub fn initialize_log4rs() -> Result<()> {
    let (log4rs_file, _) = find_input_file("assets/log4rs.json")?;
    log4rs::init_file(log4rs_file, Default::default())?;
    debug!("初始化 log4rs 成功");
    Ok(())
}
