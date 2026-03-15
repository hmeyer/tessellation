use nalgebra as na;
use nalgebra::RealField;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Plane<S: RealField + Debug> {
    pub p: na::Point3<S>,
    pub n: na::Vector3<S>,
}
