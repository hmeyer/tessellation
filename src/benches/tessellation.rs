extern crate alga;
#[macro_use]
extern crate bencher;
extern crate implicit3d;
extern crate nalgebra;
extern crate num_traits;
extern crate tessellation;
use alga::general::Real;
use bencher::Bencher;
use implicit3d::{Intersection, Object, SlabX, SlabY, SlabZ, Sphere};
use nalgebra as na;
use num_traits::Float;
use tessellation::{AsUSize, BoundingBox, ImplicitFunction, ManifoldDualContouringImpl};


struct ObjectAdaptor<S: Real> {
    implicit: Box<implicit3d::Object<S>>,
    resolution: S,
}

impl<S: ::std::fmt::Debug + na::Real + ::num_traits::Float + From<f32>> ImplicitFunction<S>
    for ObjectAdaptor<S> {
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
    let _0: S = From::from(0f32);
    let _1: S = From::from(1f32);
    Intersection::from_vec(vec![SlabX::new(_1), SlabY::new(_1), SlabZ::new(_1)], _0).unwrap()
        as Box<Object<S>>
}

fn create_hollow_cube<S: From<f32> + Float + Real>() -> Box<Object<S>> {
    let _02: S = From::from(0.2f32);
    let _05: S = From::from(0.5f32);
    Intersection::difference_from_vec(vec![create_cube(), Sphere::new(_05)], _02).unwrap()
        as Box<Object<S>>
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

fn sample_value_grid<S: Real + AsUSize + Float + From<f32>>(b: &mut Bencher) {
    let o = create_object::<S>();
    let tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.tessellation_step1()
    });
}

fn compact_value_grid<S: AsUSize + Real + Float + From<f32>>(b: &mut Bencher) {
    let o = create_object::<S>();
    let mut tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    tess.tessellation_step1();
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.compact_value_grid()
    });
}

fn generate_edge_grid<S: AsUSize + Real + Float + From<f32>>(b: &mut Bencher) {
    let o = create_object::<S>();
    let mut tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    tess.tessellation_step1();
    tess.compact_value_grid();
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.generate_edge_grid()
    });
}

fn generate_leaf_vertices<S: AsUSize + Real + Float + From<f32>>(b: &mut Bencher) {
    let o = create_object::<S>();
    let mut tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    b.iter(|| {
        let my_tess = tess.clone();
        my_tess.generate_leaf_vertices()
    });
}

fn subsample_octtree<S: Real + Float + From<f32> + AsUSize>(b: &mut Bencher) {
    let o = create_object::<S>();
    let mut tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    let (leafs, index_map) = tess.generate_leaf_vertices();
    tess.vertex_index_map = index_map;
    tess.vertex_octtree.push(leafs);
    b.iter(|| {
        let mut my_tess = tess.clone();
        loop {
            let next = tessellation::subsample_octtree(my_tess.vertex_octtree.last().unwrap());
            if next.len() == my_tess.vertex_octtree.last().unwrap().len() {
                break;
            }
            my_tess.vertex_octtree.push(next);
        }
    });
}

fn solve_qefs<S: Real + Float + From<f32> + AsUSize>(b: &mut Bencher) {
    let o = create_object::<S>();
    let mut tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    let (leafs, index_map) = tess.generate_leaf_vertices();
    tess.vertex_index_map = index_map;
    tess.vertex_octtree.push(leafs);
    loop {
        let next = tessellation::subsample_octtree(tess.vertex_octtree.last().unwrap());
        if next.len() == tess.vertex_octtree.last().unwrap().len() {
            break;
        }
        tess.vertex_octtree.push(next);
    }
    b.iter(|| {
        let my_tess = tess.clone();
        my_tess.solve_qefs();
    });
}

fn compute_quad<S: From<f32> + AsUSize + Real + Float>(b: &mut Bencher) {
    let o = create_object::<S>();
    let mut tess = ManifoldDualContouringImpl::new(&o, From::from(0.02), From::from(0.1));
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    let (leafs, index_map) = tess.generate_leaf_vertices();
    tess.vertex_index_map = index_map;
    tess.vertex_octtree.push(leafs);
    loop {
        let next = tessellation::subsample_octtree(tess.vertex_octtree.last().unwrap());
        if next.len() == tess.vertex_octtree.last().unwrap().len() {
            break;
        }
        tess.vertex_octtree.push(next);
    }
    tess.solve_qefs();
    b.iter(|| {
        let my_tess = tess.clone();
        for edge_index in my_tess.edge_grid.borrow().keys() {
            my_tess.compute_quad(*edge_index);
        }
    });
}



benchmark_group!(
    bench_tessellation_f32,
    sample_value_grid<f32>,
    compact_value_grid<f32>,
    generate_edge_grid<f32>,
    generate_leaf_vertices<f32>,
    subsample_octtree<f32>,
    solve_qefs<f32>,
    compute_quad<f32>
);
benchmark_group!(
    bench_tessellation_f64,
    sample_value_grid<f64>,
    compact_value_grid<f64>,
    generate_edge_grid<f64>,
    generate_leaf_vertices<f64>,
    subsample_octtree<f64>,
    solve_qefs<f64>,
    compute_quad<f64>
);
benchmark_main!(bench_tessellation_f32, bench_tessellation_f64);
