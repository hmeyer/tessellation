use hlua;
use std::sync::mpsc;
use truescad_bbox::BoundingBox;
use truescad_primitive::{Bender, Cone, Cylinder, Intersection, Mesh, Object, SlabZ, Sphere,
                         Twister};
use truescad_types::{Float, Point, Vector, INFINITY, NEG_INFINITY};

#[derive(Clone, Debug)]
pub struct LObject {
    pub o: Option<Box<Object>>,
}


// this macro implements the required trait so that we can *push* the object to lua
// (ie. move it inside lua)
implement_lua_push!(LObject, |mut metatable| {
    {
        // we create a `__index` entry in the metatable
        // when the lua code calls `object:translate()`, it will look for `translate` in there
        let mut index = metatable.empty_array("__index");

        index.set(
            "translate",
            ::hlua::function4(|o: &mut LObject, x: Float, y: Float, z: Float| {
                o.translate(x, y, z)
            }),
        );
        index.set(
            "rotate",
            ::hlua::function4(|o: &mut LObject, x: Float, y: Float, z: Float| {
                o.rotate(x, y, z)
            }),
        );
        index.set(
            "scale",
            ::hlua::function4(|o: &mut LObject, x: Float, y: Float, z: Float| {
                o.scale(x, y, z)
            }),
        );
        index.set("clone", ::hlua::function1(|o: &mut LObject| o.clone()));
    }
    // Add __tostring metamethod for printing LObjects.
    metatable.set(
        "__tostring",
        ::hlua::function1(|o: &mut LObject| format!("{:#?}", o)),
    );
});

// this macro implements the require traits so that we can *read* the object back
implement_lua_read!(LObject);


impl LObject {
    pub fn into_object(&self) -> Option<Box<Object>> {
        self.o.clone()
    }
    pub fn export_factories<'a, L>(env: &mut hlua::LuaTable<L>, console: mpsc::Sender<String>)
    where
        L: hlua::AsMutLua<'a>,
    {
        env.set(
            "Box",
            hlua::function4(
                |x: Float, y: Float, z: Float, smooth_lua: hlua::AnyLuaValue| {
                    let mut smooth = 0.;
                    if let hlua::AnyLuaValue::LuaNumber(v) = smooth_lua {
                        smooth = v;
                    }
                    LObject {
                        o: Some(Intersection::from_vec(
                            vec![
                                ::truescad_primitive::SlabX::new(x),
                                ::truescad_primitive::SlabY::new(y),
                                ::truescad_primitive::SlabZ::new(z),
                            ],
                            smooth,
                        ).unwrap() as Box<Object>),
                    }
                },
            ),
        );
        env.set(
            "Sphere",
            hlua::function1(|radius: Float| {
                LObject {
                    o: Some(Sphere::new(radius) as Box<Object>),
                }
            }),
        );
        env.set(
            "iCylinder",
            hlua::function1(|radius: Float| {
                LObject {
                    o: Some(Cylinder::new(radius) as Box<Object>),
                }
            }),
        );
        env.set(
            "iCone",
            hlua::function1(|slope: Float| {
                LObject {
                    o: Some(Cone::new(slope, 0.) as Box<Object>),
                }
            }),
        );
        env.set(
            "Cylinder",
            hlua::function4(
                |length: Float,
                 radius1: Float,
                 radius2_lua: hlua::AnyLuaValue,
                 smooth_lua: hlua::AnyLuaValue| {
                    let mut radius2 = radius1;
                    let mut smooth = 0.;
                    if let hlua::AnyLuaValue::LuaNumber(v) = radius2_lua {
                        radius2 = v;
                        if let hlua::AnyLuaValue::LuaNumber(v) = smooth_lua {
                            smooth = v;
                        }
                    }
                    let mut conie;
                    if radius1 == radius2 {
                        conie = Cylinder::new(radius1) as Box<Object>;
                    } else {
                        let slope = (radius2 - radius1).abs() / length;
                        let offset;
                        if radius1 < radius2 {
                            offset = -radius1 / slope - length * 0.5;
                        } else {
                            offset = radius2 / slope + length * 0.5;
                        }
                        conie = Cone::new(slope, offset) as Box<Object>;
                        let rmax = radius1.max(radius2);
                        let conie_box = BoundingBox::new(
                            Point::new(-rmax, -rmax, NEG_INFINITY),
                            Point::new(rmax, rmax, INFINITY),
                        );
                        conie.set_bbox(conie_box);
                    }
                    LObject {
                        o: Some(
                            Intersection::from_vec(vec![conie, SlabZ::new(length)], smooth).unwrap()
                                as Box<Object>,
                        ),
                    }
                },
            ),
        );
        env.set(
            "Bend",
            hlua::function2(|o: &LObject, width: Float| {
                LObject {
                    o: if let Some(obj) = o.into_object() {
                        Some(Bender::new(obj, width) as Box<Object>)
                    } else {
                        None
                    },
                }
            }),
        );
        env.set(
            "Twist",
            hlua::function2(|o: &LObject, height: Float| {
                LObject {
                    o: if let Some(obj) = o.into_object() {
                        Some(Twister::new(obj, height) as Box<Object>)
                    } else {
                        None
                    },
                }
            }),
        );
        env.set(
            "Mesh",
            hlua::function1(move |filename: String| {
                LObject {
                    o: match Mesh::new(&filename) {
                        Ok(mesh) => {
                            console
                                .send(
                                    "Warning: Mesh support is currently horribly inefficient!"
                                        .to_string(),
                                )
                                .unwrap();
                            Some(mesh as Box<Object>)
                        }
                        Err(e) => {
                            console
                                .send(format!("Could not read mesh: {:}", e))
                                .unwrap();
                            None
                        }
                    },
                }
            }),
        );
    }
    fn translate(&mut self, x: Float, y: Float, z: Float) -> LObject {
        LObject {
            o: if let Some(ref obj) = self.o {
                Some(obj.clone().translate(Vector::new(x, y, z)))
            } else {
                None
            },
        }
    }
    fn rotate(&mut self, x: Float, y: Float, z: Float) -> LObject {
        LObject {
            o: if let Some(ref obj) = self.o {
                Some(obj.clone().rotate(Vector::new(x, y, z)))
            } else {
                None
            },
        }
    }
    fn scale(&mut self, x: Float, y: Float, z: Float) -> LObject {
        LObject {
            o: if let Some(ref obj) = self.o {
                Some(obj.clone().scale(Vector::new(x, y, z)))
            } else {
                None
            },
        }
    }
}
