use bencher::*;
use implicit3d::{
    Intersection, Object, PlaneNegX, PlaneNegY, PlaneNegZ, PlaneX, PlaneY, PlaneZ, Sphere,
};
use nalgebra as na;
use num_traits::Float;
use tessellation::{AsUSize, BoundingBox, ImplicitFunction, ManifoldDualContouring, RealField};

struct ObjectAdaptor<S: RealField> {
    implicit: Box<dyn implicit3d::Object<S>>,
    resolution: S,
}

impl<
        S: ::std::fmt::Debug + ::num_traits::Float + From<f32> + RealField + implicit3d::RealField,
    > ImplicitFunction<S> for ObjectAdaptor<S>
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

fn create_cube<S: From<f32> + Float + RealField + implicit3d::RealField>() -> Box<dyn Object<S>> {
    let zero: S = From::from(0f32);
    let one: S = From::from(1f32);
    Intersection::from_vec(
        vec![
            Box::new(PlaneX::new(one)),
            Box::new(PlaneY::new(one)),
            Box::new(PlaneZ::new(one)),
            Box::new(PlaneNegX::new(one)),
            Box::new(PlaneNegY::new(one)),
            Box::new(PlaneNegZ::new(one)),
        ],
        zero,
    )
    .unwrap() as Box<dyn Object<S>>
}

fn create_hollow_cube<S: From<f32> + Float + RealField + implicit3d::RealField>(
) -> Box<dyn Object<S>> {
    let point_two: S = From::from(0.2f32);
    let point_five: S = From::from(0.5f32);
    Intersection::difference_from_vec(
        vec![create_cube(), Box::new(Sphere::new(point_five))],
        point_two,
    )
    .unwrap() as Box<dyn Object<S>>
}

fn create_object<S: RealField + AsUSize + Float + From<f32> + RealField + implicit3d::RealField>(
) -> ObjectAdaptor<S> {
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

fn tessellate<S: From<f32> + AsUSize + RealField + Float + implicit3d::RealField>(b: &mut Bencher) {
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
