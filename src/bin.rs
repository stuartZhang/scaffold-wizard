#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod main;

use ::clap::{App, load_yaml};
use ::std::fs;
use main::Result;

fn main() -> Result<()> {
    // 解析命令行参数
    let yaml = load_yaml!("bin-cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let bin_dir = main::get_exe_dir()?;
    // 初始化日志系统
    let log4rs_file = matches.value_of("log4rs-file");
    main::initialize_log4rs(log4rs_file, Some(bin_dir.as_path()))?;
    // 收集答案
    let input_file = matches.value_of("input-file").map(|s| s.to_string());
    if let (Some(output_dir), Some(answers_str)) = main::main(input_file.into(), Some(bin_dir)) {
        // 写入【问卷】的输出文件
        let output_file = matches.value_of("output-file").unwrap_or("answers.json");
        let output_file = main::find_output_file(output_file, &output_dir);
        if let Some(output_dir) = output_file.parent() {
            fs::create_dir_all(output_dir)?;
        }
        fs::write(output_file, &answers_str[..])?;
    }
    Ok(())
}
