use ::glib::clone;
use ::gtk::{Builder, Button, Dialog, Label, prelude::*};

pub fn show(builder: &Builder, message: &str) {
    let invalid_dialog: Dialog = builder.get_object("error-dialog")
        .expect("不能从 glade 布局文件里，找到 error-dialog 元素");
    invalid_dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        gtk::Inhibit(true)
    });
    let message_label: Label = builder.get_object("error-dialog-message-label")
        .expect("不能从 glade 布局文件里，找到 error-dialog-message-label 元素");
    message_label.set_text(message);
    let ok_button: Button = builder.get_object("error-dialog-ok-button")
        .expect("不能从 glade 布局文件里，找到 error-dialog-ok-button 元素");
    ok_button.connect_clicked(clone!(@weak invalid_dialog => move |_| {
        invalid_dialog.hide();
    }));
    invalid_dialog.show_all();
}
