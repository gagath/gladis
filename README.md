<!--
SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# gladis

![Maintenance status is "deprecated"](https://img.shields.io/badge/maintenance-deprecated-red.svg)
[![Build](https://github.com/MicroJoe/gladis/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/MicroJoe/gladis/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/gladis.svg)](https://crates.io/crates/gladis)
[![Documentation](https://docs.rs/gladis/badge.svg)](https://docs.rs/gladis)
[![License](https://img.shields.io/crates/l/gladis.svg)](https://crates.io/crates/gladis)
[![REUSE status](https://api.reuse.software/badge/github.com/MicroJoe/gladis)](https://api.reuse.software/info/github.com/MicroJoe/gladis)

Easily import Glade-generated UI files into Rust code.

**This crate is DEPRECATED.  
Use [CompositeTemplate](https://gtk-rs.org/gtk4-rs/stable/latest/book/composite_templates.html)
from [gtk3-macros](https://crates.io/crates/gtk3-macros)
or [gtk4-macros](https://crates.io/crates/gtk4-macros) official GTK crates.**

## Usage

In order to use Gladis, you have to add the following dependencies into your
project's `Cargo.toml` file:

```toml
[dependencies]
gladis = "2.0.0"
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
        Self::from_resource("/dev/null/hello_builder/window.ui").unwrap()
    }
}
```

Without Gladis, you would have to manually parse each of the Glade entries.

```rust
pub struct Window {
    pub window: gtk::ApplicationWindow,
    pub label: gtk::Label,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/dev/null/hello_builder/window.ui");
        let window: gtk::ApplicationWindow = builder
            .object("window")
            .expect("Failed to find the window object");

        let label: gtk::Label = builder
            .object("label")
            .expect("Failed to find the label object");

        Self { window, label }
    }
}
```

## Relm support

This crate is compatible with [Relm](https://github.com/antoyo/relm), a
popular framework for writing UIs with GTK+. See the `examples/relm` directory,
and give it a shot!

## License

Licensed under either of [Apache License, Version 2.0](LICENSES/Apache-2.0.txt)
or [MIT license](LICENSES/MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
