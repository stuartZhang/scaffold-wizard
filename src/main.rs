use ::gtk::prelude::*;
use ::gio::prelude::*;
use ::gtk::{Application, ApplicationWindow, Button};
fn main() {
    let application = Application::new(
        Some("my.demo1"),
        Default::default(),
    ).expect("GTK 绑定失败");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Hello World 窗体标题");
        window.set_default_size(350, 70);

        let button = Button::with_label("点击【Hello World】!");
        button.connect_clicked(|_| {
            println!("点了！");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
