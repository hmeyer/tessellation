use hlua;
use lobject::LObject;
use truescad_primitive::{Intersection, Object, Union};
use truescad_types::Float;

// Struct to be used to construct boolean Objects.
// The lua helpers below pump LObjects from Lua Arrays into this LObjectVector, which is then used
// to construct the boolean Objects.
pub struct LObjectVector {
    pub v: Option<Vec<Box<Object>>>,
}


// this macro implements the required trait so that we can *push* the object to lua
// (ie. move it inside lua)
implement_lua_push!(LObjectVector, |mut metatable| {
    // we create a `__index` entry in the metatable
    let mut index = metatable.empty_array("__index");
    index.set(
        "push",
        ::hlua::function2(|v: &mut LObjectVector, o: &mut LObject| {
            v.push(o.into_object());
        }),
    );
});

// this macro implements the require traits so that we can *read* the object back
implement_lua_read!(LObjectVector);


impl LObjectVector {
    pub fn new(o: Option<Box<Object>>) -> LObjectVector {
        LObjectVector {
            v: if let Some(o) = o { Some(vec![o]) } else { None },
        }
    }
    pub fn export_factories(lua: &mut hlua::Lua, env_name: &str) {
        lua.set(
            "__new_object_vector",
            hlua::function1(|o: &LObject| LObjectVector::new(o.into_object())),
        );
        lua.set(
            "__new_union",
            hlua::function2(|o: &LObjectVector, smooth: Float| {
                LObject {
                    o: if let Some(ref v) = o.v {
                        Some(Union::from_vec(v.clone(), smooth).unwrap() as Box<Object>)
                    } else {
                        None
                    },
                }
            }),
        );
        lua.set(
            "__new_intersection",
            hlua::function2(|o: &LObjectVector, smooth: Float| {
                LObject {
                    o: if let Some(ref v) = o.v {
                        Some(Intersection::from_vec(v.clone(), smooth).unwrap()
                            as Box<Object>)
                    } else {
                        None
                    },
                }
            }),
        );
        lua.set(
            "__new_difference",
            hlua::function2(|o: &LObjectVector, smooth: Float| {
                LObject {
                    o: if let Some(ref v) = o.v {
                        Some(
                            Intersection::difference_from_vec(v.clone(), smooth).unwrap()
                                as Box<Object>,
                        )
                    } else {
                        None
                    },
                }
            }),
        );
        lua.execute::<()>(&format!(
            "
            function __array_to_ov(lobjects)
              ov = __new_object_vector(lobjects[1])
              for i = 2, #lobjects do
                ov:push(lobjects[i])
              end
              return ov
            end

            function Union(lobjects, smooth)
              smooth = smooth or 0
              return __new_union(__array_to_ov(lobjects), smooth)
            end

            function Intersection(lobjects, smooth)
              smooth = smooth or 0
              return __new_intersection(__array_to_ov(lobjects), smooth)
            end

            function Difference(lobjects, smooth)
              smooth = smooth or 0
              return __new_difference(__array_to_ov(lobjects), smooth)
            end

            {env}.Union = Union;
            {env}.Intersection = Intersection;
            {env}.Difference = Difference;",
            env = env_name
        )).unwrap();
    }
    pub fn push(&mut self, o: Option<Box<Object>>) {
        if let Some(o) = o {
            if let Some(ref mut v) = self.v {
                v.push(o);
            }
        } else {
            self.v = None
        }
    }
}
