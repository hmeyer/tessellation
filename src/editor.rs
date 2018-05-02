use gtk::Inhibit;
use gtk::traits::*;
use implicit3d;
use mesh_view;
use na;
use object_widget;
use settings;
use sourceview::{BufferExt, LanguageManagerExt, StyleSchemeManagerExt};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use truescad_luascad;
use truescad_tessellation::{ImplicitFunction, ManifoldDualContouring, Mesh};
use truescad_types::Float;

#[derive(Clone)]
pub struct Editor {
    pub widget: ::gtk::ScrolledWindow,
    source_view: ::sourceview::View,
    buffer: Option<::sourceview::Buffer>,
}

struct ObjectAdapter<S> {
    implicit: Box<implicit3d::Object<S>>,
    resolution: S,
}

impl<S: ::std::fmt::Debug + na::Real + ::num_traits::Float + From<f32>> ImplicitFunction<S>
    for ObjectAdapter<S> {
    fn bbox(&self) -> &::implicit3d::BoundingBox<S> {
        self.implicit.bbox()
    }
    fn value(&self, p: na::Point3<S>) -> S {
        self.implicit.approx_value(p, self.resolution)
    }
    fn normal(&self, p: na::Point3<S>) -> na::Vector3<S> {
        self.implicit.normal(p)
    }
}

impl Editor {
    pub fn new(xw: &object_widget::ObjectWidget, debug_buffer: &::gtk::TextBuffer) -> Editor {
        let widget = ::gtk::ScrolledWindow::new(None, None);
        let mut buffer = None;
        let mut src_view = ::sourceview::View::new();
        if let Some(lang_mgr) = ::sourceview::LanguageManager::get_default() {
            let lang_search_paths = lang_mgr.get_search_path();
            let mut lang_search_paths_str: Vec<&str> =
                lang_search_paths.iter().map(AsRef::as_ref).collect();
            lang_search_paths_str.push("./language-specs/");
            lang_mgr.set_search_path(&lang_search_paths_str);
            if let Some(lua) = lang_mgr.get_language("truescad-lua") {
                if let Some(style_mgr) = ::sourceview::StyleSchemeManager::get_default() {
                    style_mgr.append_search_path("./styles/");
                    if let Some(scheme) = style_mgr.get_scheme("build") {
                        let b = ::sourceview::Buffer::new_with_language(&lua);
                        b.set_highlight_syntax(true);
                        b.set_style_scheme(&scheme);
                        src_view = ::sourceview::View::new_with_buffer(&b);
                        buffer = Some(b);
                    } else {
                        println!("failed to get scheme.");
                    }
                } else {
                    println!("failed to get default StyleSchemeManager.");
                }
            } else {
                println!("failed to get lang.");
            }
        } else {
            println!("failed to get default LanguageManager.");
        }


        widget.add(&src_view);
        // TODO: Find out why this causes a non-draw on startup.
        // tv.set_wrap_mode(::gtk::WrapMode::WordChar);
        let renderer = xw.renderer.clone();
        let drawing_area = xw.drawing_area.clone();
        let debug_buffer_clone = debug_buffer.clone();
        let editor = Editor {
            widget: widget,
            source_view: src_view,
            buffer: buffer,
        };
        let editor_clone = editor.clone();

        editor.source_view.connect_key_release_event(
            move |_: &::sourceview::View, key: &::gdk::EventKey| -> Inhibit {
                match key.get_keyval() {
                    ::gdk::enums::key::F5 => {
                        // compile
                        let mut output = Vec::new();
                        let obj = editor_clone.get_object(&mut output);
                        debug_buffer_clone.set_text(&String::from_utf8(output).unwrap());
                        renderer.borrow_mut().set_object(obj);
                        drawing_area.queue_draw();
                    }
                    _ => {
                        // println!("unbound key release: {:?}", x);
                    }
                }
                Inhibit(false)
            },
        );
        editor
    }
    fn get_object(&self, msg: &mut Write) -> Option<Box<implicit3d::Object<Float>>> {
        let code_buffer = self.source_view.get_buffer().unwrap();
        let code_text = code_buffer
            .get_text(
                &code_buffer.get_start_iter(),
                &code_buffer.get_end_iter(),
                true,
            )
            .unwrap();
        match truescad_luascad::eval(&code_text) {
            Ok((print_result, maybe_object)) => {
                writeln!(msg, "{}", print_result).unwrap();
                match maybe_object {
                    Some(mut o) => {
                        let s = settings::SettingsData::new();
                        o.set_parameters(&implicit3d::PrimitiveParameters {
                            fade_range: s.fade_range,
                            r_multiplier: s.r_multiplier,
                        });
                        Some(o)
                    }
                    None => {
                        writeln!(msg, "\nwarning : no object - did you call build()?").unwrap();
                        None
                    }
                }
            }
            Err(x) => {
                writeln!(msg, "\nerror : {:?}", x).unwrap();
                None
            }
        }
    }
    pub fn open(&self, filename: &str) {
        let open_result = File::open(&filename);
        if let Ok(f) = open_result {
            let reader = BufReader::new(f);
            let mut buffer = String::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    buffer.push_str(&line);
                    buffer.push_str("\n");
                }
            }
            self.source_view.get_buffer().unwrap().set_text(&buffer);
        } else {
            println!("could not open {:?}: {:?}", &filename, open_result);
        }
    }
    pub fn save(&self, filename: &str) {
        save_from_sourceview(&self.source_view, filename);
    }
    pub fn tessellate(&self) -> Option<Mesh<Float>> {
        let maybe_obj = self.get_object(&mut ::std::io::stdout());
        if let Some(obj) = maybe_obj {
            let s = settings::SettingsData::new();
            let adapter = ObjectAdapter {
                implicit: obj,
                resolution: s.tessellation_resolution,
            };

            let mesh = ManifoldDualContouring::new(
                &adapter,
                s.tessellation_resolution,
                s.tessellation_error,
            ).tessellate();
            if let Some(ref mesh) = mesh {
                mesh_view::show_mesh(&mesh);
            }
            return mesh;
        }
        return None;
    }
}

fn save_from_sourceview(source_view: &::sourceview::View, filename: &str) {
    let open_result = File::create(filename);
    if let Ok(f) = open_result {
        let code_buffer = source_view.get_buffer().unwrap();
        let code_text = code_buffer
            .get_text(
                &code_buffer.get_start_iter(),
                &code_buffer.get_end_iter(),
                true,
            )
            .unwrap();
        let mut writer = BufWriter::new(f);
        let write_result = writer.write(code_text.as_bytes());
        println!("writing {:?}: {:?}", &filename, write_result);
    } else {
        println!(
            "opening for write {:?} failed: {:?}",
            &filename,
            open_result
        );
    }
}
