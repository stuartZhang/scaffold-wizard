use ::eval::Value as eValue;
use ::glib::clone;
use ::gtk::{Box as GtkBox, CheckButton, Orientation, prelude::*};
use ::serde_json::{map::Map, Value as jValue};
use ::std::{cell::RefCell, rc::Rc};

pub fn build_ui((name, json): (&String, &jValue), answers: Rc<RefCell<Map<String, eValue>>>) -> Option<GtkBox> {
    if let Some(choices) = json["choices"].as_array() {
        if answers.borrow().get(name).is_none() {
            let choice_answers: Map<String, eValue> = Map::with_capacity(choices.len());
            answers.borrow_mut().insert(name.clone(), eValue::Object(choice_answers));
        }
        let question_name = Rc::new(name.clone());
        let v_inner_box = GtkBox::new(Orientation::Vertical, 10);
        for choice in choices {
            let choice_name = choice["name"].as_str().unwrap_or("未配置");
            if !super::evaluate_when(&format!("问题 {} 的备选项 {}", name, choice_name)[..], choice, Rc::clone(&answers)) {
                continue;
            }
            let choice_value = choice["value"].as_str().unwrap_or_default().to_string();
            let answers = Rc::clone(&answers);
            let mut choice_answers = answers.borrow_mut();
            let choice_answers = choice_answers.get_mut(name).unwrap().as_object_mut().unwrap();
            let check_btn = CheckButton::with_label(choice_name);
            check_btn.set_active(if let Some(choice_checked) = choice_answers.get(&choice_value) {
                choice_checked.as_bool().unwrap_or_default()
            } else {
                let choice_checked = choice["checked"].as_bool().unwrap_or_default();
                choice_answers.insert(choice_value.clone(), eValue::Bool(choice_checked));
                choice_checked
            });
            check_btn.connect_toggled(clone!(@strong answers, @strong question_name => move |check_btn| {
                let choice_checked = check_btn.get_active();
                let mut choice_answers = answers.borrow_mut();
                let choice_answers = choice_answers.get_mut(&*question_name).unwrap().as_object_mut().unwrap();
                choice_answers.insert(choice_value.clone(), eValue::Bool(choice_checked));
            }));
            v_inner_box.pack_start(&check_btn, false, true, 0);
        }
        return Some(v_inner_box);
    }
    None
}
