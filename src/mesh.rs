use alga::general::RealField;
use na;
use std::fmt::Debug;

/// Mesh that will be returned from tessellate.
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh<S> {
    /// The list of vertices.
    pub vertices: Vec<[S; 3]>,
    /// The list of triangles as indexes into vertices.
    pub faces: Vec<[usize; 3]>,
}

impl<S: 'static + RealField + Debug> Mesh<S> {
    /// Return the normal of the face at index face as triple of f32.
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
            }).collect();
        let r = (v[1] - v[0]).cross(&(v[2] - v[0])).normalize();
        [r[0], r[1], r[2]]
    }
    /// Return the vertics of the face at index face as triple of f32.
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let m = Mesh {
            vertices: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            faces: vec![[0, 1, 2]],
        };
        assert_eq!(m.normal32(0), [0., 0., 1.]);
        assert_eq!(m.vertex32(0), [0., 0., 0.]);
        assert_eq!(m.vertex32(1), [1., 0., 0.]);
        assert_eq!(m.vertex32(2), [0., 1., 0.]);
    }
}
