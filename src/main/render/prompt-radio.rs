use ::eval::Value as eValue;
use ::glib::clone;
use ::gtk::{Box as GtkBox, Orientation, prelude::*, RadioButton};
use ::serde_json::{map::Map, Value as jValue};
use ::std::{cell::RefCell, rc::Rc};

pub fn build_ui((name, json): (&String, &jValue), answers: Rc<RefCell<Map<String, eValue>>>) -> Option<GtkBox> {
    if let Some(choices) = json["choices"].as_array() {
        if answers.borrow().get(name).is_none() {
            let default_index = json["default"].as_u64().unwrap_or_default() as usize;
            if let Some(choice) = choices.iter().nth(default_index) {
                answers.borrow_mut().insert(name.clone(), choice["value"].clone());
            }
        }
        let question_name = Rc::new(name.clone());
        let v_inner_box = GtkBox::new(Orientation::Vertical, 10);
        let mut current_radio: Option<RadioButton> = None;
        for choice in choices.iter() {
            let choice_name = choice["name"].as_str().unwrap_or("未配置");
            if !super::evaluate_when(&format!("问题 {} 的备选项 {}", name, choice_name)[..], choice, Rc::clone(&answers)) {
                continue;
            }
            let answers = Rc::clone(&answers);
            let radio_button = if let Some(radio_button) = current_radio {
                RadioButton::with_label_from_widget(&radio_button, choice_name)
            } else {
                RadioButton::with_label(choice_name)
            };
            let choice_value = &choice["value"];
            radio_button.set_active(if let Some(answer_value) = answers.borrow().get(name) {
                choice_value == answer_value
            } else {
                false
            });
            radio_button.connect_toggled(clone!(@strong question_name, @strong choice_value => move |radio_button| {
                let choice_checked = radio_button.get_active();
                if choice_checked {
                    answers.borrow_mut().insert(question_name.to_string(), choice_value.clone());
                }
            }));
            v_inner_box.pack_start(&radio_button, false, true, 0);
            current_radio = Some(radio_button);
        }
        return Some(v_inner_box);
    }
    None
}
