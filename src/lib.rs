//! tessellation is crate for creating polygons from implicit functions or volumes.
//! It uses
//! [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf).
//!
//! # Examples
//!
//! Create a unit sphere and tessellate it:
//!
//! ```rust
//! use nalgebra as na;
//!
//! struct UnitSphere;
//!
//! impl tessellation::ImplicitFunction<f64> for UnitSphere {
//!   fn value(&self, p: &na::Point3<f64>) -> f64 {
//!     na::Vector3::new(p.x, p.y, p.z).norm() - 1.0
//!   }
//!   fn normal(&self, p: &na::Point3<f64>) -> na::Vector3<f64> {
//!     na::Vector3::new(p.x, p.y, p.z).normalize()
//!   }
//! }
//!
//! let sphere = UnitSphere;
//! let mut mdc = tessellation::ManifoldDualContouring::new(&sphere, 0.2, 0.1);
//! let triangles = mdc.tessellate().unwrap();
//! ```
#![warn(missing_docs)]

use nalgebra as na;
use std::fmt::Debug;

mod bitset;
mod cell_configs;
mod manifold_dual_contouring;
mod mesh;
mod plane;
mod qef;
mod vertex_index;

pub use self::manifold_dual_contouring::ManifoldDualContouring;
pub use self::manifold_dual_contouring::ProgressEvent;
pub use self::mesh::Mesh;

/// Trait alias for nalgebra's RealField.
pub trait RealField: na::RealField + Copy {}
impl RealField for f64 {}
impl RealField for f32 {}

/// Trait to be implemented by functions that should be tessellated.
pub trait ImplicitFunction<S: Debug + RealField> {
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
    fn as_usize(&self) -> usize;
}

impl AsUSize for f32 {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl AsUSize for f64 {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

