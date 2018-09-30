use alga::general::Real;
use na;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Plane<S: 'static + Real + Debug> {
    pub p: na::Point3<S>,
    pub n: na::Vector3<S>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert!(true);
    }
}
