mod utils;
mod render;

use ::glib::clone;
use ::gio::prelude::*;
use ::gtk::{Application, ApplicationWindow, Builder, Button, Frame, prelude::*};
use ::log::debug;
use ::quick_xml::{events::{BytesText, Event}, Reader, Writer};
use ::std::{cell::RefCell, io::Cursor, rc::Rc, path::{Path, PathBuf}};
use ::serde_json::{map::Map, Value as jValue};
use render::{Data, StepTracer, StepDirection, Widgets};
pub use utils::{find_input_file, find_output_file, get_exe_dir, initialize_log4rs, Result};

pub fn main(questions_str: String, bin_dir: Option<PathBuf>) -> String {
    let application = Application::new(Some("scaffold.wizard"), Default::default())
        .expect("GTK 绑定失败");
    let answers_str = Rc::new(RefCell::new(String::new()));
    {
        let answers_str = Rc::clone(&answers_str);
        application.connect_activate(move |application|
            build_ui(application, &questions_str[..], Rc::clone(&answers_str), bin_dir.as_deref()));
    }
    application.run(&[]);
    answers_str.take()
}
fn build_ui(application: &Application, questions_str: &str, answers_str: Rc<RefCell<String>>, bin_dir: Option<&Path>) {
    let json_obj: jValue = serde_json::from_str(&questions_str[..])
        .expect("问卷配置文件不能被解析");
    let prompts = json_obj["prompts"].as_object()
        .expect("问卷配置文件中，没有找到 prompts 根结点");
    let step_count = prompts.len();
    let step_index = Rc::new(RefCell::new(0usize));
    let data = Rc::new(Data {
        answers: Rc::new(RefCell::new(Map::new())),
        prompts: Rc::new(prompts.clone())
    });
    let view_layout = include_str!("window.glade");
    let view_layout = if let Some(bin_dir) = bin_dir {
        prefix_icon_file_path(view_layout, bin_dir)
            .expect("篡改 <property name=\"icon\"> 与 <property name=\"pixbuf\"> 图标路径失败")
    } else {
        view_layout.to_string()
    };
    //
    let builder = Builder::from_string(view_layout.as_str());
    let window: ApplicationWindow = builder.get_object("outmost-window")
        .expect("不能从 glade 布局文件里，找到 outmost-window 元素");
    window.set_application(Some(application));
    let step_viewer: Frame = builder.get_object("step-viewer")
        .expect("不能从 glade 布局文件里，找到 step-viewer 元素");
    let prev_step_btn: Button = builder.get_object("btn-prev-step")
        .expect("不能从 glade 布局文件里，找到 btn-prev-step 元素");
    let next_step_btn: Button = builder.get_object("btn-next-step")
        .expect("不能从 glade 布局文件里，找到 btn-next-step 元素");
    builder.connect_signals(clone!(@weak step_index, @strong data, @strong answers_str => @default-panic, move |builder, handler_name|
        render::connect_signals(builder, handler_name, step_count, step_index, Rc::clone(&data), Rc::clone(&answers_str), bin_dir.map(|p| p.to_path_buf()))));
    render::draw_page(Widgets {
        step_viewer,
        prev_step_btn,
        next_step_btn
    }, StepTracer {
        step_index: Rc::clone(&step_index),
        step_count,
        direction: StepDirection::FORWARD
    }, Rc::clone(&data), bin_dir);
    window.show_all();
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
