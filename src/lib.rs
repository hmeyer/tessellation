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

use alga::general::RealField;
pub use bbox::BoundingBox;
use std::fmt::Debug;

mod bitset;
mod cell_configs;
mod manifold_dual_contouring;
mod mesh;
mod plane;
mod qef;
mod vertex_index;

pub use self::manifold_dual_contouring::ManifoldDualContouring;
pub use self::mesh::Mesh;

/// Trait to be implemented by functions that should be tessellated.
pub trait ImplicitFunction<S: Debug + RealField> {
    /// Return a Bounding Box, which is essential, so the algorithm knows where to search for
    /// surfaces.
    fn bbox(&self) -> &BoundingBox<S>;
    /// Evaluate the function on p and return the value. A value of zero signifies that p is on the
    /// surface to be tessellated. A negative value means p in inside the object. A positive value
    /// means p is outside the object.
    /// The magnitude of value must be continuous. Furthermore value has to be equal or greater
    /// than the euclidean distance between p and the surface.
    fn value(&self, p: &na::Point3<S>) -> S;
    /// Compute the normal of the function at p.
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S>;
}

/// Trait which allows to convert Self to usize, since To<usize> is not implemented by f32 and f64.
pub trait AsUSize {
    /// Convert Self to usize.
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
