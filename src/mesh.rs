use alga::general::Real;
use na;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh<S> {
    pub vertices: Vec<[S; 3]>,
    pub faces: Vec<[usize; 3]>,
}

impl<S: 'static + Real + Debug> Mesh<S> {
    pub fn normal32(&self, face: usize) -> [f32; 3]
    where
        f64: From<S>,
    {
        let v: Vec<na::Point3<f32>> = self.faces[face]
            .iter()
            .map(|&i| {
                let v: (f64, f64, f64) = (
                    self.vertices[i][0].into(),
                    self.vertices[i][1].into(),
                    self.vertices[i][2].into(),
                );
                na::Point3::<f32>::new(v.0 as f32, v.1 as f32, v.2 as f32)
            })
            .collect();
        let r = (v[1] - v[0]).cross(&(v[2] - v[0])).normalize();
        [r[0], r[1], r[2]]
    }
    pub fn vertex32(&self, i: usize) -> [f32; 3]
    where
        f64: From<S>,
    {
        let v: (f64, f64, f64) = (
            self.vertices[i][0].into(),
            self.vertices[i][1].into(),
            self.vertices[i][2].into(),
        );
        [v.0 as f32, v.1 as f32, v.2 as f32]
    }
}
