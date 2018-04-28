extern crate alga;
#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate bbox;
extern crate nalgebra as na;
extern crate num_traits;
extern crate stl_io;
use alga::general::Real;
use bbox::BoundingBox;
use num_traits::Float;
use std::fmt::Debug;

mod transformer;
pub use self::transformer::AffineTransformer;

mod twister;
pub use self::twister::Twister;

mod bender;
pub use self::bender::Bender;

mod boolean;
pub use self::boolean::{Intersection, Union};

mod sphere;
pub use self::sphere::Sphere;

mod cylinder;
pub use self::cylinder::{Cone, Cylinder};

mod slab;
pub use self::slab::{SlabX, SlabY, SlabZ};

mod mesh;
pub use self::mesh::Mesh;

pub struct PrimitiveParameters<S> {
    pub fade_range: S,
    pub r_multiplier: S,
}

pub const ALWAYS_PRECISE: f32 = 1.;
pub const EPSILON: f32 = 1e-10;


pub fn normal_from_object<S: Debug + Real + Float + From<f32>>(
    f: &Object<S>,
    p: na::Point3<S>,
) -> na::Vector3<S> {
    let null: S = From::from(0.0);
    let e: S = From::from(EPSILON);
    let a: S = From::from(ALWAYS_PRECISE);
    let epsilon_x = na::Vector3::<S>::new(e, null, null);
    let epsilon_y = na::Vector3::<S>::new(null, e, null);
    let epsilon_z = na::Vector3::<S>::new(null, null, e);
    let center = f.approx_value(p, a);
    let dx = f.approx_value(&p + epsilon_x, a) - center;
    let dy = f.approx_value(&p + epsilon_y, a) - center;
    let dz = f.approx_value(&p + epsilon_z, a) - center;
    na::Vector3::<S>::new(dx, dy, dz).normalize()
}

pub trait Object<S: Real + Float + From<f32>>
    : ObjectClone<S> + Debug + Sync + Send {
    fn bbox(&self) -> &BoundingBox<S>;
    fn set_bbox(&mut self, _: BoundingBox<S>) {
        unimplemented!();
    }
    fn set_parameters(&mut self, _: &PrimitiveParameters<S>) {}
    // Value is 0 on object surfaces, negative inside and positive outside of objects.
    // If positive, value is guarateed to be the minimum distance to the object surface.
    // return some approximation (which is always larger then the proper value).
    // Only do a proper calculation, for values smaller then slack.
    fn approx_value(&self, _: na::Point3<S>, _: S) -> S {
        unimplemented!();
    }
    fn normal(&self, _: na::Point3<S>) -> na::Vector3<S> {
        unimplemented!();
    }
    fn translate(&self, v: na::Vector3<S>) -> Box<Object<S>> {
        AffineTransformer::new_translate(self.clone_box(), v)
    }
    fn rotate(&self, r: na::Vector3<S>) -> Box<Object<S>> {
        AffineTransformer::new_rotate(self.clone_box(), r)
    }
    fn scale(&self, s: na::Vector3<S>) -> Box<Object<S>> {
        AffineTransformer::new_scale(self.clone_box(), s)
    }
}

pub trait ObjectClone<S> {
    fn clone_box(&self) -> Box<Object<S>>;
}

impl<S: Real + Float + From<f32>, T> ObjectClone<S> for T
where
    T: 'static + Object<S> + Clone,
{
    fn clone_box(&self) -> Box<Object<S>> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl<S> Clone for Box<Object<S>> {
    fn clone(&self) -> Box<Object<S>> {
        self.clone_box()
    }
}

// Objects never equal each other
impl<S> PartialEq for Box<Object<S>> {
    fn eq(&self, _: &Box<Object<S>>) -> bool {
        false
    }
}

// Objects are never ordered
impl<S> PartialOrd for Box<Object<S>> {
    fn partial_cmp(&self, _: &Box<Object<S>>) -> Option<::std::cmp::Ordering> {
        None
    }
}
