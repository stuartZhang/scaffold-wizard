use ::eval::Value as eValue;
use ::glib::clone;
use ::gtk::{Align, prelude::*, Switch};
use ::serde_json::{map::Map, Value as jValue};
use ::std::{cell::RefCell, rc::Rc};

pub fn build_ui((name, json): (&String, &jValue), answers: Rc<RefCell<Map<String, eValue>>>) -> Switch {
    if answers.borrow().get(name).is_none() {
        let switch = json["default"].as_bool().unwrap_or_default();
        answers.borrow_mut().insert(name.clone(), eValue::Bool(switch));
    }
    let question_name = Rc::new(name.clone());
    let switch_btn = Switch::new();
    switch_btn.set_active(answers.borrow().get(name).unwrap().as_bool().unwrap());
    switch_btn.set_halign(Align::Center);
    switch_btn.connect_property_active_notify(clone!(@strong question_name => move |switch_btn| {
        answers.borrow_mut().insert(question_name.to_string(), eValue::Bool(switch_btn.get_active()));
    }));
    switch_btn
}
