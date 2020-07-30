//! Easily import Glade-generated UI files into Rust code.
//!
//! ```
//! use gtk::prelude::*;
//! use gladis::Gladis;
//!
//! const GLADE_SRC: &str = r#"
//! <?xml version="1.0" encoding="UTF-8"?>
//! <!-- Generated with glade 3.22.2 -->
//! <interface>
//!   <requires lib="gtk+" version="3.20"/>
//!   <object class="GtkApplicationWindow" id="window">
//!     <property name="can_focus">False</property>
//!     <child type="titlebar">
//!       <placeholder/>
//!     </child>
//!     <child>
//!       <object class="GtkLabel" id="label">
//!         <property name="visible">True</property>
//!         <property name="can_focus">False</property>
//!         <property name="label" translatable="yes">label</property>
//!       </object>
//!     </child>
//!   </object>
//! </interface>"#;
//!
//! #[derive(Gladis, Clone)]
//! pub struct Window {
//!     pub window: gtk::ApplicationWindow,
//!     pub label: gtk::Label,
//! }
//!
//! fn main() {
//!     gtk::init().unwrap();
//!     let _ui = Window::from_string(GLADE_SRC);
//! }
//! ```

use gtk;

pub trait Gladis {
    //! A trait to load a struct from a builder.
    //!
    //! # Automatic implementation
    //!
    //! This trait wakes little sense alone, but truly show its power when used
    //! with the [gladis_proc_macro](https://docs.rs/gladis_proc_macro) crate
    //! and its `#[derive(Gladis)]` macro.
    //!
    //! ```
    //! use gtk::prelude::*;
    //! use gladis::Gladis;
    //!
    //! #[derive(Gladis, Clone)]
    //! pub struct Window {
    //!     pub window: gtk::ApplicationWindow,
    //!     pub label: gtk::Label,
    //! }
    //! ```
    //!
    //! # Manual implementation
    //!
    //! Below is an example of manual implementation of the trait.
    //!
    //! ```
    //! use gtk::prelude::*;
    //! use gladis::Gladis;
    //!
    //! pub struct Window {
    //!     pub window: gtk::ApplicationWindow,
    //!     pub label: gtk::Label,
    //! }
    //!
    //! impl Gladis for Window {
    //!     fn from_builder(builder: gtk::Builder) -> Self {
    //!         let window: gtk::ApplicationWindow = builder
    //!             .get_object("window")
    //!             .expect("Failed to find the window object");
    //!
    //!         let label: gtk::Label = builder
    //!             .get_object("label")
    //!             .expect("Failed to find the label object");
    //!
    //!         Self { window, label }
    //!     }
    //!
    //!     fn from_string(src: &str) -> Self {
    //!         let builder = gtk::Builder::from_string(src);
    //!         Gladis::from_builder(builder)
    //!     }
    //!
    //!     fn from_resource(resource_path: &str) -> Self {
    //!         let builder = gtk::Builder::from_resource(resource_path);
    //!         Gladis::from_builder(builder)
    //!     }
    //! }
    //! ```

    /// Populate struct from a builder.
    ///
    /// This method should not be called directly but is used as a common
    /// function for the `from_string` and `from_resource` functions to
    /// share the same code.
    fn from_builder(builder: gtk::Builder) -> Self;

    /// Populate struct from a Glade document.
    fn from_string(src: &str) -> Self
    where
        Self: std::marker::Sized,
    {
        let builder = gtk::Builder::from_string(src);
        Gladis::from_builder(builder)
    }

    /// Populate struct from a Glade document as a resource.
    fn from_resource(resource_path: &str) -> Self
    where
        Self: std::marker::Sized,
    {
        let builder = gtk::Builder::from_resource(resource_path);
        Gladis::from_builder(builder)
    }
}

// Re-export #[derive(Gladis)].
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use gladis_proc_macro::Gladis;
