extern crate alga;
extern crate truescad_types;
extern crate nalgebra as na;

use alga::linear::Transformation;
use truescad_types::{Float, INFINITY, Transform, NEG_INFINITY};

#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: na::Point3<Float>,
    pub max: na::Point3<Float>,
}

fn point_min(p: &[na::Point3<Float>]) -> na::Point3<Float> {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = point_min(p1);
        let b = point_min(p2);
        na::Point3::<Float>::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }
}
fn point_max(p: &[na::Point3<Float>]) -> na::Point3<Float> {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = point_max(p1);
        let b = point_max(p2);
        na::Point3::<Float>::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }
}

impl BoundingBox {
    pub fn infinity() -> BoundingBox {
        BoundingBox {
            min: na::Point3::<Float>::new(NEG_INFINITY, NEG_INFINITY,NEG_INFINITY),
            max: na::Point3::<Float>::new(INFINITY, INFINITY, INFINITY),
        }
    }
    pub fn neg_infinity() -> BoundingBox {
        BoundingBox {
            min: na::Point3::<Float>::new(INFINITY, INFINITY, INFINITY),
            max: na::Point3::<Float>::new(NEG_INFINITY, NEG_INFINITY,NEG_INFINITY),
        }
    }
    pub fn new(min: na::Point3<Float>, max: na::Point3<Float>) -> BoundingBox {
        BoundingBox { min: min, max: max }
    }
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: point_min(&[self.min, other.min]),
            max: point_max(&[self.max, other.max]),
        }
    }
    pub fn intersection(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: point_max(&[self.min, other.min]),
            max: point_min(&[self.max, other.max]),
        }
    }
    pub fn transform(&self, mat: &Transform) -> BoundingBox {
        let a = &self.min;
        let b = &self.max;
        let corners = [mat.transform_point(&na::Point3::<Float>::new(a.x, a.y, a.z)),
                       mat.transform_point(&na::Point3::<Float>::new(a.x, a.y, b.z)),
                       mat.transform_point(&na::Point3::<Float>::new(a.x, b.y, a.z)),
                       mat.transform_point(&na::Point3::<Float>::new(a.x, b.y, b.z)),
                       mat.transform_point(&na::Point3::<Float>::new(b.x, a.y, a.z)),
                       mat.transform_point(&na::Point3::<Float>::new(b.x, a.y, b.z)),
                       mat.transform_point(&na::Point3::<Float>::new(b.x, b.y, a.z)),
                       mat.transform_point(&na::Point3::<Float>::new(b.x, b.y, b.z))];
        BoundingBox {
            min: point_min(&corners),
            max: point_max(&corners),
        }
    }
    pub fn dilate(&self, d: Float) -> BoundingBox {
        BoundingBox {
            min: na::Point3::<Float>::new(self.min.x - d, self.min.y - d, self.min.z - d),
            max: na::Point3::<Float>::new(self.max.x + d, self.max.y + d, self.max.z + d),
        }
    }
    pub fn insert(&self, o: na::Point3<Float>) -> BoundingBox {
        BoundingBox {
            min: point_min(&[self.min, o]),
            max: point_max(&[self.max, o]),
        }
    }
    pub fn dim(&self) -> na::Vector3<Float> {
        self.max - self.min
    }
    pub fn value(&self, p: na::Point3<Float>) -> Float {
        // If p is not inside (neg), then it is outside (pos) on only one side.
        // So so calculating the max of the diffs on both sides should result in the true value,
        // if positive.
        let xval = (p.x - self.max.x).max(self.min.x - p.x);
        let yval = (p.y - self.max.y).max(self.min.y - p.y);
        let zval = (p.z - self.max.z).max(self.min.z - p.z);
        xval.max(yval.max(zval))
    }
    pub fn contains(&self, p: na::Point3<Float>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y &&
        p.z >= self.min.z && p.z <= self.max.z
    }
}
