use {BoundingBox, Object, PrimitiveParameters};
use alga::general::Real;
use alga::linear::Transformation;
use na;
use num_traits::Float;


#[derive(Clone, Debug)]
pub struct AffineTransformer<S: Real> {
    object: Box<Object<S>>,
    transform: na::Matrix4<S>,
    scale_min: S,
    bbox: BoundingBox<S>,
}

impl<S: Real + Float + From<f64>> Object<S> for AffineTransformer<S> {
    fn approx_value(&self, p: na::Point3<S>, slack: S) -> S {
        let approx = self.bbox.distance(p);
        if approx <= slack {
            self.object
                .approx_value(self.transform.transform_point(&p), slack / self.scale_min)
                * self.scale_min
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn set_parameters(&mut self, p: &PrimitiveParameters<S>) {
        self.object.set_parameters(p);
    }
    fn normal(&self, p: na::Point3<S>) -> na::Vector3<S> {
        self.transform
            .transform_vector(&self.object.normal(self.transform.transform_point(&p)))
            .normalize()
    }
    fn translate(&self, v: na::Vector3<S>) -> Box<Object<S>> {
        let new_trans = self.transform.append_translation(&-v);
        AffineTransformer::new_with_scaler(self.object.clone(), new_trans, self.scale_min)
    }
    fn rotate(&self, r: na::Vector3<S>) -> Box<Object<S>> {
        let euler = ::na::Rotation::from_euler_angles(r.x, r.y, r.z).to_homogeneous();
        let new_trans = self.transform * euler;
        AffineTransformer::new_with_scaler(self.object.clone(), new_trans, self.scale_min)
    }
    fn scale(&self, s: na::Vector3<S>) -> Box<Object<S>> {
        let _1: S = From::from(1f64);
        let new_trans = self.transform
            .append_nonuniform_scaling(&na::Vector3::new(_1 / s.x, _1 / s.y, _1 / s.z));
        AffineTransformer::new_with_scaler(
            self.object.clone(),
            new_trans,
            self.scale_min * Float::min(s.x, Float::min(s.y, s.z)),
        )
    }
}

impl<S: Real + Float + From<f64>> AffineTransformer<S> {
    fn identity(o: Box<Object<S>>) -> Box<Object<S>> {
        AffineTransformer::new(o, na::Matrix4::identity())
    }
    fn new(o: Box<Object<S>>, t: na::Matrix4<S>) -> Box<AffineTransformer<S>> {
        let _1: S = From::from(1f64);
        AffineTransformer::new_with_scaler(o, t, _1)
    }
    fn new_with_scaler(
        o: Box<Object<S>>,
        t: na::Matrix4<S>,
        scale_min: S,
    ) -> Box<AffineTransformer<S>> {
        // TODO: Calculate scale_min from t.
        // This should be something similar to
        // 1./Vector::new(t.x.x, t.y.x, t.z.x).magnitude().min(
        // 1./Vector::new(t.x.y, t.y.y, t.z.y).magnitude().min(
        // 1./Vector::new(t.x.z, t.y.z, t.z.z).magnitude()))

        match t.try_inverse() {
            None => panic!("Failed to invert {:?}", t),
            Some(t_inv) => {
                let bbox = o.bbox().transform(&t_inv);
                Box::new(AffineTransformer {
                    object: o,
                    transform: t,
                    scale_min: scale_min,
                    bbox: bbox,
                })
            }
        }
    }
    pub fn new_translate(o: Box<Object<S>>, v: na::Vector3<S>) -> Box<Object<S>> {
        AffineTransformer::identity(o).translate(v)
    }
    pub fn new_rotate(o: Box<Object<S>>, r: na::Vector3<S>) -> Box<Object<S>> {
        AffineTransformer::identity(o).rotate(r)
    }
    pub fn new_scale(o: Box<Object<S>>, s: na::Vector3<S>) -> Box<Object<S>> {
        AffineTransformer::identity(o).scale(s)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub struct MockObject<S: Real> {
        value: S,
        normal: na::Vector3<S>,
        bbox: BoundingBox<S>,
    }

    impl<S: ::std::fmt::Debug + Float + Real> MockObject<S> {
        pub fn new(value: S, normal: na::Vector3<S>) -> Box<MockObject<S>> {
            Box::new(MockObject {
                value: value,
                normal: normal,
                bbox: BoundingBox::infinity(),
            })
        }
    }

    impl<S: ::std::fmt::Debug + Real + Float + From<f64>> Object<S> for MockObject<S> {
        fn approx_value(&self, _: na::Point3<S>, _: S) -> S {
            self.value
        }
        fn normal(&self, _: na::Point3<S>) -> na::Vector3<S> {
            self.normal.clone()
        }
        fn bbox(&self) -> &BoundingBox<S> {
            &self.bbox
        }
    }

    #[test]
    fn translate() {
        let mock_object = MockObject::new(1.0, na::Vector3::new(1.0, 0.0, 0.0));
        let translated = mock_object.translate(na::Vector3::new(0.0001, 0.0, 0.0));
        let p = na::Point3::new(1.0, 0.0, 0.0);
        assert_eq!(mock_object.normal(p), translated.normal(p));
    }
}
