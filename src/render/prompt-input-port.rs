use ::eval::Value as eValue;
use ::glib::clone;
use ::gtk::{Adjustment, Align, prelude::*, SpinButton};
use ::serde_json::{json, map::Map, Value as jValue};
use ::std::{cell::RefCell, rc::Rc};

pub fn build_ui((name, json): (&String, &jValue), answers: Rc<RefCell<Map<String, eValue>>>) -> SpinButton {
    if answers.borrow().get(name).is_none() {
        let port = json["default"].as_u64().unwrap_or(9000u64);
        answers.borrow_mut().insert(name.clone(), json!(port));
    }
    let question_name = Rc::new(name.clone());
    let adjustment = Adjustment::new(answers.borrow().get(name).unwrap().as_f64().unwrap(), 1000f64, 99999f64, 1f64, 10f64, 0f64);
    let spin_btn = SpinButton::new(Some(&adjustment), 0f64, 0u32);
    spin_btn.set_numeric(true);
    spin_btn.set_width_chars(5);
    spin_btn.set_max_length(5);
    spin_btn.set_max_width_chars(5);
    spin_btn.set_halign(Align::Center);
    spin_btn.set_property_width_request(150);
    spin_btn.connect_property_value_notify(clone!(@strong question_name => move |spin_btn| {
        answers.borrow_mut().insert(question_name.to_string(), json!(spin_btn.get_value() as u64));
    }));
    spin_btn
}
