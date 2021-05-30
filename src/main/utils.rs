pub mod error;

use ::std::{env, error::Error as StdError, fmt::{Debug, Display}, path::{Path, PathBuf}, rc::Rc, result::Result as StdResult};
use ::log::{debug, error};
use ::gtk::Builder;
use super::render;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

pub fn get_exe_dir() -> Result<PathBuf> {
    let mut input_file_path = env::current_exe()?;
    input_file_path.pop();
    Ok(input_file_path)
}
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
    let mut input_file_path = get_exe_dir()?;
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
#[allow(dead_code)]
pub fn find_output_file(output_file: &str, output_dir: &PathBuf) -> PathBuf {
    let output_file_path = Path::new(output_file);
    if output_file_path.is_absolute() {
        return output_file_path.to_path_buf();
    }
    let mut output_dir = output_dir.clone();
    output_dir.push(output_file);
    output_dir
}
pub fn initialize_log4rs(log4rs_file: Option<&str>, bin_dir: Option<&Path>) -> Result<()> {
    let log4rs_file = log4rs_file.unwrap_or("../assets/log4rs.json");
    let log4rs_file = if let Some(bin_dir) = bin_dir {
        let mut bin_dir = bin_dir.to_path_buf();
        bin_dir.push(Path::new(log4rs_file));
        if bin_dir.exists() && bin_dir.is_file() {
            Some(bin_dir)
        } else {
            None
        }
    } else if let Ok((log4rs_file, _)) = find_input_file(log4rs_file) {
        Some(log4rs_file)
    } else {
        None
    };
    if let Some(log4rs_file) = log4rs_file {
        let home_dir = if let Some(bin_dir) = bin_dir {
            let mut bin_dir = bin_dir.to_path_buf();
            bin_dir.pop();
            Some(bin_dir)
        } else if let Ok(mut bin_dir) = get_exe_dir() {
            bin_dir.pop();
            Some(bin_dir)
        } else {
            None
        };
        if let Some(home_dir) = home_dir {
            env::set_var("INSTALL_HOME_DIR", home_dir.into_os_string());
        }
        let log4rs_file = log4rs_file.canonicalize().unwrap_or(log4rs_file);
        log4rs::init_file(log4rs_file.clone(), Default::default())?;
        debug!("成功载入 log4rs 配置文件“{}”，已开启日志功能", log4rs_file.display());
    } else {
        println!("[WARN]不能找到 log4rs 配置文件“{:?}”，所以日志没有被开启", log4rs_file);
    }
    Ok(())
}
pub struct Unwrap {
    builder: Rc<Builder>
}
impl Unwrap {
    pub fn new(builder: Rc<Builder>) -> Self {
        Unwrap {builder}
    }
    pub fn result2<T, E: Debug, F>(expr: StdResult<T, E>, message: F) -> T
    where F: AsRef<str> + Display {
        match expr {
            Ok(rs) => rs,
            Err(err) => {
                let message = message.as_ref();
                error!("[unwrap_build]{}, {:?}", message, err);
                panic!("{}, {:?}", message, err)
            }
        }
    }
    pub fn result3<T, E: Debug, F>(&self, expr: StdResult<T, E>, message: F) -> T
    where F: AsRef<str> + Display {
        match expr {
            Ok(rs) => rs,
            Err(err) => {
                let message = message.as_ref();
                error!("[unwrap_build]{}, {:?}", message, err);
                render::err_popup(&self.builder, message);
                panic!("{}, {:?}", message, err)
            }
        }
    }
    pub fn option2<T, F>(expr: Option<T>, message: F) -> T
    where F: AsRef<str> + Display {
        match expr {
            Some(rs) => rs,
            None => {
                let message = message.as_ref();
                error!("[unwrap_build]{}", message);
                panic!("{}", message)
            }
        }
    }
    pub fn option3<T, F>(&self, expr: Option<T>, message: F) -> T
    where F: AsRef<str> + Display {
        match expr {
            Some(rs) => rs,
            None => {
                let message = message.as_ref();
                error!("[unwrap_build]{}", message);
                render::err_popup(&self.builder, message);
                panic!("{}", message)
            }
        }
    }
}
