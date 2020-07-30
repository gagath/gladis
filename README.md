# gladis

[![Latest version](https://img.shields.io/crates/v/gladis.svg)](https://crates.io/crates/gladis)
[![Documentation](https://docs.rs/gladis/badge.svg)](https://docs.rs/gladis)
[![License](https://img.shields.io/crates/l/gladis.svg)](https://crates.io/crates/gladis)

A Rust crate to easily import Glade-generated UI files into Rust code.

## Usage

In order to use Gladis, you have to add the following dependencies into your
project's `Cargo.toml` file:

```toml
[dependencies]
gladis = "0.3.*"
```

After this is done, you can enjoy the Gladis derive!

```rust
#[derive(Gladis, Clone)]
pub struct Window {
    pub window: gtk::ApplicationWindow,
    pub label: gtk::Label,
}

impl Window {
    pub fn new() -> Self {
        Self::from_resource("/dev/null/hello_builder/window.ui")
    }
}
```

This is possible thanks to the
[gladis_proc_macro](https://crates.io/crates/gladis_proc_macro) package (as
this module is quite dumb and only declares the Gladis trait).

Without Gladis, you would have to manually parse each of the Glade entries like
described in the [official Gtk-rs Glade
tutorial](https://gtk-rs.org/docs-src/tutorial/glade):

```rust
pub struct Window {
    pub window: gtk::ApplicationWindow,
    pub label: gtk::Label,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/dev/null/hello_builder/window.ui");
        let window: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        let label: gtk::Label = builder
            .get_object("label")
            .expect("Failed to find the label object");

        Self { window, label }
    }
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
