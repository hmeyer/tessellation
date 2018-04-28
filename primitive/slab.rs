use {BoundingBox, Object};
use alga::general::Real;
use na;
use num_traits::Float;

pub trait Axis
    : ::std::fmt::Debug + Clone + ::std::marker::Sync + ::std::marker::Send {
    fn value() -> usize;
}

#[derive(Clone, Debug)]
pub struct AxisX {}
impl Axis for AxisX {
    fn value() -> usize {
        0
    }
}

#[derive(Clone, Debug)]
pub struct AxisY {}
impl Axis for AxisY {
    fn value() -> usize {
        1
    }
}
#[derive(Clone, Debug)]
pub struct AxisZ {}
impl Axis for AxisZ {
    fn value() -> usize {
        2
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Slab<A: Axis, S: Real> {
    distance_from_zero: S,
    bbox: BoundingBox<S>,
    normal_pos: na::Vector3<S>,
    normal_neg: na::Vector3<S>,
    _phantom: ::std::marker::PhantomData<A>,
}

impl<A: Axis, S: From<f32> + Real + Float> Slab<A, S> {
    pub fn new(thickness: S) -> Box<Slab<A, S>> {
        let d = thickness * From::from(0.5f32);
        let mut p_neg = na::Point3::new(S::neg_infinity(), S::neg_infinity(), S::neg_infinity());
        p_neg[A::value()] = -d;
        let mut p_pos = na::Point3::new(S::infinity(), S::infinity(), S::infinity());
        p_pos[A::value()] = d;

        let _0: S = From::from(0f32);
        let mut normal_pos = na::Vector3::new(_0, _0, _0);
        let mut normal_neg = na::Vector3::new(_0, _0, _0);
        normal_pos[A::value()] = From::from(1f32);
        normal_neg[A::value()] = From::from(-1f32);

        Box::new(Slab {
            distance_from_zero: d,
            bbox: BoundingBox::new(p_neg, p_pos),
            normal_pos: normal_pos,
            normal_neg: normal_neg,
            _phantom: ::std::marker::PhantomData,
        })
    }
}

impl<A: 'static + Axis, S: Float + From<f32> + Real> Object<S> for Slab<A, S> {
    fn approx_value(&self, p: na::Point3<S>, _: S) -> S {
        return Float::abs(p[A::value()]) - self.distance_from_zero;
    }
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn normal(&self, p: na::Point3<S>) -> na::Vector3<S> {
        if Float::is_sign_positive(p.x) {
            return self.normal_pos.clone();
        } else {
            return self.normal_neg.clone();
        }
    }
}

pub type SlabX<S> = Slab<AxisX, S>;
pub type SlabY<S> = Slab<AxisY, S>;
pub type SlabZ<S> = Slab<AxisZ, S>;
