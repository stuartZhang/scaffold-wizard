mod main;

use ::clap::{App, load_yaml};
use ::std::fs;
use main::Result;

fn main() -> Result<()> {
    // 解析命令行参数
    let yaml = load_yaml!("bin-cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_file = matches.value_of("input-file").unwrap_or("assets/prompt-manifest.json");
    let output_file = matches.value_of("output-file").unwrap_or("assets/answers.json");
    let log4rs_file = matches.value_of("log4rs-file");
    // 初始化日志系统
    main::initialize_log4rs(log4rs_file)?;
    // 拼接输入与输出文件路径
    let (input_file, output_dir) = main::find_input_file(input_file)?;
    let output_file = main::find_output_file(output_file, &output_dir);
    // 读入【问卷】的输入文件
    let questions_str = fs::read_to_string(input_file)?;
    // 收集答案
    let answers_str = main::main(questions_str);
    // 写入【问卷】的输出文件
    if let Some(output_dir) = output_file.parent() {
        fs::create_dir_all(output_dir)?;
    }
    fs::write(output_file, &answers_str[..])?;
    Ok(())
}
