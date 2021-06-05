#![feature(option_insert)]
mod main;

use ::std::{ffi::CString, os::raw::c_char, path::PathBuf, ptr, thread};
use main::{QuestionsInput, Unwrap};

#[no_mangle]
pub extern fn inquire(questions: *const c_char, bin_dir: *const c_char, log4rs_file: *const c_char) -> *const c_char {
    let questions = Unwrap::result2(unsafe {CString::from_raw(questions as *mut i8)}.into_string(), "解析输入字符串错误");
    let bin_dir = Unwrap::result2(unsafe {CString::from_raw(bin_dir as *mut i8)}.into_string(), "解析输入字符串错误");
    let bin_dir = PathBuf::from(bin_dir);
    if !log4rs_file.is_null() { // 初始化日志系统
        let log4rs_file = unsafe {CString::from_raw(log4rs_file as *mut i8)}.into_string();
        let log4rs_file = log4rs_file.as_deref().ok();
        Unwrap::result2(main::initialize_log4rs(log4rs_file, Some(bin_dir.as_path())), "log4rs初始化失败");
    }
    // 收集答案
    if let (_, Some(answers)) = main::main(QuestionsInput::FileText(questions), Some(bin_dir)) {
        // 写入【问卷】的输出文件
        return Unwrap::result2(CString::new(answers), "打包输出字符串错误").into_raw();
    }
    ptr::null()
}
#[export_name = "inquireAsync"]
pub extern fn inquire_async(questions: *const c_char, bin_dir: *const c_char, log4rs_file: *const c_char, cb: extern fn(*const c_char, *const c_char)) {
    let questions = Unwrap::result2(unsafe {CString::from_raw(questions as *mut i8)}.into_string(), "解析输入字符串错误");
    let bin_dir = Unwrap::result2(unsafe {CString::from_raw(bin_dir as *mut i8)}.into_string(), "解析输入字符串错误");
    let bin_dir = PathBuf::from(bin_dir);
    if !log4rs_file.is_null() { // 初始化日志系统
        let log4rs_file = unsafe {CString::from_raw(log4rs_file as *mut i8)}.into_string();
        let log4rs_file = log4rs_file.as_deref().ok();
        Unwrap::result2(main::initialize_log4rs(log4rs_file, Some(bin_dir.as_path())), "log4rs初始化失败");
    }
    // 收集答案
    thread::spawn(move || {
        let join_handle = thread::spawn(move || {
            main::main(QuestionsInput::FileText(questions), Some(bin_dir)).1
        });
        match join_handle.join() {
            Ok(answers) => {
                if let Some(answers) = answers {
                    cb(ptr::null(), Unwrap::result2(CString::new(answers), "打包输出字符串错误").into_raw());
                }
            },
            Err(err) => {
                cb(Unwrap::result2(CString::new(format!("{:?}", err)), "打包输出字符串错误").into_raw(), ptr::null());
            }
        }
    });
}
#[cfg(test)]
mod test {
    use ::lazy_static::lazy_static;
    use ::mut_static::MutStatic;
    use ::std::{env, fs, path::Path, thread, time::Duration};
    use super::{c_char, CString, main, ptr};
    lazy_static! {
        static ref ANSWER_ASYNC: MutStatic<Option<String>> = MutStatic::new();
    }
    #[test]
    fn test_sync() {
        let input_file = "../assets/prompt-manifest.json";
        let output_file = "../assets/answers-unit_test.json";
        // 拼接输入与输出文件路径
        let (input_file, output_dir) = main::find_input_file(input_file).unwrap();
        // 读入【问卷】的输入文件
        let questions = fs::read_to_string(input_file).unwrap();
        let questions = CString::new(questions).unwrap();
        // 工作目录
        let mut bin_dir = env::current_dir().unwrap();
        bin_dir.push(Path::new("target/debug"));
        let bin_dir = CString::new(bin_dir.to_str().unwrap()).unwrap();
        // 收集答案
        let answers = super::inquire(questions.into_raw(), bin_dir.into_raw(), ptr::null());
        let answers = unsafe {CString::from_raw(answers as *mut c_char)}.into_string().unwrap();
        // 读入【问卷】的输出文件
        let output_file = main::find_output_file(output_file, &output_dir);
        let output_file = fs::read_to_string(output_file).unwrap();
        // 比较
        assert_eq!(format!("{}\n", answers), output_file);
    }
    #[test]
    fn test_async() {
        let input_file = "../assets/prompt-manifest.json";
        // 拼接输入与输出文件路径
        let (input_file, _) = main::find_input_file(input_file).unwrap();
        // 读入【问卷】的输入文件
        let questions = fs::read_to_string(input_file).unwrap();
        let questions = CString::new(questions).unwrap();
        // 工作目录
        let mut bin_dir = env::current_dir().unwrap();
        bin_dir.push(Path::new("target/debug"));
        let bin_dir = CString::new(bin_dir.to_str().unwrap()).unwrap();
        // 收集答案
        ANSWER_ASYNC.set(None).unwrap();
        super::inquire_async(questions.into_raw(), bin_dir.into_raw(), ptr::null(), cb);
        loop {
            if let Some(_) = *ANSWER_ASYNC.read().unwrap() {
                break;
            }
            thread::sleep(Duration::from_millis(600));
        }
    }
    extern fn cb(err: *const c_char, answers: *const c_char) {
        let mut answers_async = ANSWER_ASYNC.write().unwrap();
        if err.is_null() {
            let answers = unsafe {CString::from_raw(answers as *mut c_char)}.into_string().unwrap();
            // 读入【问卷】的输出文件
            let input_file = "../assets/prompt-manifest.json";
            let output_file = "../assets/answers-unit_test.json";
            let (_, output_dir) = main::find_input_file(input_file).unwrap();
            let output_file = main::find_output_file(output_file, &output_dir);
            let output_file = fs::read_to_string(output_file).unwrap();
            // 比较
            answers_async.replace(answers.clone());
            assert_eq!(format!("{}\n", answers), output_file);
        } else {
            let err = unsafe {CString::from_raw(err as *mut c_char)}.into_string().unwrap();
            answers_async.replace(err);
        }
    }
}
