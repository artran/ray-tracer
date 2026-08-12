# Ray Tracer

A from-scratch ray tracer written in Rust, following the exercises in Jamis Buck's *The Ray Tracer Challenge*. It builds a scene out of spheres and planes with a single point light, shades each pixel with Phong lighting plus shadows, and writes the result as a PPM image.

## Quick start

```bash
cargo build              # build the whole workspace
cargo run                # render the sample scene to /tmp/scene.ppm
cargo test --workspace   # run the full test suite (167 tests)
```

To view the rendered image, convert the PPM with ImageMagick:

```bash
convert /tmp/scene.ppm /tmp/scene.png
```

## What the sample scene contains

`src/main.rs` defines a small fixed scene:

- a matte pink **floor plane** and **rear wall plane**
- three **spheres** — a green one at `(-0.5, 1.0, 0.5)`, a lime one at `(1.5, 0.5, -0.5)` scaled to half size, and a gold one at `(-1.5, 0.33, -0.75)` scaled to a third
- a single white **point light** at `(-10, 10, -10)` (the default)
- a 1000×750 **camera** with a 60° field of view at `(0, 1.5, -5)`, looking at the origin

Every object is assembled with a builder, wrapped in an `Rc`, and registered on a `WorldBuilder`:

```rust
let world = WorldBuilder::new()
    .with_object(Rc::new(floor))
    .with_object(Rc::new(rear_wall))
    .with_object(Rc::new(middle))
    .with_object(Rc::new(right))
    .with_object(Rc::new(left))
    .build();

let camera = CameraBuilder::new()
    .with_hsize(1000)
    .with_vsize(750)
    .with_field_of_view(PI / 3.0)
    .with_transform(Matrix::view_transform(
        Vector4::point(0.0, 1.5, -5.0),
        Vector4::point(0.0, 1.0, 0.0),
        Vector4::vector(0.0, 1.0, 0.0),
    ))
    .build();

let canvas = camera.render(&world);
```

## Workspace layout

The repository is a Cargo workspace with two packages:

| Package | Location | Purpose |
|---|---|---|
| `ray-tracer` | repo root | The renderer: scene graph, shapes, materials, patterns, camera, PPM output |
| `ray-math` | `crates/math/` | The linear-algebra primitives: `Matrix`, `Vector4`, `EPSILON` |

### `ray-tracer` modules

| Module | Contents |
|---|---|
| `camera` | `Camera` + `CameraBuilder`. Casts one ray per pixel through a view transform and renders a `World` into a `Canvas`. Precomputes the inverse transform and pixel size at build time. |
| `canvas` | `Canvas` pixel buffer. `save(&mut impl Write)` serializes it as PPM. |
| `color` | `Color` with `r`, `g`, `b` `f32` fields, arithmetic operators, and `black()` / `white()` helpers. |
| `consts` | Re-exports `EPSILON` from `ray-math` for tolerance-based float comparisons. |
| `intersection` | `Intersection`, `Intersections`, and `Computations` — hit selection and the per-hit data (point, eye/normal vectors, over-point) used by shading. |
| `light` | `PointLight` (position + intensity). The world holds exactly one. |
| `material` | `Material` + `MaterialBuilder` — Phong parameters (`ambient`, `diffuse`, `specular`, `shininess`) plus a pattern. |
| `pattern` | The `Pattern` trait with two implementations: `solid` and `stripes`. |
| `ray` | `Ray` (a point origin and vector direction) with `position(t)` and `transform`. |
| `shape` | The `Shape` trait with `sphere` and `plane` implementations. |
| `transform` | The `Transform` trait, implemented for `Matrix<4>`: translation, scaling, rotation (x/y/z), shearing, and `view_transform`. |
| `world` | `World` + `WorldBuilder` — the scene's objects and light, plus the intersection and shading pipeline. |

### `ray-math` crate

- `Vector4` — points and vectors in one type, distinguished by `w` (`is_point` / `is_vector`); includes `dot`, `cross_product`, `reflect`, `magnitude`, `normalize`.
- `Matrix<const L: usize>` — const-generic square matrices with `determinant`, `submatrix`, `minor`, `cofactor`, `transpose`, `identity`, `try_inverse`.
- `EPSILON` — `1.0e-05`, the shared float tolerance.

## How a frame is rendered

1. `Camera::render(&world)` loops over pixels; `ray_for_pixel` casts a ray through the camera's inverse transform.
2. `World::intersect(&ray)` asks every shape for its intersections and pools them.
3. `Intersections::hit()` picks the nearest positive `t`.
4. `prepare_computations` derives the hit point, surface normal, eye vector, and an over-point nudged along the normal to avoid acne.
5. `World::shade_hit` calls `Material::lighting`, which applies Phong shading (ambient + diffuse + specular) and returns early with ambient-only color when the point is in shadow (`World::is_shadowed` casts a shadow ray toward the light).
6. The color lands in the `Canvas`, which is saved as PPM.

## Design conventions

New code should follow the established patterns:

- **Builders everywhere** — `XBuilder::new().with_*(...).build()`. Setters consume and return `self`. The camera and shapes precompute stored invariants (inverse transform, pixel size) during `build()`.
- **Traits over concrete types** — shape-like and pattern-like behavior plugs into the `Shape` / `Pattern` traits rather than branching on types. Shared ownership is `Rc<dyn Shape>` / `Rc<dyn Pattern>`. Equality across trait objects uses downcasting via `as_any` plus a `shape_eq` / `pattern_eq` hook.
- **Inverse transforms are cached** — shapes store `inv_transform` so ray intersection and normal computation don't invert per ray.
- **`f32` throughout**, with `EPSILON`-based comparisons instead of exact equality in production code.
- **Errors via `Result`** — e.g. `Canvas::save` (I/O) and `Matrix::try_inverse` (non-invertible). Hard invariants, like `Ray::new` requiring a point origin, are asserted at construction.

### Adding a shape or pattern

Implement the trait and register it in the module:

```rust
// shape: implement local_intersect, local_normal_at, material,
// transformation, inv_transform, lighting, plus the as_any/shape_eq hooks
impl Shape for MyShape { /* ... */ }
```

Then export it from `src/shape/mod.rs` (or `src/pattern/mod.rs`) and add a builder following `SphereBuilder` / `PlaneBuilder`.

## Testing

Tests live beside the code under `#[cfg(test)]` in each module. The suite uses `rstest` for parameterized cases and `spectral` for float closeness assertions. 167 tests total: 119 in `ray-tracer`, 48 in `ray-math`.

```bash
cargo test --workspace                        # everything
cargo test camera::tests                      # one module's tests
cargo test -p ray-math matrix::tests          # a module in the math crate
```

## Tooling

- `cargo fmt` — rustfmt across the workspace (`cargo fmt --check` to verify).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — the lint gate; keep it clean.
- `pre-commit` — trailing-whitespace, end-of-file-fixer, YAML and large-file checks (`.pre-commit-config.yaml`).

## Status and known gaps

Implemented: spheres and planes, translation/scaling/rotation/shearing transforms, `view_transform`, solid and stripe patterns, Phong shading, shadows, PPM output.

Not yet implemented (good next steps): reflection and refraction, transparency, multiple light sources, CSG or imported meshes, anti-aliasing, and non-PPM output formats. The `World` currently holds a single `PointLight` and `shade_hit` returns lighting only.
