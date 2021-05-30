mod utils;
mod render;

use ::glib::clone;
use ::gio::prelude::*;
use ::gtk::{Application, ApplicationWindow, Builder, Button, FileChooserAction, FileChooserDialog, FileFilter, Frame, prelude::*, ResponseType};
use ::log::debug;
use ::quick_xml::{events::{BytesText, Event}, Reader, Writer};
use ::std::{borrow::Borrow, cell::RefCell, env, fs, io::Cursor, rc::Rc, path::{Path, PathBuf}};
use ::serde_json::{map::Map, Value as jValue};
use render::{Data, StepTracer, StepDirection, Widgets};
pub use utils::{find_input_file, find_output_file, get_exe_dir, initialize_log4rs, Result, Unwrap};

pub enum QuestionsInput {
    FilePath(String),
    #[allow(dead_code)]
    FileText(String),
    None
}
impl From<Option<String>> for QuestionsInput {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(i) => QuestionsInput::FilePath(i),
            None => QuestionsInput::None
        }
    }
}
pub fn main(q_input: QuestionsInput, bin_dir: Option<PathBuf>) -> (Option<PathBuf>, Option<String>) {
    env::set_var("G_FILENAME_ENCODING", "UTF-8"); // 参见 https://gtk-rs.org/docs/gtk/struct.FileChooser.html#file-names-and-encodings
    let application = Unwrap::result2(Application::new(Some("scaffold.wizard"), Default::default()), "GTK 绑定失败");
    let q_file_path = Rc::new(q_input);
    let a_file_dir: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let answers_str: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    {
        let a_file_dir = Rc::clone(&a_file_dir);
        let answers_str = Rc::clone(&answers_str);
        application.connect_activate(move |application|
            build_ui(application, bin_dir.as_deref(), Rc::clone(&q_file_path), Rc::clone(&a_file_dir), Rc::clone(&answers_str)));
    }
    application.run(&[]);
    (a_file_dir.take(), answers_str.take())
}
fn build_ui(application: &Application, bin_dir: Option<&Path>, q_input: Rc<QuestionsInput>, a_file_dir: Rc<RefCell<Option<PathBuf>>>, answers_str: Rc<RefCell<Option<String>>>) {
    let view_layout = include_str!("window.glade");
    let view_layout = if let Some(bin_dir) = bin_dir {
        Unwrap::result2(prefix_icon_file_path(view_layout, bin_dir),
            "篡改 <property name=\"icon\"> 与 <property name=\"pixbuf\"> 图标路径失败")
    } else {
        view_layout.to_string()
    };
    let bin_dir = Rc::new(bin_dir.map(|p| p.to_path_buf()));
    let builder = Rc::new(Builder::from_string(view_layout.as_str()));
    let unwrap = Rc::new(Unwrap::new(Rc::clone(&builder)));
    let window: ApplicationWindow = unwrap.option3(builder.get_object("outmost-window"), "不能从 glade 布局文件里，找到 outmost-window 元素");
    window.set_application(Some(application));
    if let QuestionsInput::FilePath(q_file_path) = q_input.borrow() { // 由命令行提供了输入文件的路径
        let (input_file, _) = unwrap.result3(utils::find_input_file(q_file_path),
            format!("无效输入文件路径：{}", q_file_path));
        let questions_str = unwrap.result3(fs::read_to_string(input_file.clone()),
            format!("无效输入文件路径：{}", input_file.display()));
        build_ui_core(Rc::clone(&builder), questions_str, Rc::clone(&answers_str), Rc::clone(&bin_dir), Rc::clone(&unwrap));
        let mut a_file_dir = a_file_dir.borrow_mut();
        a_file_dir.insert(unwrap.option3(input_file.parent().map(|p| p.to_path_buf()),
            format!("{} 没有上一级目录", input_file.display())));
    } else if let QuestionsInput::FileText(questions_str) = q_input.borrow() { // 由 dll 调用提供了输入文件的内容
        build_ui_core(Rc::clone(&builder), questions_str, Rc::clone(&answers_str), Rc::clone(&bin_dir), Rc::clone(&unwrap));
    } else { // 什么都没有提供
        let json_file_filter: FileFilter = unwrap.option3(builder.get_object("json_file_filter"),
            "不能从 glade 布局文件里，找到 json_file_filter 元素");
        let file_chooser = FileChooserDialog::new(Some("寻找问题清单文件"), Some(&window), FileChooserAction::Open);
        file_chooser.set_filter(&json_file_filter);
        if let Some(bin_dir) = bin_dir.borrow() {
            let mut assets_dir = bin_dir.clone();
            assets_dir.push("../assets");
            if assets_dir.exists() && assets_dir.is_dir() {
                file_chooser.set_current_folder(assets_dir);
            }
        }
        file_chooser.add_buttons(&[("取消", ResponseType::Cancel), ("打开", ResponseType::Ok)]);
        if ResponseType::Ok == file_chooser.run() {
            if let Some(input_file) =  file_chooser.get_filename() {
                let questions_str = unwrap.result3(fs::read_to_string(input_file.clone()),
                    format!("无效输入文件路径：{}", input_file.display()));
                build_ui_core(Rc::clone(&builder), questions_str, Rc::clone(&answers_str), Rc::clone(&bin_dir), Rc::clone(&unwrap));
                file_chooser.hide();
                let mut a_file_dir = a_file_dir.borrow_mut();
                a_file_dir.insert(unwrap.option3(input_file.parent().map(|p| p.to_path_buf()),
                    format!("{} 没有上一级目录", input_file.display())));
            }
        }
    }
    window.show_all();
}
fn build_ui_core<T>(builder: Rc<Builder>, questions_str: T, answers_str: Rc<RefCell<Option<String>>>, bin_dir: Rc<Option<PathBuf>>, unwrap: Rc<Unwrap>) where T: AsRef<str> {
    let json_obj: jValue = unwrap.result3(serde_json::from_str(questions_str.as_ref()), "问卷配置文件不能被解析");
    let prompts = unwrap.option3(json_obj["prompts"].as_object(), "问卷配置文件中，没有找到 prompts 根结点");
    let step_count = prompts.len();
    let step_index = Rc::new(RefCell::new(0usize));
    let data = Rc::new(Data {
        answers: Rc::new(RefCell::new(Map::new())),
        prompts: Rc::new(prompts.clone())
    });
    let step_viewer: Frame = unwrap.option3(builder.get_object("step-viewer"), "不能从 glade 布局文件里，找到 step-viewer 元素");
    let prev_step_btn: Button = unwrap.option3(builder.get_object("btn-prev-step"), "不能从 glade 布局文件里，找到 btn-prev-step 元素");
    let next_step_btn: Button = unwrap.option3(builder.get_object("btn-next-step"), "不能从 glade 布局文件里，找到 btn-next-step 元素");
    builder.connect_signals(clone!(@weak step_index, @strong data, @strong answers_str, @strong bin_dir, @strong unwrap => @default-panic, move |builder, handler_name| render::connect_signals(builder, handler_name, step_count, step_index, Rc::clone(&data), Rc::clone(&answers_str), Rc::clone(&bin_dir), Rc::clone(&unwrap))));
    render::draw_page(Widgets {
        step_viewer,
        prev_step_btn,
        next_step_btn
    }, StepTracer {
        step_index: Rc::clone(&step_index),
        step_count,
        direction: StepDirection::FORWARD
    }, Rc::clone(&data), bin_dir.as_deref(), Rc::clone(&unwrap));
}
fn prefix_icon_file_path(source: &str, bin_dir: &Path) -> Result<String> {
    let mut reader = Reader::from_str(source);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    let mut is_pixbuf = false;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"property" && e.attributes().count() == 1 && e.attributes().any(|attribute|{
                if let Ok(attribute) = attribute {
                    if attribute.key == b"name" {
                        if let Ok(value) = attribute.unescape_and_decode_value(&reader) {
                            return value == "pixbuf" || value == "icon";
                        }
                    }
                }
                false
            }) => {
                is_pixbuf = true;
                writer.write_event(&Event::Start(e.clone()))?
            },
            Ok(Event::Text(ref e)) if is_pixbuf => {
                is_pixbuf = false;
                if let Ok(file_path) = e.unescape_and_decode(&reader) {
                    let mut bin_dir = bin_dir.to_path_buf();
                    bin_dir.push(Path::new(&file_path[..]));
                    if let Some(file_path) = bin_dir.to_str() {
                        writer.write_event(&Event::Text(BytesText::from_plain(file_path.as_bytes())))?
                    }
                }
            },
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(&e)?,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }
    let result = String::from_utf8(writer.into_inner().into_inner())?;
    debug!("glade: {}", result);
    Ok(result)
}
