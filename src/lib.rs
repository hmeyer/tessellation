//! tessellation is crate for creating polygons from implicit functions or volumes.
//! It uses
//! [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf).
//!
//! # Examples
//!
//! Create a unit sphere and tessellate it:
//!
//! ```rust
//! extern crate nalgebra as na;
//! extern crate tessellation;
//!
//! struct UnitSphere {
//!   bbox : tessellation::BoundingBox<f64>
//! }
//!
//! impl UnitSphere {
//!   fn new() -> UnitSphere {
//!     UnitSphere {
//!       bbox: tessellation::BoundingBox::new(&na::Point3::new(-1., -1., -1.),
//!                                            &na::Point3::new( 1.,  1.,  1.)) }
//!   }
//! }
//!
//! impl tessellation::ImplicitFunction<f64> for UnitSphere {
//!    fn bbox(&self) -> &tessellation::BoundingBox<f64> {
//!      &self.bbox
//!    }
//!   fn value(&self, p: &na::Point3<f64>) -> f64 {
//!     return na::Vector3::new(p.x, p.y, p.z).norm() - 1.0;
//!   }
//!   fn normal(&self, p: &na::Point3<f64>) -> na::Vector3<f64> {
//!     return na::Vector3::new(p.x, p.y, p.z).normalize();
//!   }
//! }
//!
//! let sphere = UnitSphere::new();
//! let mut mdc =  tessellation::ManifoldDualContouring::new(&sphere, 0.2, 0.1);
//! let triangles = mdc.tessellate().unwrap();
//! ```
#![warn(missing_docs)]
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
pub use bbox::BoundingBox;
use std::fmt::Debug;

mod bitset;
mod vertex_index;
mod manifold_dual_contouring;
mod cell_configs;
mod qef;
mod mesh;
mod plane;

pub use self::manifold_dual_contouring::ManifoldDualContouringImpl;
// This is just exposed for the bench test - do not use!
pub use self::manifold_dual_contouring::subsample_octtree;


pub trait ImplicitFunction<S: Debug + Real> {
    fn bbox(&self) -> &BoundingBox<S>;
    fn value(&self, p: &na::Point3<S>) -> S;
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S>;
}

pub trait AsUSize {
    fn as_usize(self) -> usize;
}

impl AsUSize for f32 {
    fn as_usize(self) -> usize {
        self as usize
    }
}

impl AsUSize for f64 {
    fn as_usize(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
#[macro_use]
extern crate approx;
