extern crate alga;
extern crate bbox;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate num_traits;
extern crate rand;
extern crate rayon;
extern crate time;


use alga::general::Real;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Plane<S: 'static + Real + Debug> {
    pub p: na::Point3<S>,
    pub n: na::Vector3<S>,
}

mod bitset;
mod vertex_index;
mod manifold_dual_contouring;
mod cell_configs;
mod qef;

pub use self::manifold_dual_contouring::ManifoldDualContouring;
// This is just exposed for the bench test - do not use!
pub use self::manifold_dual_contouring::ManifoldDualContouringImpl;
pub use self::manifold_dual_contouring::subsample_octtree;


pub trait ImplicitFunction<S: Debug + Real> {
    fn bbox(&self) -> &bbox::BoundingBox<S>;
    fn value(&self, p: na::Point3<S>) -> S;
    fn normal(&self, p: na::Point3<S>) -> na::Vector3<S>;
}

pub trait CeilAsUSize: ::num_traits::Float {
    fn ceil_as_usize(self) -> usize;
}

impl CeilAsUSize for f32 {
    fn ceil_as_usize(self) -> usize {
        self.ceil() as usize
    }
}

impl CeilAsUSize for f64 {
    fn ceil_as_usize(self) -> usize {
        self.ceil() as usize
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh<S> {
    pub vertices: Vec<[S; 3]>,
    pub faces: Vec<[usize; 3]>,
}

impl<S: 'static + Real + Debug> Mesh<S> {
    pub fn normal32(&self, face: usize) -> [f32; 3]
    where
        f64: std::convert::From<S>,
    {
        let v: Vec<na::Point3<f32>> = self.faces[face]
            .iter()
            .map(|&i| {
                let v: (f64, f64, f64) = (
                    self.vertices[i][0].into(),
                    self.vertices[i][1].into(),
                    self.vertices[i][2].into(),
                );
                na::Point3::<f32>::new(v.0 as f32, v.1 as f32, v.2 as f32)
            })
            .collect();
        let r = (v[1] - v[0]).cross(&(v[2] - v[0])).normalize();
        [r[0], r[1], r[2]]
    }
    pub fn vertex32(&self, i: usize) -> [f32; 3]
    where
        f64: std::convert::From<S>,
    {
        let v: (f64, f64, f64) = (
            self.vertices[i][0].into(),
            self.vertices[i][1].into(),
            self.vertices[i][2].into(),
        );
        [v.0 as f32, v.1 as f32, v.2 as f32]
    }
}

#[cfg(test)]
#[macro_use]
extern crate approx;
