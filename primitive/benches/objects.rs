extern crate alga;
#[macro_use]
extern crate bencher;
extern crate nalgebra;
extern crate num_traits;
extern crate truescad_primitive;
use alga::general::Real;
use bencher::Bencher;
use nalgebra as na;
use num_traits::{Float, FloatConst};
use std::fmt::Debug;
use truescad_primitive::{Intersection, Object, SlabX, SlabY, SlabZ, Sphere, Twister};


const STEPS: usize = 50;


fn evaluate<S: From<f32> + Debug + Float + Real>(obj: &Object<S>) -> S {
    let _0 = From::from(0f32);
    let mut p = na::Point3::new(_0, _0, obj.bbox().min.z);
    let xd = (obj.bbox().max.x - obj.bbox().min.x) / From::from(STEPS as f32);
    let yd = (obj.bbox().max.y - obj.bbox().min.y) / From::from(STEPS as f32);
    let zd = (obj.bbox().max.z - obj.bbox().min.z) / From::from(STEPS as f32);
    let slack = Float::min(xd, Float::min(yd, zd)) / From::from(10f32);
    let mut result = _0;
    for _ in 0..STEPS {
        p.y = obj.bbox().min.y;
        for _ in 0..STEPS {
            p.x = obj.bbox().min.x;
            for _ in 0..STEPS {
                result += obj.approx_value(p, slack);
                p.x += xd;
            }
            p.y += yd;
        }
        p.z += zd;
    }
    return result;
}

fn normals<S: 'static + From<f32> + Debug + Float + Real>(obj: &Object<S>) -> na::Vector3<S> {
    let _0 = From::from(0f32);
    let mut p = na::Point3::new(_0, _0, obj.bbox().min.z);
    let xd = (obj.bbox().max.x - obj.bbox().min.x) / From::from(STEPS as f32);
    let yd = (obj.bbox().max.y - obj.bbox().min.y) / From::from(STEPS as f32);
    let zd = (obj.bbox().max.z - obj.bbox().min.z) / From::from(STEPS as f32);
    let mut result = na::Vector3::new(_0, _0, _0);
    for _ in 0..STEPS {
        p.y = obj.bbox().min.y;
        for _ in 0..STEPS {
            p.x = obj.bbox().min.x;
            for _ in 0..STEPS {
                result += obj.normal(p);
                p.x += xd;
            }
            p.y += yd;
        }
        p.z += zd;
    }
    return result;
}

fn sphere<S: From<f32> + Debug + Float + Real>(b: &mut Bencher) {
    let object = Sphere::new(From::from(1f32));
    b.iter(|| evaluate(&*object as &Object<S>));
}
fn sphere_normals<S: From<f32> + Debug + Float + Real>(b: &mut Bencher) {
    let object = Sphere::new(From::from(1f32));
    b.iter(|| normals(&*object as &Object<S>));
}

fn create_cube<S: From<f32> + Debug + Float + Real>() -> Box<Object<S>> {
    let _0 = From::from(0f32);
    let _1 = From::from(1f32);
    Intersection::from_vec(vec![SlabX::new(_1), SlabY::new(_1), SlabZ::new(_1)], _0).unwrap()
        as Box<Object<S>>
}

fn cube<S: From<f32> + Debug + Float + Real>(b: &mut Bencher) {
    let object = create_cube();
    b.iter(|| evaluate(&*object as &Object<S>));
}
fn cube_normals<S: From<f32> + Debug + Float + Real>(b: &mut Bencher) {
    let object = create_cube();
    b.iter(|| normals(&*object as &Object<S>));
}

fn create_hollow_cube<S: From<f32> + Debug + Float + FloatConst + Real>() -> Box<Object<S>> {
    Intersection::difference_from_vec(
        vec![create_cube(), Sphere::new(From::from(0.5f32))],
        From::from(0.2f32),
    ).unwrap() as Box<Object<S>>
}

fn hollow_cube<S: From<f32> + Debug + Float + FloatConst + Real>(b: &mut Bencher) {
    let object = create_hollow_cube();
    b.iter(|| evaluate(&*object as &Object<S>));
}
fn hollow_cube_normals<S: From<f32> + Debug + Float + FloatConst + Real>(b: &mut Bencher) {
    let object = create_hollow_cube();
    b.iter(|| normals(&*object as &Object<S>));
}

fn twisted_cube<S: From<f32> + Debug + Float + FloatConst + Real>(b: &mut Bencher) {
    let object = Twister::new(create_cube(), From::from(4f32));
    b.iter(|| evaluate(&*object as &Object<S>));
}
fn twisted_cube_normals<S: From<f32> + Debug + Float + FloatConst + Real>(b: &mut Bencher) {
    let object = Twister::new(create_cube(), From::from(4f32));
    b.iter(|| normals(&*object as &Object<S>));
}

benchmark_group!(
    bench_values_f32,
    sphere<f32>,
    cube<f32>,
    hollow_cube<f32>,
    twisted_cube<f32>
);
benchmark_group!(
    bench_values_f64,
    sphere<f64>,
    cube<f64>,
    hollow_cube<f64>,
    twisted_cube<f64>
);
benchmark_group!(
    bench_normals_f32,
    sphere_normals<f32>,
    cube_normals<f32>,
    hollow_cube_normals<f32>,
    twisted_cube_normals<f32>
);
benchmark_group!(
    bench_normals_f64,
    sphere_normals<f64>,
    cube_normals<f64>,
    hollow_cube_normals<f64>,
    twisted_cube_normals<f64>
);
benchmark_main!(
    bench_values_f32,
    bench_normals_f32,
    bench_values_f64,
    bench_normals_f64
);
