use alga::general::RealField;
use bbox::BoundingBox;
use na;
use num_traits::Float;
use plane::Plane;
use std::convert;
use std::fmt::Debug;

pub const EPSILON: f32 = 1e-10;

// Quadratic error function

#[derive(Clone, Debug)]
pub struct Qef<S: 'static + RealField + Debug> {
    // Point closest to all planes.
    pub solution: na::Vector3<S>,
    sum: na::Vector3<S>,
    pub num: usize,
    // Upper right triangle of AT * A
    ata: [S; 6],
    // Vector AT * B
    atb: na::Vector3<S>,
    // Scalar BT * B
    btb: S,
    pub error: S,
    bbox: BoundingBox<S>,
}

impl<S: 'static + RealField + Float + Debug + From<f32>> Qef<S> {
    pub fn new(planes: &[Plane<S>], bbox: BoundingBox<S>) -> Qef<S> {
        let mut qef = Qef {
            solution: na::Vector3::new(S::nan(), S::nan(), S::nan()),
            sum: na::Vector3::new(
                convert::From::from(0.),
                convert::From::from(0.),
                convert::From::from(0.),
            ),
            num: planes.len(),
            ata: [convert::From::from(0.); 6],
            atb: na::Vector3::new(
                convert::From::from(0.),
                convert::From::from(0.),
                convert::From::from(0.),
            ),
            btb: convert::From::from(0.),
            error: S::nan(),
            bbox,
        };
        for p in planes {
            qef.ata[0] += p.n[0] * p.n[0];
            qef.ata[1] += p.n[0] * p.n[1];
            qef.ata[2] += p.n[0] * p.n[2];
            qef.ata[3] += p.n[1] * p.n[1];
            qef.ata[4] += p.n[1] * p.n[2];
            qef.ata[5] += p.n[2] * p.n[2];
            // TODO: use proper dot product api
            let pn = p.p.x * p.n.x + p.p.y * p.n.y + p.p.z * p.n.z;
            qef.atb[0] += p.n[0] * pn;
            qef.atb[1] += p.n[1] * pn;
            qef.atb[2] += p.n[2] * pn;
            qef.btb += pn * pn;
            qef.sum[0] += p.p[0];
            qef.sum[1] += p.p[1];
            qef.sum[2] += p.p[2];
        }
        qef
    }
    pub fn solve(&mut self) {
        let m = &self.ata;
        let ma = na::Matrix3::new(m[0], m[1], m[2], m[1], m[3], m[4], m[2], m[4], m[5]);
        let sum_as_s: S = convert::From::from(self.num as f32);
        let mean: na::Vector3<S> = self.sum / sum_as_s;
        if let Some(inv) = ma.try_inverse() {
            let b_rel_mean: na::Vector3<S> = self.atb - ma * mean;
            self.solution = inv * b_rel_mean + mean;
        }

        // If solution is not contained in cell bbox, start a binary search for a proper solution.
        // NAN-solution will also not be contained in the bbox.
        if !self.bbox.contains(&na::Point3::new(
            self.solution.x,
            self.solution.y,
            self.solution.z,
        )) {
            let accuracy = (self.bbox.max.x - self.bbox.min.x) / convert::From::from(100.0);
            self.solution = self.search_solution(accuracy, &mut self.bbox.clone(), &ma);
            debug_assert!(
                self.bbox.dilate(accuracy).contains(&na::Point3::new(
                    self.solution.x,
                    self.solution.y,
                    self.solution.z
                )),
                "{:?} outside of {:?}",
                self.solution,
                self
            );
        }
        self.error = self.error(&self.solution, &ma);
    }
    // Do a binary search. Stop, if bbox is smaller then accuracy.
    fn search_solution(
        &self,
        accuracy: S,
        bbox: &mut BoundingBox<S>,
        ma: &na::Matrix3<S>,
    ) -> na::Vector3<S> {
        // Generate bbox mid-point and error value on mid-point.
        // TODO: use proper apis
        let mid = na::Point3::new(
            (bbox.max.x + bbox.min.x) * convert::From::from(0.5),
            (bbox.max.y + bbox.min.y) * convert::From::from(0.5),
            (bbox.max.z + bbox.min.z) * convert::From::from(0.5),
        );
        let na_mid = na::Vector3::new(mid.x, mid.y, mid.z);
        if bbox.max.x - bbox.min.x <= accuracy {
            return na_mid;
        }
        let mid_error = self.error(&na_mid, ma);
        // For each dimension generate delta and error on delta - which results in the gradient for
        // that direction. Based on the gradient sign choose proper half of the bbox.
        // TODO: Verify this is the right thing to do. Error is essentially an Elipsoid, so we
        // might need to do something more clever here.
        for dim in 0..3 {
            let mut d_mid = na_mid;
            d_mid[dim] += convert::From::from(EPSILON);
            let d_error = self.error(&d_mid, ma);
            if d_error < mid_error {
                bbox.min[dim] = mid[dim];
            } else {
                bbox.max[dim] = mid[dim];
            }
        }
        self.search_solution(accuracy, bbox, ma)
    }
    fn error(&self, point: &na::Vector3<S>, ma: &na::Matrix3<S>) -> S {
        let _2_as_s: S = convert::From::from(2f32);
        self.btb - _2_as_s * na::Matrix::dot(point, &self.atb) + na::Matrix::dot(point, &(*ma * *point))
    }
    pub fn merge(&mut self, other: &Qef<S>) {
        for i in 0..6 {
            self.ata[i] += other.ata[i];
        }
        self.atb += other.atb;
        self.btb += other.btb;
        self.sum += other.sum;
        self.num += other.num;
        self.bbox = self.bbox.union(&other.bbox);
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    use super::{BoundingBox, Qef};
    use na;

    #[test]
    fn origin() {
        let origin = na::Point3::new(0., 0., 0.);
        let mut qef = Qef::new(
            &[
                Plane {
                    p: origin,
                    n: na::Vector3::new(0., 1., 2.).normalize(),
                },
                Plane {
                    p: origin,
                    n: na::Vector3::new(1., 2., 3.).normalize(),
                },
                Plane {
                    p: origin,
                    n: na::Vector3::new(2., 3., 4.).normalize(),
                },
            ],
            BoundingBox::<f64>::new(&na::Point3::new(0., 0., 0.), &na::Point3::new(1., 1., 1.)),
        );
        qef.solve();
        assert!(
            qef.solution.norm() < 0.01,
            "{:?} nowhere near origin",
            qef.solution
        );
    }

    #[test]
    fn points_on_cube_solution_in_origin() {
        let mut qef = Qef::new(
            &[
                Plane {
                    p: na::Point3::new(1., 0., 0.),
                    n: na::Vector3::new(0., 1., 1.).normalize(),
                },
                Plane {
                    p: na::Point3::new(0., 1., 0.),
                    n: na::Vector3::new(1., 0., 1.).normalize(),
                },
                Plane {
                    p: na::Point3::new(0., 0., 1.),
                    n: na::Vector3::new(1., 1., 0.).normalize(),
                },
            ],
            BoundingBox::<f64>::new(&na::Point3::new(0., 0., 0.), &na::Point3::new(1., 1., 1.)),
        );
        qef.solve();
        assert!(relative_eq!(qef.solution, &na::Vector3::new(0., 0., 0.)));
    }

    #[test]
    fn points_on_origin_solution_on_cube() {
        let mut qef = Qef::new(
            &[
                Plane {
                    p: na::Point3::new(1., 0., 0.),
                    n: na::Vector3::new(1., 0., 0.),
                },
                Plane {
                    p: na::Point3::new(0., 2., 0.),
                    n: na::Vector3::new(0., 1., 0.),
                },
                Plane {
                    p: na::Point3::new(0., 0., 3.),
                    n: na::Vector3::new(0., 0., 1.),
                },
            ],
            BoundingBox::<f64>::new(&na::Point3::new(0., 0., 0.), &na::Point3::new(1., 2., 3.)),
        );
        qef.solve();
        let expected_solution = na::Vector3::new(1., 2., 3.);
        assert!(
            relative_eq!(qef.solution, &expected_solution),
            "{} != {}",
            qef.solution,
            expected_solution
        );
    }
}
