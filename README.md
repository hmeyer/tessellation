# tessellation
[![Build Status](https://travis-ci.org/hmeyer/tessellation.svg?branch=master)](https://travis-ci.org/hmeyer/tessellation)
[![Codecov](https://codecov.io/github/hmeyer/tessellation/coverage.svg?branch=master)](https://codecov.io/github/hmeyer/tessellation)
[![Cargo](https://img.shields.io/crates/v/tessellation.svg)](https://crates.io/crates/tessellation)
[![License: GPL-3.0](https://img.shields.io/crates/l/direct-gui.svg)](#license)
[![Downloads](https://img.shields.io/crates/d/tessellation.svg)](#downloads)

Tessellation implements [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf).

Tessellation is a library for 3d tessellation, e.g. it will create a set of triangles from any implicit function of volume.
Tessellation implements [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf).
# Examples

Create a unit sphere and tessellate it:

```rust
extern crate nalgebra as na;
extern crate tessellation;
//!
struct UnitSphere {
  bbox : tessellation::BoundingBox<f64>
}
//!
impl UnitSphere {
  fn new() -> UnitSphere {
    UnitSphere {
      bbox: tessellation::BoundingBox::new(&na::Point3::new(-1., -1., -1.),
                                           &na::Point3::new( 1.,  1.,  1.)) }
  }
}

impl tessellation::ImplicitFunction<f64> for UnitSphere {
   fn bbox(&self) -> &tessellation::BoundingBox<f64> {
     &self.bbox
   }
  fn value(&self, p: &na::Point3<f64>) -> f64 {
    return na::Vector3::new(p.x, p.y, p.z).norm() - 1.0;
  }
  fn normal(&self, p: &na::Point3<f64>) -> na::Vector3<f64> {
    return na::Vector3::new(p.x, p.y, p.z).normalize();
  }
}

let sphere = UnitSphere::new();
let mut mdc =  tessellation::ManifoldDualContouring::new(&sphere, 0.2, 0.1);
let triangles = mdc.tessellate().unwrap();
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
