mod utils;
mod render;

use ::glib::clone;
use ::gio::prelude::*;
use ::gtk::{Application, ApplicationWindow, Builder, Button, Frame, prelude::*};
use ::std::{cell::RefCell, rc::Rc};
use ::serde_json::{map::Map, Value as jValue};
use render::{Data, StepTracer, StepDirection, Widgets};
pub use utils::{find_input_file, find_output_file, initialize_log4rs, Result};

pub fn main(questions_str: String) -> String {
    let application = Application::new(Some("scaffold.wizard"), Default::default())
        .expect("GTK 绑定失败");
    let answers_str = Rc::new(RefCell::new(String::new()));
    {
        let answers_str = Rc::clone(&answers_str);
        application.connect_activate(move |application| build_ui(application, &questions_str[..], Rc::clone(&answers_str)));
    }
    application.run(&[]);
    answers_str.take()
}
fn build_ui(application: &Application, questions_str: &str, answers_str: Rc<RefCell<String>>) {
    let json_obj: jValue = serde_json::from_str(&questions_str[..])
        .expect("问卷配置文件不能被解析");
    let prompts = json_obj["prompts"].as_object()
        .expect("问卷配置文件中，没有找到 prompts 根结点");
    // println!("{:?}", prompts);
    let step_count = prompts.len();
    let step_index = Rc::new(RefCell::new(0usize));
    let data = Rc::new(Data {
        answers: Rc::new(RefCell::new(Map::new())),
        prompts: Rc::new(prompts.clone())
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
    builder.connect_signals(clone!(@weak step_index, @strong data, @strong answers_str => @default-panic, move |builder, handler_name|
        render::connect_signals(builder, handler_name, step_count, step_index, Rc::clone(&data), Rc::clone(&answers_str))));
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
