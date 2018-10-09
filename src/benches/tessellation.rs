extern crate alga;
#[macro_use]
extern crate bencher;
extern crate implicit3d;
extern crate nalgebra;
extern crate num_traits;
extern crate tessellation;
use alga::general::Real;
use bencher::Bencher;
use implicit3d::{
    Intersection, Object, PlaneNegX, PlaneNegY, PlaneNegZ, PlaneX, PlaneY, PlaneZ, Sphere,
};
use nalgebra as na;
use num_traits::Float;
use tessellation::{AsUSize, BoundingBox, ImplicitFunction, ManifoldDualContouring};

struct ObjectAdaptor<S: Real> {
    implicit: Box<implicit3d::Object<S>>,
    resolution: S,
}

impl<S: ::std::fmt::Debug + na::Real + ::num_traits::Float + From<f32>> ImplicitFunction<S>
    for ObjectAdaptor<S>
{
    fn bbox(&self) -> &BoundingBox<S> {
        self.implicit.bbox()
    }
    fn value(&self, p: &na::Point3<S>) -> S {
        self.implicit.approx_value(p, self.resolution)
    }
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S> {
        self.implicit.normal(p)
    }
}

fn create_cube<S: From<f32> + Float + Real>() -> Box<Object<S>> {
    let zero: S = From::from(0f32);
    let one: S = From::from(1f32);
    Intersection::from_vec(
        vec![
            PlaneX::new(one),
            PlaneY::new(one),
            PlaneZ::new(one),
            PlaneNegX::new(one),
            PlaneNegY::new(one),
            PlaneNegZ::new(one),
        ],
        zero,
    ).unwrap() as Box<Object<S>>
}

fn create_hollow_cube<S: From<f32> + Float + Real>() -> Box<Object<S>> {
    let point_two: S = From::from(0.2f32);
    let point_five: S = From::from(0.5f32);
    Intersection::difference_from_vec(vec![create_cube(), Sphere::new(point_five)], point_two)
        .unwrap() as Box<Object<S>>
}

fn create_object<S: Real + AsUSize + Float + From<f32>>() -> ObjectAdaptor<S> {
    let mut object = create_hollow_cube::<S>();
    object.set_parameters(&implicit3d::PrimitiveParameters {
        fade_range: From::from(0.1),
        r_multiplier: From::from(1.0),
    });
    ObjectAdaptor {
        implicit: object,
        resolution: From::from(0.02),
    }
}

fn tessellate<S: From<f32> + AsUSize + Real + Float>(b: &mut Bencher) {
    let o = create_object::<S>();
    let tess = ManifoldDualContouring::new(&o, From::from(0.02), From::from(0.1));
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.tessellate();
    });
}

benchmark_group!(bench_tessellation_f32, tessellate<f32>,);
benchmark_group!(bench_tessellation_f64, tessellate<f64>,);
benchmark_main!(bench_tessellation_f32, bench_tessellation_f64);
