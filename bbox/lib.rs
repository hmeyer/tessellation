//! ```bbox_rs``` is crate for managing axis aligned 3d Bounding Boxes.
//! Bounding Boxes can be created, dilated, transformed and joined with other Bounding Boxes using
//! CSG operations.
//! Finally you can test whether or not a Bounding Box contains some point and what approximate
//! distance a Point has to the Box.
//! # Examples
//!
//! Intersect two Bounding Boxes
//!
//! ```rust,no_run
//! extern crate nalgebra as na;
//! extern crate truescad_bbox;
//! let bbox1 = truescad_bbox::BoundingBox::<f64>::new(na::Point3::new(0., 0., 0.),
//!                                                    na::Point3::new(1., 2., 3.));
//! let bbox2 = truescad_bbox::BoundingBox::<f64>::new(na::Point3::new(-1., -2., -3.),
//!                                                    na::Point3::new(3., 2., 1.));
//! let intersection = bbox1.intersection(&bbox2);
//! ```
//! Rotate a Bounding Box:
//!
//! ```rust,no_run
//! extern crate nalgebra as na;
//! extern crate truescad_bbox;
//! let rotation = na::Rotation::from_euler_angles(10., 11., 12.).to_homogeneous();
//! let bbox = truescad_bbox::BoundingBox::<f64>::new(na::Point3::new(0., 0., 0.),
//!                                                   na::Point3::new(1., 2., 3.));
//! let rotated_box = bbox.transform(&rotation);
//! ```

extern crate alga;
extern crate nalgebra as na;
extern crate num_traits;

use alga::general::Real;
use num_traits::Float;
use std::fmt::Debug;

/// 3D Bounding Box - defined by two diagonally opposing points.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox<S: 'static + Real + Debug> {
    pub min: na::Point3<S>,
    pub max: na::Point3<S>,
}

fn point_min<S: 'static + Float + Real + Debug>(p: &[na::Point3<S>]) -> na::Point3<S> {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = point_min(p1);
        let b = point_min(p2);
        na::Point3::<S>::new(
            Real::min(a.x, b.x),
            Real::min(a.y, b.y),
            Real::min(a.z, b.z),
        )
    }
}
fn point_max<S: 'static + Float + Real + Debug>(p: &[na::Point3<S>]) -> na::Point3<S> {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = point_max(p1);
        let b = point_max(p2);
        na::Point3::<S>::new(
            Real::max(a.x, b.x),
            Real::max(a.y, b.y),
            Real::max(a.z, b.z),
        )
    }
}

impl<S: 'static + Float + Real + Debug> BoundingBox<S> {
    /// Returns an infinte sized box.
    pub fn infinity() -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(S::neg_infinity(), S::neg_infinity(), S::neg_infinity()),
            max: na::Point3::<S>::new(S::infinity(), S::infinity(), S::infinity()),
        }
    }
    /// Returns a negatively infinte sized box.
    pub fn neg_infinity() -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(S::infinity(), S::infinity(), S::infinity()),
            max: na::Point3::<S>::new(S::neg_infinity(), S::neg_infinity(), S::neg_infinity()),
        }
    }
    /// Create a new Bounding Box by supplying two points.
    pub fn new(a: na::Point3<S>, b: na::Point3<S>) -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(
                Real::min(a.x, b.x),
                Real::min(a.y, b.y),
                Real::min(a.z, b.z),
            ),
            max: na::Point3::<S>::new(
                Real::max(a.x, b.x),
                Real::max(a.y, b.y),
                Real::max(a.z, b.z),
            ),
        }
    }
    /// Create a CSG Union of two Bounding Boxes.
    pub fn union(&self, other: &BoundingBox<S>) -> BoundingBox<S> {
        BoundingBox {
            min: point_min(&[self.min, other.min]),
            max: point_max(&[self.max, other.max]),
        }
    }
    /// Create a CSG Intersection of two Bounding Boxes.
    pub fn intersection(&self, other: &BoundingBox<S>) -> BoundingBox<S> {
        BoundingBox {
            min: point_max(&[self.min, other.min]),
            max: point_min(&[self.max, other.max]),
        }
    }
    /// Transform a Bounding Box - resulting in a enclosing axis aligned Bounding Box.
    pub fn transform<M>(&self, mat: &M) -> BoundingBox<S>
    where
        M: alga::linear::Transformation<na::Point3<S>>,
    {
        let a = &self.min;
        let b = &self.max;
        let corners = [
            mat.transform_point(&na::Point3::<S>::new(a.x, a.y, a.z)),
            mat.transform_point(&na::Point3::<S>::new(a.x, a.y, b.z)),
            mat.transform_point(&na::Point3::<S>::new(a.x, b.y, a.z)),
            mat.transform_point(&na::Point3::<S>::new(a.x, b.y, b.z)),
            mat.transform_point(&na::Point3::<S>::new(b.x, a.y, a.z)),
            mat.transform_point(&na::Point3::<S>::new(b.x, a.y, b.z)),
            mat.transform_point(&na::Point3::<S>::new(b.x, b.y, a.z)),
            mat.transform_point(&na::Point3::<S>::new(b.x, b.y, b.z)),
        ];
        BoundingBox {
            min: point_min(&corners),
            max: point_max(&corners),
        }
    }
    /// Dilate a Bounding Box by some amount in all directions.
    pub fn dilate(&self, d: S) -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(self.min.x - d, self.min.y - d, self.min.z - d),
            max: na::Point3::<S>::new(self.max.x + d, self.max.y + d, self.max.z + d),
        }
    }
    /// Add a Point to a Bounding Box, e.g. expand the Bounding Box to contain that point.
    pub fn insert(&self, o: na::Point3<S>) -> BoundingBox<S> {
        BoundingBox {
            min: point_min(&[self.min, o]),
            max: point_max(&[self.max, o]),
        }
    }
    /// Return the size of the Box.
    pub fn dim(&self) -> na::Vector3<S> {
        self.max - self.min
    }
    /// Returns the approximate distance of p to the box. The result is guarateed to be not less
    /// than the euclidean distance of p to the box.
    pub fn distance(&self, p: na::Point3<S>) -> S {
        // If p is not inside (neg), then it is outside (pos) on only one side.
        // So so calculating the max of the diffs on both sides should result in the true value,
        // if positive.
        let xval = Real::max(p.x - self.max.x, self.min.x - p.x);
        let yval = Real::max(p.y - self.max.y, self.min.y - p.y);
        let zval = Real::max(p.z - self.max.z, self.min.z - p.z);
        Real::max(xval, Real::max(yval, zval))
    }
    /// Return true if the Bounding Box contains p.
    pub fn contains(&self, p: na::Point3<S>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
            && p.z >= self.min.z && p.z <= self.max.z
    }
}
