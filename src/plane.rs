use alga::general::RealField;
use crate::na;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Plane<S: 'static + RealField + Debug> {
    pub p: na::Point3<S>,
    pub n: na::Vector3<S>,
}
