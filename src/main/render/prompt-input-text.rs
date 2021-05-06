use ::eval::Value as eValue;
use ::gdk_pixbuf::Pixbuf;
use ::glib::clone;
use ::gtk::{Entry, EntryIconPosition, prelude::*,};
use ::serde_json::{map::Map, Value as jValue};
use ::std::{path::Path, cell::RefCell, rc::Rc};
use crate::main::utils;

pub fn build_ui((name, json): (&String, &jValue), answers: Rc<RefCell<Map<String, eValue>>>, bin_dir: Option<&Path>) -> Entry {
    if answers.borrow().get(name).is_none() {
        let text = json["default"].as_str().unwrap_or_default();
        answers.borrow_mut().insert(name.clone(), eValue::String(text.to_string()));
    }
    let question_name = Rc::new(name.clone());
    let input_str = Entry::new();
    let bin_dir = if let Some(bin_dir) = bin_dir {
        Some(bin_dir.to_path_buf())
    } else if let Ok(bin_dir) = utils::get_exe_dir() {
        Some(bin_dir)
    } else {
        None
    };
    if let Some(mut icon_path) = bin_dir {
        icon_path.push(Path::new("../assets/images/dialog-question.png"));
        if let Ok(pix_buf) = Pixbuf::from_file(icon_path) {
            input_str.set_icon_from_pixbuf(EntryIconPosition::Primary, Some(&pix_buf));
        }
    }
    input_str.set_placeholder_text(Some("请输入..."));
    input_str.set_text(answers.borrow().get(name).unwrap().as_str().unwrap());
    input_str.set_property_width_request(200);
    input_str.connect_property_text_notify(clone!(@strong answers, @strong question_name => move |input_str| {
        answers.borrow_mut().insert(question_name.to_string(), eValue::String(input_str.get_text().to_string()));
    }));
    input_str
}
