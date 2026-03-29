# tessellation
![build workflow](https://github.com/hmeyer/tessellation/actions/workflows/rust.yml/badge.svg?branch=main)
[![Codecov](https://codecov.io/github/hmeyer/tessellation/coverage.svg?branch=main)](https://codecov.io/github/hmeyer/tessellation)
[![Cargo](https://img.shields.io/crates/v/tessellation.svg)](https://crates.io/crates/tessellation)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/tessellation.svg)](#downloads)


Tessellation is a library for 3d tessellation, e.g. it will create a set of triangles from any implicit function of volume.
Tessellation implements [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf).
# Examples

Create a unit sphere and tessellate it:

```rust
use nalgebra as na;

struct UnitSphere;

impl tessellation::ImplicitFunction<f64> for UnitSphere {
  fn value(&self, p: &na::Point3<f64>) -> f64 {
    na::Vector3::new(p.x, p.y, p.z).norm() - 1.0
  }
  fn normal(&self, p: &na::Point3<f64>) -> na::Vector3<f64> {
    na::Vector3::new(p.x, p.y, p.z).normalize()
  }
}

let sphere = UnitSphere;
let mut mdc = tessellation::ManifoldDualContouring::new(&sphere, 0.2, 0.1);
let triangles = mdc.tessellate().unwrap();
```

# Progress reporting

For long-running tessellations (e.g. in a WebAssembly/WebWorker context) you can
receive incremental progress via a callback:

```rust
mdc.tessellate_with_progress(|event| {
    println!("{:.0}%  {:?}", event.progress_fraction() * 100.0, event);
});
```

`ProgressEvent` covers nine pipeline stages — `BoundsFound`, `SamplingGrid`,
`CompactingGrid`, `GeneratingEdges`, `GeneratingVerts`, `OctreeLayer`,
`SolvingQef`, `GeneratingQuad`, and `Done` — each carrying a `done`/`total`
pair or a `layer` counter.  `progress_fraction()` maps every event to a scalar
in `[0, 1]` using per-stage weight heuristics, so values are monotonically
non-decreasing and reach exactly `1.0` at `Done`.

The callback runs on the same thread and requires no channels, atomics, or
shared state, making it compatible with single-threaded WASM runtimes.

# Algorithm

The implementation follows [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf) in roughly these steps:

1. **Sample value grid** — The grid is *not* sampled densely. Instead the bounding box is recursively subdivided octree-style, starting at the next power-of-two size that covers the bbox. At each level the implicit function is evaluated at the 8 sub-cube corners. Because the function is required to satisfy `|value| >= distance_to_surface`, a sub-cube can be skipped entirely when `|value| > diagonal_of_sub_cube` — the surface cannot possibly pass through it. Only when a sub-cube cannot be skipped and has reached unit size (one grid cell) is the value stored. This means only the cells near the surface are sampled at full resolution; the rest of space is never visited.

2. **Compact value grid** — Drop all grid corners that have no sign-change neighbor. This reduces memory by ~10× while keeping all corners adjacent to the surface.

3. **Generate edge grid** — For each grid edge whose two endpoints have opposite signs, find the exact zero crossing using a Newton-on-edge step (projecting the gradient onto the edge direction) with bisection fallback, and record the surface position and normal as a tangent plane.

4. **Generate leaf vertices** — For each grid cell that contains at least one active edge, create one leaf vertex and accumulate the tangent planes from all crossing edges into a Quadratic Error Function (QEF).

5. **Build octree** — Repeatedly subsample the leaf layer: connected vertices that map to the same parent cell are merged by summing their QEFs. This continues until the layer size stabilises. A manifold check (Euler characteristic + per-face edge-count) decides whether a group may be merged.

6. **Solve QEFs top-down** — Starting from the coarsest octree layer, solve each merged QEF to find the vertex position that minimises squared distance to all contributing tangent planes. If the error is below the threshold the coarse vertex is used; otherwise the algorithm recurses into the children.

7. **Generate quads** — For each active edge, look up the four dual cells that share it and connect their solved vertices into a quad (two triangles), forming the final mesh.

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
