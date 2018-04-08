extern crate alga;
extern crate nalgebra as na;
extern crate num_traits;

use alga::general::Real;
use num_traits::Float;
use std::fmt::Debug;

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
    pub fn infinity() -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(S::neg_infinity(), S::neg_infinity(), S::neg_infinity()),
            max: na::Point3::<S>::new(S::infinity(), S::infinity(), S::infinity()),
        }
    }
    pub fn neg_infinity() -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(S::infinity(), S::infinity(), S::infinity()),
            max: na::Point3::<S>::new(S::neg_infinity(), S::neg_infinity(), S::neg_infinity()),
        }
    }
    pub fn new(min: na::Point3<S>, max: na::Point3<S>) -> BoundingBox<S> {
        BoundingBox { min: min, max: max }
    }
    pub fn union(&self, other: &BoundingBox<S>) -> BoundingBox<S> {
        BoundingBox {
            min: point_min(&[self.min, other.min]),
            max: point_max(&[self.max, other.max]),
        }
    }
    pub fn intersection(&self, other: &BoundingBox<S>) -> BoundingBox<S> {
        BoundingBox {
            min: point_max(&[self.min, other.min]),
            max: point_min(&[self.max, other.max]),
        }
    }
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
    pub fn dilate(&self, d: S) -> BoundingBox<S> {
        BoundingBox {
            min: na::Point3::<S>::new(self.min.x - d, self.min.y - d, self.min.z - d),
            max: na::Point3::<S>::new(self.max.x + d, self.max.y + d, self.max.z + d),
        }
    }
    pub fn insert(&self, o: na::Point3<S>) -> BoundingBox<S> {
        BoundingBox {
            min: point_min(&[self.min, o]),
            max: point_max(&[self.max, o]),
        }
    }
    pub fn dim(&self) -> na::Vector3<S> {
        self.max - self.min
    }
    pub fn value(&self, p: na::Point3<S>) -> S {
        // If p is not inside (neg), then it is outside (pos) on only one side.
        // So so calculating the max of the diffs on both sides should result in the true value,
        // if positive.
        let xval = Real::max(p.x - self.max.x, self.min.x - p.x);
        let yval = Real::max(p.y - self.max.y, self.min.y - p.y);
        let zval = Real::max(p.z - self.max.z, self.min.z - p.z);
        Real::max(xval, Real::max(yval, zval))
    }
    pub fn contains(&self, p: na::Point3<S>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
            && p.z >= self.min.z && p.z <= self.max.z
    }
}
