# AGENTS.md

This file gives repository-specific guidance to coding agents working in this Rust ray tracer.

## Project Snapshot

- Language: Rust
- Crate type: single binary crate (`src/main.rs`)
- Edition: Rust 2024 (`Cargo.toml`)
- Domain: ray tracing primitives, materials, patterns, transforms, intersections, and scene rendering
- Test layout: unit tests live beside implementation files under `#[cfg(test)]`
- Dev dependencies used by tests: `rstest`, `spectral`

## Repository Rules Files

- Cursor rules in `.cursor/rules/`: none found
- `.cursorrules`: none found
- Copilot instructions in `.github/copilot-instructions.md`: none found
- Do not assume any hidden editor-specific rules beyond what is written here

## Commands

Run all commands from the repository root:

```bash
cargo build
```

- Builds the binary crate in debug mode

```bash
cargo run
```

- Runs the scene renderer in `src/main.rs`
- Current program writes a PPM file to `/tmp/scene.ppm`

```bash
cargo test
```

- Runs the full unit test suite (`167` tests currently)

```bash
cargo test camera::tests::constructing_a_camera -- --exact
```

- Runs a single exact unit test
- Use this pattern for focused verification after small changes

```bash
cargo test world::tests::test_is_shadowed
```

- Runs all tests whose names contain the given substring
- Useful when iterating on a module or behavior family

```bash
cargo fmt --check
```

- Checks formatting without rewriting files

```bash
cargo fmt
```

- Applies standard rustfmt formatting
- Use after edits unless the file already matches rustfmt output

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

- Preferred lint command
- Current baseline fails on `clippy::manual_is_multiple_of` in `src/matrix.rs`

## How To Run One Test

- Exact test: `cargo test module::tests::name -- --exact`
- Substring match: `cargo test partial_name`
- Module-focused sweep: `cargo test shape::sphere::tests`
- When changing math-heavy code, prefer one targeted test first, then `cargo test`

## Code Organization

- Core modules live directly under `src/`; subdomains use submodules like `src/shape/` and `src/pattern/`
- `src/main.rs` declares all top-level modules and contains a sample scene setup
- Many domain types use builders: `CameraBuilder`, `MaterialBuilder`, `SphereBuilder`, `PlaneBuilder`, `WorldBuilder`
- Traits define polymorphic behavior: `Shape`, `Pattern`, `Transform`
- Shared ownership is modeled with `Rc<dyn Shape>` and `Rc<dyn Pattern>`

## Imports

- Prefer grouping imports in this order: standard library, external crates, then `crate::...`
- Keep imports explicit; the codebase usually names concrete items directly
- Test modules often use `use super::*;` plus a few targeted `crate::...` imports
- Be aware that some files rely on root-level aliases like `crate::Color` and `crate::Vector4`; do not introduce them casually in new code unless the surrounding file already uses that pattern

## Formatting Conventions

- Use default rustfmt formatting
- Indent with four spaces, no tabs
- Keep chained builder calls one per line when they stop fitting comfortably on one line
- Multi-line comments are rare; prefer short doc comments or no comment at all
- Existing files separate tests with a long `/* --- Tests --- */` divider; preserve it when editing an existing file that already uses it

## Types And Numeric Conventions

- Use `f32` consistently for geometry, colors, transforms, and ray math
- Reuse `EPSILON` from `src/consts.rs` for floating-point tolerance checks
- For float comparisons in production code, prefer tolerance-based comparisons when exact equality is unstable
- For tests, use `spectral` closeness assertions (`is_close_to`) for non-trivial float results
- `Matrix` uses const generics (`Matrix<const L: usize>`); keep that style when extending matrix operations

## Naming Conventions

- Types and traits: `UpperCamelCase`
- Functions, methods, variables, modules, and tests: `snake_case`
- Builder setters follow `with_*` naming and consume `self`
- Constructors usually use `new`; default values use `Default` or builder defaults
- Use descriptive math/domain names like `eye_vector`, `normal_vector`, `light_source`, `inv_transform`
- Test names are sentence-like and behavior-oriented; continue that style

## Error Handling And Panics

- Use `Result` for genuinely fallible I/O or algebraic operations, e.g. `Canvas::save` and `Matrix::try_inverse`
- Use assertions for hard invariants at construction boundaries, e.g. `Ray::new` requires a point origin and vector direction
- Avoid adding new `unwrap()` calls unless the invariant is already guaranteed locally and matches current style
- Prefer propagating I/O errors with `?` rather than swallowing them
- Existing code sometimes uses `panic!` for impossible index cases; keep new panic sites rare and justified

## Trait And Builder Patterns

- New shape-like behavior should usually plug into the existing trait-based design rather than branching on concrete types
- If you add a new configurable domain object, prefer a small builder with `with_*` setters and a final `build()`
- Builders typically hold concrete values, then precompute stored invariants during `build()`
- Shapes store inverse transforms for performance; preserve that pattern when extending shape types

## Testing Conventions

- Keep tests in the same file as the implementation unless there is a strong reason not to
- Use plain `#[test]` for simple cases
- Use `rstest` fixtures and `#[case]` tables for repeated setup or parameterized scenarios
- Use `spectral::assert_that!` rather than raw `assert_eq!` when matching existing nearby tests
- Arrange/Act/Assert comments appear occasionally; only keep them when they clarify a longer test

## Implementation Preferences

- Prefer small, composable methods over large monolithic functions
- Prefer returning expressions directly instead of introducing temporary variables without a readability benefit
- Use early returns for guard clauses, especially in intersection and shading code
- Keep ownership simple; clone `Rc` handles instead of introducing unnecessary lifetimes
- Do not add dependencies without a strong reason; the crate is intentionally lightweight

## When Editing Existing Files

- Match the local style of the file first, even if another file does it slightly differently
- Preserve existing public APIs unless the task explicitly calls for a refactor
- Be careful around hand-rolled math and geometric invariants; small changes can invalidate many tests
- If you change render behavior, run at least the most relevant module tests plus `cargo test`
- If you touch formatting-only code, do not mix unrelated behavioral edits into the same change

## Practical Checklist For Agents

- Read the target module and its colocated tests before changing behavior
- Prefer the smallest safe change that fits the current architecture
- Run a focused single test first when possible
- Run `cargo test` before finishing
- Mention the current Clippy baseline issue if linting is part of your report
