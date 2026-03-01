use bencher::*;
use nalgebra as na;
use num_traits::Float;
use tessellation::{AsUSize, BoundingBox, ImplicitFunction, ManifoldDualContouring, RealField};

struct UnitSphere<S: RealField> {
    bbox: BoundingBox<S>,
}

impl<S: RealField + Float + From<f32>> UnitSphere<S> {
    fn new() -> Self {
        UnitSphere {
            bbox: BoundingBox::new(
                &na::Point3::new(
                    From::from(-1.5f32),
                    From::from(-1.5f32),
                    From::from(-1.5f32),
                ),
                &na::Point3::new(From::from(1.5f32), From::from(1.5f32), From::from(1.5f32)),
            ),
        }
    }
}

impl<S: std::fmt::Debug + RealField + Float + From<f32>> ImplicitFunction<S> for UnitSphere<S> {
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn value(&self, p: &na::Point3<S>) -> S {
        na::Vector3::new(p.x, p.y, p.z).norm() - From::from(1.0f32)
    }
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S> {
        na::Vector3::new(p.x, p.y, p.z).normalize()
    }
}

fn tessellate_sphere<S: From<f32> + AsUSize + RealField + Float>(b: &mut Bencher) {
    let sphere = UnitSphere::<S>::new();
    let tess = ManifoldDualContouring::new(&sphere, From::from(0.1f32), From::from(0.05f32));
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.tessellate();
    });
}

benchmark_group!(bench_tessellation_f32, tessellate_sphere<f32>);
benchmark_group!(bench_tessellation_f64, tessellate_sphere<f64>);
benchmark_main!(bench_tessellation_f32, bench_tessellation_f64);
