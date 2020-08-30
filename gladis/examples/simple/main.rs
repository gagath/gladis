use std::env::args;

use gtk::prelude::*;
use gio::prelude::*;

use gladis::Gladis;

#[derive(Gladis, Clone)]
struct Ui {
    window: gtk::Window,
    button: gtk::Button,
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("simple.glade");
    let ui = Ui::from_string(glade_src).unwrap();

    ui.window.set_application(Some(app));

    ui.button.connect_clicked(move |b| {
        b.set_label("clicked!");
    });

    ui.window.show_all();
}

fn main() {

    let application = gtk::Application::new(
        Some("com.github.MicroJoe.gladis.examples.simple"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}