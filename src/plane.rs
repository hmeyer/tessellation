use simba::scalar::RealField;
use nalgebra as na;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Plane<S: RealField + Debug> {
    pub p: na::Point3<S>,
    pub n: na::Vector3<S>,
}
