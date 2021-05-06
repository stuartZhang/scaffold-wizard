mod main;

use ::std::{ffi::CString, os::raw::c_char};

#[no_mangle]
pub extern fn inquire(questions: *const c_char) -> *const c_char {
    let questions = unsafe {CString::from_raw(questions as *mut i8)}.into_string()
        .expect("解析输入字符串错误");
    // 初始化日志系统
    main::initialize_log4rs().expect("log4rs初始化失败");
    // 收集答案
    let answers = main::main(questions);
    CString::new(answers).expect("打包输出字符串错误").into_raw()
}
#[cfg(test)]
mod test {
    use ::std::fs;
    use super::{c_char, CString, main};
    #[test]
    fn test() {
        let input_file = "assets/prompt-manifest.json";
        let output_file = "assets/answers-unit_test.json";
        // 拼接输入与输出文件路径
        let (input_file, output_dir) = main::find_input_file(input_file).unwrap();
        // 读入【问卷】的输入文件
        let questions = fs::read_to_string(input_file).unwrap();
        let questions = CString::new(questions).unwrap();
        // 收集答案
        let answers = super::inquire(questions.into_raw());
        let answers = unsafe {CString::from_raw(answers as *mut c_char)}.into_string().unwrap();
        // 读入【问卷】的输出文件
        let output_file = main::find_output_file(output_file, &output_dir);
        let output_file = fs::read_to_string(output_file).unwrap();
        // 比较
        assert_eq!(answers, output_file);
    }
}
