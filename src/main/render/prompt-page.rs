use ::gtk::{Align, Box as GtkBox, Label, Orientation, prelude::*};
use ::log::debug;
use ::serde_json::Value as jValue;
use ::std::{path::Path, rc::Rc};
use super::{Data, prompt_checkbox, prompt_input_port, prompt_input_text, prompt_radio, prompt_switch, StepDirection, StepTracer, Widgets};

pub fn draw_page(widgets: Widgets, step_tracer: StepTracer, data: Rc<Data>, bin_dir: Option<&Path>) {
    let answers = Rc::clone(&data.answers);
    let prompts = Rc::clone(&data.prompts);
    let (name, json) = loop {
        let is_first_step = step_tracer.is_first();
        let is_last_step = step_tracer.is_last();
        let step_number = step_tracer.get_number();
        let mut step_index = step_tracer.step_index.borrow_mut();
        let (name, json) = prompts.iter().nth(*step_index)
            .expect(&format!("没有第 {} 步配置", step_number)[..]);
        debug!("第 {} 步，内容 {} {:?}", step_number, name, json);
        if super::evaluate_when(&format!("第 {} 步", step_number)[..], json, Rc::clone(&answers)) {
            break (name, json);
        } else {
            match step_tracer.direction {
                StepDirection::FORWARD => {
                    if is_last_step {
                        break (name, json);
                    } else {
                        *step_index += 1;
                    }
                },
                StepDirection::BACKWARD => {
                    if is_first_step {
                        break (name, json);
                    } else {
                        *step_index -= 1;
                    }
                }
            }
        }
    };
    widgets.prev_step_btn.set_sensitive(!step_tracer.is_first());
    widgets.next_step_btn.set_sensitive(!step_tracer.is_last());
    let q_type = json["type"].as_str().unwrap_or_default();
    let v_outer_box = GtkBox::new(Orientation::Vertical, 25);
    v_outer_box.set_halign(Align::Center);
    v_outer_box.set_valign(Align::Center);
    let label = prompt_question_label(json, &step_tracer, Rc::clone(&data));
    v_outer_box.pack_start(&label, false, true, 0);
    if q_type == "checkbox" {
        if let Some(v_inner_box) = prompt_checkbox::build_ui((name, json), Rc::clone(&answers)) {
            v_outer_box.pack_start(&v_inner_box, false, true, 0);
        }
    } else if q_type == "list" {
        if let Some(v_inner_box) = prompt_radio::build_ui((name, json), Rc::clone(&answers)) {
            v_outer_box.pack_start(&v_inner_box, false, true, 0);
        }
    } else if q_type == "input" || q_type == "string" {
        let q_type = json["subType"].as_str().unwrap_or_default();
        if q_type == "port" {
            let spin_btn = prompt_input_port::build_ui((name, json), Rc::clone(&answers));
            v_outer_box.pack_start(&spin_btn, false, true, 0);
        } else {
            let input_str = prompt_input_text::build_ui((name, json), Rc::clone(&answers), bin_dir);
            v_outer_box.pack_start(&input_str, false, true, 0);
        }
    } else if q_type == "confirm" {
        let switch_btn = prompt_switch::build_ui((name, json), answers);
        v_outer_box.pack_start(&switch_btn, false, true, 0);
    }
    // 替换切签内容
    widgets.step_viewer.foreach(|child_widget| widgets.step_viewer.remove(child_widget));
    widgets.step_viewer.add(&v_outer_box);
    v_outer_box.show_all();
}
fn prompt_question_label(json: &jValue, step_tracer: &StepTracer, data: Rc<Data>) -> Label {
    let label = Label::new(None);
    let asterisk = if json["required"].as_bool().unwrap_or_default() {
        r#"<span foreground="red" weight="heavy">*</span> "#
    } else {
        ""
    };
    let step_number = step_tracer.get_display_number(data);
    label.set_markup(json["message"].as_str().map(|text| format!("{}{}. {}", asterisk, step_number, text)).as_deref().unwrap_or_default());
    label
}
