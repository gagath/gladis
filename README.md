# gladis

[![Latest version](https://img.shields.io/crates/v/gladis.svg)](https://crates.io/crates/gladis)
[![Documentation](https://docs.rs/gladis/badge.svg)](https://docs.rs/gladis)
[![License](https://img.shields.io/crates/l/gladis.svg)](https://crates.io/crates/gladis)

A Rust crate to easily import Glade-generated UI files into Rust code.

## Example

Without Gladis, you would have to manually parse each of the Glade entries like described in the [official Gtk-rs Glade tutorial](https://gtk-rs.org/docs-src/tutorial/glade):

```rust
fn build_ui(app: &gtk::Application) {
    let builder = gtk::Builder::from_string(include_str!("main.glade"));

    let window: gtk::Window = builder
        .get_object("window")
        .expect("could not find window");
    let my_label: gtk::Label = builder
        .get_object("my_label")
        .expect("could not find my_label");

    my_label.set_label("hello from Rust!");
    window.set_application(Some(app));
    window.show_all();
}
```

With Gladis, this part can be automated by declaring a struct that describes the elements to extract from the `.glade` file:

```rust
use gladis::Gladis;
use gladis_proc_macro::Gladis;

#[derive(Gladis, Clone)]
struct Ui {
    window: gtk::Window,
    my_label: gtk::Label,
}

fn build_ui(app: &gtk::Application) {
    let ui = Ui::from_glade_src(include_str!("main.glade"));

    ui.label.set_label("hello from Rust!");
    ui.window.set_application(Some(app));
    ui.window.show_all();
}
```

This is possible thanks to the [gladis_proc_macro](https://crates.io/crates/gladis_proc_macro) package (as this module is quite dumb and only declares the Gladis trait).

# License

Apache v2.
