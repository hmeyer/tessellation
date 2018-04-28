// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]

extern crate alga;
extern crate cairo;
extern crate gdk;
extern crate gtk;
extern crate implicit3d;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate nalgebra;
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sourceview;
extern crate stl_io;
extern crate toml;
extern crate truescad_luascad;
extern crate truescad_tessellation;
extern crate truescad_types;

pub mod render;
pub mod object_widget;
pub mod editor;
pub mod menu;
pub mod mesh_view;
pub mod settings;
pub mod window;
