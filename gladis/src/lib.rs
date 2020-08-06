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
//!     let _ui = Window::from_string(GLADE_SRC).unwrap();
//! }
//! ```

use gtk;

#[derive(Debug, Clone)]
pub struct NotFoundError {
    pub identifier: String,
    pub typ: String,
}

impl Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Identified {} of type {} was not found",
            self.identifier, self.typ
        )
    }
}

impl Error for NotFoundError {}

#[derive(Debug, Clone)]
pub enum GladisError {
    NotFound(NotFoundError),
}

impl Display for GladisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GladisError::NotFound(e) => write!(f, "Not found error: {}", e),
        }
    }
}

impl Error for GladisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GladisError::NotFound(e) => Some(e),
        }
    }
}

impl GladisError {
    pub fn not_found<T>(identifier: T, typ: T) -> Self
    where
        T: ToString,
    {
        let identifier = identifier.to_string();
        let typ = typ.to_string();
        GladisError::NotFound(NotFoundError { identifier, typ })
    }
}

pub type Result<T> = std::result::Result<T, GladisError>;

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
    //! use gladis::{Gladis, Result, GladisError};
    //!
    //! pub struct Window {
    //!     pub window: gtk::ApplicationWindow,
    //!     pub label: gtk::Label,
    //! }
    //!
    //! impl Gladis for Window {
    //!     fn from_builder(builder: gtk::Builder) -> Result<Self> {
    //!         let window: gtk::ApplicationWindow = builder
    //!             .get_object("window")
    //!             .ok_or(GladisError::not_found("window", "gtk::ApplicationWindow"))?;
    //!
    //!         let label: gtk::Label = builder
    //!             .get_object("label")
    //!             .ok_or(GladisError::not_found("label", "gtk::Label"))?;
    //!
    //!         Ok(Self { window, label })
    //!     }
    //! }
    //! ```

    /// Populate struct from a builder.
    ///
    /// This method should not be called directly but is used as a common
    /// function for the `from_string` and `from_resource` functions to
    /// share the same code.
    fn from_builder(builder: gtk::Builder) -> Result<Self>
    where
        Self: std::marker::Sized;

    /// Populate struct from a Glade document.
    fn from_string(src: &str) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let builder = gtk::Builder::from_string(src);
        Gladis::from_builder(builder)
    }

    /// Populate struct from a Glade document as a resource.
    fn from_resource(resource_path: &str) -> Result<Self>
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
use std::{error::Error, fmt::Display};
