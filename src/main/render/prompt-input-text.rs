use ::eval::Value as eValue;
use ::gio::Icon;
use ::glib::clone;
use ::gtk::{Entry, prelude::*,};
use ::serde_json::{map::Map, Value as jValue};
use ::std::{cell::RefCell, rc::Rc};

pub fn build_ui((name, json): (&String, &jValue), answers: Rc<RefCell<Map<String, eValue>>>) -> Entry {
    if answers.borrow().get(name).is_none() {
        let text = json["default"].as_str().unwrap_or_default();
        answers.borrow_mut().insert(name.clone(), eValue::String(text.to_string()));
    }
    let question_name = Rc::new(name.clone());
    let input_str = Entry::new();
    if let Ok(icon) = Icon::new_for_string("gtk-info") {
        input_str.set_property_primary_icon_gicon(Some(&icon));
    }
    input_str.set_placeholder_text(Some("请输入..."));
    input_str.set_text(answers.borrow().get(name).unwrap().as_str().unwrap());
    input_str.set_property_width_request(200);
    input_str.connect_property_text_notify(clone!(@strong answers, @strong question_name => move |input_str| {
        answers.borrow_mut().insert(question_name.to_string(), eValue::String(input_str.get_text().to_string()));
    }));
    input_str
}
