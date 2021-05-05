mod utils;
mod render;

use ::glib::clone;
use ::gio::prelude::*;
use ::gtk::{Application, ApplicationWindow, Builder, Button, Frame, prelude::*};
use ::std::{fs, cell::RefCell, rc::Rc};
use ::serde_json::{map::Map, Value as jValue};
use utils::{Error, Result};
use render::{Data, StepTracer, StepDirection, Widgets};

fn main() -> Result<()> {
    utils::initialize_log4rs()?;
    let prompts_json = read_json_file()?;
    let application = Application::new(Some("scaffold.wizard"), Default::default())
        .expect("GTK 绑定失败");
    application.connect_activate(move |application| build_ui(application, prompts_json.clone()));
    application.run(&[]);
    Ok(())
}
fn build_ui(application: &Application, prompts: Map<String, jValue>) {
    // println!("{:?}", prompts);
    let step_count = prompts.len();
    let step_index = Rc::new(RefCell::new(0usize));
    let data = Rc::new(Data {
        answers: Rc::new(RefCell::new(Map::new())),
        prompts: Rc::new(prompts)
    });
    let view_layout = include_str!("window.glade");
    let builder = Builder::from_string(view_layout);
    let window: ApplicationWindow = builder.get_object("outmost-window")
        .expect("不能从 glade 布局文件里，找到 outmost-window 元素");
    window.set_application(Some(application));
    let step_viewer: Frame = builder.get_object("step-viewer")
        .expect("不能从 glade 布局文件里，找到 step-viewer 元素");
    let prev_step_btn: Button = builder.get_object("btn-prev-step")
        .expect("不能从 glade 布局文件里，找到 btn-prev-step 元素");
    let next_step_btn: Button = builder.get_object("btn-next-step")
        .expect("不能从 glade 布局文件里，找到 btn-next-step 元素");
    builder.connect_signals(clone!(@weak data, @weak step_index => @default-panic, move |builder, handler_name|
        render::connect_signals(builder, handler_name, step_count, step_index, data)));
    render::draw_page(Widgets {
        step_viewer,
        prev_step_btn,
        next_step_btn
    }, StepTracer {
        step_index: Rc::clone(&step_index),
        step_count,
        direction: StepDirection::FORWARD
    }, Rc::clone(&data));
    window.show_all();
}
fn read_json_file() -> Result<Map<String, jValue>> {
    let mut json_file_path = utils::get_assets_dir()?;
    json_file_path.push("prompt-manifest.json");
    let json_str = fs::read_to_string(json_file_path)?;
    let json_obj: jValue = serde_json::from_str(&json_str[..])?;
    if let Some(prompts) = json_obj["prompts"].as_object() {
        return Ok(prompts.clone());
    }
    Err(Box::new(Error::new("json 配置对象的顶层属性里没有 prompts", None)))
}
fn write_json_file(content: &str) -> Result<()> {
    let mut json_file_path = utils::get_assets_dir()?;
    json_file_path.push("answers.json");
    fs::write(json_file_path, content)?;
    Ok(())
}
