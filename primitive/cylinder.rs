use {BoundingBox, Object};
use alga::general::Real;
use na;
use num_traits::Float;


// A cylinder along the Z-Axis
#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder<S: Real> {
    radius: S,
    bbox: BoundingBox<S>,
}

impl<S: Real + Float> Cylinder<S> {
    pub fn new(r: S) -> Box<Cylinder<S>> {
        Box::new(Cylinder {
            radius: r,
            bbox: BoundingBox::new(
                na::Point3::new(-r, -r, S::neg_infinity()),
                na::Point3::new(r, r, S::infinity()),
            ),
        })
    }
}

impl<S: ::std::fmt::Debug + Real + From<f64> + Float> Object<S> for Cylinder<S> {
    fn approx_value(&self, p: na::Point3<S>, slack: S) -> S {
        let approx = self.bbox.distance(p);
        if approx <= slack {
            let _0: S = From::from(0f64);
            let pv = na::Vector3::new(p.x, p.y, _0);
            return pv.norm() - self.radius;
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn normal(&self, p: na::Point3<S>) -> na::Vector3<S> {
        let _0: S = From::from(0f64);
        let pv = na::Vector3::new(p.x, p.y, _0);
        return pv.normalize();
    }
}

// A cone along the Z-Axis
#[derive(Clone, Debug, PartialEq)]
pub struct Cone<S: Real> {
    slope: S,
    distance_multiplier: S,
    offset: S,            // Offset the singularity from Z-zero
    normal_multiplier: S, // muliplier for the normal caclulation
    bbox: BoundingBox<S>,
}

impl<S: Real + Float + From<f64>> Cone<S> {
    pub fn new(slope: S, offset: S) -> Box<Cone<S>> {
        let _1: S = From::from(1f64);
        Box::new(Cone {
            slope: slope,
            distance_multiplier: _1 / Float::sqrt(slope * slope + _1), // cos(atan(slope))
            offset: offset,
            normal_multiplier: slope / Float::sqrt(slope * slope + _1), // sin(atan(slope))
            bbox: BoundingBox::infinity(),
        })
    }
}

impl<S: ::std::fmt::Debug + Real + From<f64> + Float> Object<S> for Cone<S> {
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn set_bbox(&mut self, bbox: BoundingBox<S>) {
        self.bbox = bbox
    }
    fn approx_value(&self, p: na::Point3<S>, _: S) -> S {
        let radius = Float::abs(self.slope * (p.z + self.offset));
        let _0: S = From::from(0f64);
        let pv = na::Vector3::new(p.x, p.y, _0);
        return (pv.norm() - radius) * self.distance_multiplier;
    }
    fn normal(&self, p: na::Point3<S>) -> na::Vector3<S> {
        let s = Float::signum(p.z + self.offset);
        let _0: S = From::from(0f64);
        let mut pv = na::Vector3::new(p.x, p.y, _0);
        pv.normalize_mut();
        pv *= self.distance_multiplier;
        pv.z = -s * self.normal_multiplier;
        return pv;
    }
}
