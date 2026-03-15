use nalgebra as na;
use nalgebra::RealField;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct MeshError {
    msg: String,
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MeshError {}", self.msg)
    }
}

impl Error for MeshError {}

/// Mesh that will be returned from tessellate.
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh<S> {
    /// The list of vertices.
    pub vertices: Vec<[S; 3]>,
    /// The list of triangles as indexes into vertices.
    pub faces: Vec<[usize; 3]>,
}

impl<S: RealField + Copy + Debug> Mesh<S> {
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
            })
            .collect();
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
    /// Returns whether or not the mesh is closed.
    #[cfg(test)]
    pub fn is_closed(&self) -> Result<(), MeshError>
    where
        f64: From<S>,
    {
        let mut edge_to_face = std::collections::HashMap::new();
        for (face_index, face) in self.faces.iter().enumerate() {
            for i in 0..3 {
                if let Some(existing_index) =
                    edge_to_face.insert((face[i], face[(i + 1) % 3]), face_index)
                {
                    return Err(MeshError {
                        msg: format!(
                            "Both face #{} and face #{} share edge {}->{}.",
                            existing_index,
                            face_index,
                            i,
                            (i + 1) % 3
                        ),
                    });
                }
            }
        }
        for (edge, face_index) in edge_to_face.iter() {
            if !edge_to_face.contains_key(&(edge.1, edge.0)) {
                return Err(MeshError {
                    msg: format!(
                        "Unmachted edge {}->{} of face #{}.",
                        edge.0, edge.1, face_index
                    ),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn f32slice_eq(a: &[f32], b: &[f32]) -> bool {
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            if (a[i] - b[i]).abs() > f32::EPSILON {
                return false;
            }
        }
        true
    }

    #[test]
    fn simple() {
        let m = Mesh {
            vertices: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            faces: vec![[0, 1, 2]],
        };
        assert!(f32slice_eq(&m.normal32(0), &[0., 0., 1.]));
        assert!(f32slice_eq(&m.vertex32(0), &[0., 0., 0.]));
        assert!(f32slice_eq(&m.vertex32(1), &[1., 0., 0.]));
        assert!(f32slice_eq(&m.vertex32(2), &[0., 1., 0.]));
    }
}
