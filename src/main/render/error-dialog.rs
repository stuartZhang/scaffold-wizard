use ::glib::clone;
use ::gtk::{Builder, Button, Dialog, Label, prelude::*, ResponseType};
use super::Unwrap;

pub fn show<F>(builder: &Builder, message: F)
where F: AsRef<str> {
    let invalid_dialog: Dialog = Unwrap::option2(builder.get_object("error-dialog"),
        "不能从 glade 布局文件里，找到 error-dialog 元素");
    let message_label: Label = Unwrap::option2(builder.get_object("error-dialog-message-label"),
        "不能从 glade 布局文件里，找到 error-dialog-message-label 元素");
    message_label.set_text(message.as_ref());
    let ok_button: Button = Unwrap::option2(builder.get_object("error-dialog-ok-button"),
        "不能从 glade 布局文件里，找到 error-dialog-ok-button 元素");
    ok_button.connect_clicked(clone!(@weak invalid_dialog => move |_| {
        invalid_dialog.response(ResponseType::DeleteEvent);
    }));
    if ResponseType::DeleteEvent == invalid_dialog.run() {
        invalid_dialog.hide();
    }
}
