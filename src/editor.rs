use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use mesh_view;
use openscad;
use xplicit_primitive;
use xplicit_widget;
use settings;
use gtk::Inhibit;
use gtk::traits::*;
use xplicit_tessellation::{ManifoldDualContouring, write_stl};

#[derive(Clone)]
pub struct Editor {
    pub widget: ::gtk::ScrolledWindow,
    text_view: ::gtk::TextView,
}


impl Editor {
    pub fn new(xw: &xplicit_widget::XplicitWidget, debug_buffer: &::gtk::TextBuffer) -> Editor {
        let widget = ::gtk::ScrolledWindow::new(None, None);
        let tv = ::gtk::TextView::new();
        widget.add(&tv);
        // TODO: Find out why this causes a non-draw on startup.
        // tv.set_wrap_mode(::gtk::WrapMode::WordChar);
        let renderer = xw.renderer.clone();
        let drawing_area = xw.drawing_area.clone();
        let debug_buffer_clone = debug_buffer.clone();
        let editor = Editor {
            widget: widget,
            text_view: tv,
        };
        let editor_clone = editor.clone();

        editor.text_view.connect_key_release_event(move |_: &::gtk::TextView,
                                                         key: &::gdk::EventKey|
                                                         -> Inhibit {
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
        });
        editor
    }
    fn get_object(&self, msg: &mut Write) -> Option<Box<xplicit_primitive::Object>> {
        let code_buffer = self.text_view.get_buffer().unwrap();
        let code_text = code_buffer.get_text(&code_buffer.get_start_iter(),
                                             &code_buffer.get_end_iter(),
                                             true)
                                   .unwrap();
        let maybe_pgm = openscad::program(&code_text);
        if let Ok(pgm) = maybe_pgm {
            writeln!(msg, "\nparsed : {:?}", pgm).unwrap();
            let mut env = openscad::ast::Environment::new();
            let ast_value = pgm.eval(&mut env, msg);
            writeln!(msg, "\nexecuted : {:?}", ast_value).unwrap();
            if let openscad::ast::Value::Objects(objs) = ast_value {
                if objs.len() > 0 {
                    let mut result_union = xplicit_primitive::Union::from_vec(objs, 0.).unwrap();
                    let s = settings::SettingsData::new();
                    result_union.set_parameters(&xplicit_primitive::PrimitiveParameters {
                        fade_range: s.fade_range,
                        r_multiplier: s.r_multiplier,
                    });
                    return Some(result_union);
                }
            }
        } else {
            writeln!(msg, "{:?}", maybe_pgm).unwrap()
        }
        None
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
            self.text_view.get_buffer().unwrap().set_text(&buffer);
        } else {
            println!("could not open {:?}: {:?}", &filename, open_result);
        }
    }
    pub fn save(&self, filename: &str) {
        save_from_textview(&self.text_view, filename);
    }
    pub fn tessellate(&self) {
        let maybe_obj = self.get_object(&mut ::std::io::stdout());
        if let Some(obj) = maybe_obj {
            let s = settings::SettingsData::new();
            let mesh = ManifoldDualContouring::new(obj,
                                                   s.tessellation_resolution,
                                                   s.tessellation_error)
                           .tessellate();
            println!("Writing xplicit.stl: {:?}", write_stl("xplicit.stl", &mesh));
            mesh_view::show_mesh(&mesh);
        }
    }
}

fn save_from_textview(text_view: &::gtk::TextView, filename: &str) {
    let open_result = File::create(filename);
    if let Ok(f) = open_result {
        let code_buffer = text_view.get_buffer().unwrap();
        let code_text = code_buffer.get_text(&code_buffer.get_start_iter(),
                                             &code_buffer.get_end_iter(),
                                             true)
                                   .unwrap();
        let mut writer = BufWriter::new(f);
        let write_result = writer.write(code_text.as_bytes());
        println!("writing {:?}: {:?}", &filename, write_result);
    } else {
        println!("opening for write {:?} failed: {:?}",
                 &filename,
                 open_result);
    }
}
