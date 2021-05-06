#[path = "render/prompt-checkbox.rs"]
mod prompt_checkbox;
#[path = "render/prompt-radio.rs"]
mod prompt_radio;
#[path = "render/prompt-input-port.rs"]
mod prompt_input_port;
#[path = "render/prompt-input-text.rs"]
mod prompt_input_text;
#[path = "render/prompt-switch.rs"]
mod prompt_switch;
#[path = "render/prompt-page.rs"]
mod prompt_page;
#[path = "render/error-dialog.rs"]
mod error_dialog;
//
use ::eval::{Expr, Value as eValue};
use ::gio::ApplicationExt;
use ::glib::{clone, Value as GtkValue};
use ::gtk::{ApplicationWindow, Builder, Button, Frame, prelude::*};
use ::log::debug;
use ::serde_json::{map::Map, Value as jValue};
use ::std::{cell::RefCell, rc::Rc};
pub use prompt_page::draw_page;
//
pub enum StepDirection {
    FORWARD,
    BACKWARD
}
pub struct Widgets {
    pub step_viewer: Frame,
    pub prev_step_btn: Button,
    pub next_step_btn: Button
}
pub struct StepTracer {
    pub step_index: Rc<RefCell<usize>>,
    pub step_count: usize,
    pub direction: StepDirection
}
impl StepTracer {
    fn is_first(&self) -> bool {
        let step_index = *self.step_index.borrow();
        step_index <= 0
    }
    fn is_last(&self) -> bool {
        let step_index = *self.step_index.borrow();
        step_index >= self.step_count - 1
    }
    fn get_number(&self) -> usize {
        let step_index = *self.step_index.borrow();
        step_index + 1
    }
    fn get_display_number(&self, data: Rc<Data>) -> usize {
        let answers = Rc::clone(&data.answers);
        let prompts = Rc::clone(&data.prompts);
        let mut step_number = self.get_number();
        for step_index in 0..*self.step_index.borrow() {
            let (_, json) = prompts.iter().nth(step_index).unwrap();
            if !evaluate_when(&format!("第 {} / {} 步", step_index + 1, step_number)[..], json, Rc::clone(&answers)) {
                step_number -= 1;
            }
        }
        step_number
    }
}
#[derive(Clone)]
pub struct Data {
    pub answers: Rc<RefCell<Map<String, eValue>>>,
    pub prompts:Rc<Map<String, jValue>>
}
pub fn connect_signals(builder: &Builder, handler_name: &str,
                       step_count: usize, step_index: Rc<RefCell<usize>>,
                       data: Rc<Data>, answers_str: Rc<RefCell<String>>)
                       -> Box<dyn Fn(&[GtkValue]) -> Option<GtkValue> + 'static> {
    let window: ApplicationWindow = builder.get_object("outmost-window")
        .expect("不能从 glade 布局文件里，找到 outmost-window 元素");
    let step_viewer: Frame = builder.get_object("step-viewer")
        .expect("不能从 glade 布局文件里，找到 step-viewer 元素");
    let prev_step_btn: Button = builder.get_object("btn-prev-step")
        .expect("不能从 glade 布局文件里，找到 btn-prev-step 元素");
    let next_step_btn: Button = builder.get_object("btn-next-step")
        .expect("不能从 glade 布局文件里，找到 btn-next-step 元素");
    if handler_name == "on-btn-prev-click" {
        Box::new(clone!(@weak prev_step_btn, @weak next_step_btn, @weak step_viewer, @strong step_index, @strong data => @default-return None, move |_| {
            {
                let mut step_index = step_index.borrow_mut();
                if *step_index > 0 {
                    *step_index -= 1;
                }
            }
            draw_page(Widgets {
                step_viewer,
                prev_step_btn,
                next_step_btn
            }, StepTracer {
                step_index: Rc::clone(&step_index),
                step_count,
                direction: StepDirection::BACKWARD
            }, Rc::clone(&data));
            None
        }))
    } else if handler_name == "on-btn-next-click" {
        Box::new(clone!(@weak prev_step_btn, @weak next_step_btn, @weak step_viewer, @strong step_index, @strong builder, @strong data => @default-return None, move |_| {
            {
                let mut step_index = step_index.borrow_mut();
                if let Err(message) = validate_page(*step_index, Rc::clone(&data)) {
                    debug!("表单验证失败消息：{}", message);
                    error_dialog::show(&builder, &message[..]);
                    return None;
                }
                if *step_index < step_count - 1 {
                    *step_index += 1;
                }
            }
            draw_page(Widgets {
                step_viewer,
                prev_step_btn,
                next_step_btn
            }, StepTracer {
                step_index: Rc::clone(&step_index),
                step_count,
                direction: StepDirection::FORWARD
            }, Rc::clone(&data));
            None
        }))
    } else if handler_name == "on-btn-submit-click" {
        Box::new(clone!(@weak window => @default-return None, move |_| {
            window.close();
            None
        }))
    } else if handler_name == "on-window-delete" {
        Box::new(clone!(@strong window, @strong builder, @strong data, @strong answers_str => @default-return true.to_value(), move |_| {
            let answers = Rc::clone(&data.answers);
            let prompts = Rc::clone(&data.prompts);
            let answers_output = serde_json::to_string_pretty(&jValue::Object(answers.borrow().clone())).unwrap();
            debug!("安装向导问卷调查结果：{}", answers_output);
            let mut error_message = None;
            for (step_index, (name, json)) in prompts.iter().enumerate() {
                if evaluate_when(&format!("第 {} 个问题 {}", step_index, name)[..], json, Rc::clone(&answers)) {
                    if let Err(message) = validate_page(step_index, Rc::clone(&data)) {
                        error_message = Some(message);
                        break;
                    }
                }
            }
            if let Some(message) = error_message {
                debug!("表单验证失败消息：{}", message);
                error_dialog::show(&builder, &message[..]);
            } else {
                let mut answers_str = answers_str.borrow_mut();
                answers_str.clear();
                answers_str.push_str(&answers_output[..]);
                if let Some(application) = window.get_application() {
                    application.quit();
                }
            }
            Some(true.to_value())
        }))
    } else {
        panic!("未被处理的组件事件 {}", handler_name)
    }
}
fn validate_page(step_index: usize, data: Rc<Data>) -> Result<(), String> {
    let step_number = step_index + 1;
    let answers = Rc::clone(&data.answers);
    let prompts = Rc::clone(&data.prompts);
    let (name, json) = prompts.iter().nth(step_index)
        .expect(&format!("没有第 {} 步配置", step_number)[..]);
    let required = json["required"].as_bool().unwrap_or_default();
    let q_type = json["type"].as_str().unwrap_or_default();
    let answers = answers.borrow();
    let answer = if let Some(answer) = answers.get(name) {
        answer
    } else {
        return Err(format!("尚未回答问题“{}”。请先完成问卷", json["message"].as_str().unwrap_or_default()));
    };
    if q_type == "checkbox" {
        let choice_empty = Map::new();
        let choice_answers = answer.as_object().unwrap_or(&choice_empty);
        let checked_count = choice_answers.iter().fold(0u8, |sum, (_, checked)| {
            if checked.as_bool().unwrap_or_default() {
                return sum + 1;
            }
            sum
        });
        if required && checked_count <= 0 {
            return Err("请选择至少选择一项".to_string());
        }
        if checked_count > 1 {
            let choice_prompts = json["choices"].as_array().unwrap();
            let violate_mutex = choice_answers.iter().any(|(value, checked)| {
                checked.as_bool().unwrap_or_default() &&
                choice_prompts.iter().any(|choice| {
                    choice["value"].as_str().unwrap_or_default() == value &&
                    choice["mutex"].as_bool().unwrap_or_default()
                })
            });
            if violate_mutex {
                let multiple_choices: Vec<&str> = choice_prompts.iter()
                    .filter(|choice| !choice["mutex"].as_bool().unwrap_or_default())
                    .map(|choice| choice["name"].as_str().unwrap_or_default()).collect();
                if multiple_choices.len() > 0 {
                    return Err(format!("仅 “{}” 是被允许同时勾选的", multiple_choices.join("”, “")));
                }
                return Err("皆不支持多选".to_string());
            }
        }
    } else if q_type == "input" || q_type == "string" {
        let q_type = json["subType"].as_str().unwrap_or_default();
        if q_type != "port" {
            let text_answers = answer.as_str().unwrap_or_default();
            if required && text_answers == "" {
                return Err("请录入内容".to_string());
            }
        }
    }
    Ok(())
}
fn evaluate_when(log_prefix: &str, prompt: &jValue, answers: Rc<RefCell<Map<String, eValue>>>) -> bool {
    if let Some(when) = prompt["when"].as_str() {
        let mut expr = Expr::new(when);
        for (name, value) in answers.borrow().iter() {
            expr = expr.value(name, value);
        }
        let result = expr.exec().map(|result| {
            debug!("{} 评估 {} 条件，结果是 {:?}", log_prefix, when, result);
            match result {
                eValue::Null => false,
                eValue::Bool(result) => result,
                eValue::String(result) => result.len() > 0,
                eValue::Number(result) => result.as_f64().unwrap_or_default() != 0f64,
                eValue::Array(_) => true,
                eValue::Object(_) => true
            }
        }).unwrap_or_else(|error| {
            debug!("{} 评估 {} 条件失败，原因是 {:?}", log_prefix, when, error);
            false
        });
        debug!("{} 评估 {} 条件，结果是 {:?}", log_prefix, when, result);
        result
    } else {
        true
    }
}
