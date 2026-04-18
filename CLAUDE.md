# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
# Build the workspace
cargo build

# Run a specific example (most common workflow)
cargo run --example tinyrenderer
cargo run --example tinytracer
cargo run --example raytracer
cargo run --example runner
cargo run --example threerenderer

# Run tests
cargo test
cargo test -p tinyrenderer   # test a specific crate

# Check without building
cargo check
```

Resources (OBJ models, textures) are expected at `./resources/<folder_name>/`.

## Architecture Overview

This is a Cargo workspace implementing a software 3D renderer from scratch, organized as:

- **`math/`** — core math library: `Vec2/3/4`, `Mat3/4`, `Quaternion`, `Euler`, `Barycentric`, `BoundaryBox`, `Frustum`. All other crates depend on this.
- **`tinyrenderer/`** — the primary CPU rasterizer. Key pipeline: `obj_loader` parses `.obj`/`.mtl` files into a `Scene` → `Renderer::render()` runs per-triangle vertex+fragment shaders → writes to a `ColorBuffer`. Uses a z-buffer (`DepthBuffer`) for occlusion.
- **`renderer_macro_derive/`** — proc-macro crate. The `#[object_3d(TraitName)]` attribute generates boilerplate for scene-graph nodes (parent/child links, transform decompose/compose, event emitter, UUID). The `#[light_shadow(TraitName)]` attribute generates shadow map plumbing.
- **`sandbox/`** — thin FLTK window wrapper. `Sandbox::run_fltk(cb)` opens a window and calls `cb` every frame; `make_draw_image()` returns a closure that blits an RGB byte slice to the window.
- **`demo/`** — runnable examples that wire everything together (not a library crate).
- **`tinytracer/`** — ray tracer with sphere objects, materials, and lights. (Currently deprioritized.)
- **`three/`** — experimental WebGL-style scene graph (cameras, lights, materials, GL renderer). Uses the `#[object_3d]` macro. (Currently deprioritized.)

### Shader system (`tinyrenderer`)

Shaders are Rust closures stored in a `Shader` struct (`vertex: Box<dyn Fn(...)>`, `fragment: Box<dyn Fn(...)>`). Data flows through:
- `Uniform` — per-draw constants (MVP matrices, etc.), split into global and per-material maps.
- `Varyings` — vertex-shader outputs collected per-triangle, then perspective-correct lerped in `Shader::lerp_varyings()`.
- `GLTypes` enum — tagged union for `i32/f32/Vec2/Vec3/Vec4/Mat4` values; use the `uniform!(store, Type, "key", !)` macro to extract with a panic on missing keys, or without `!` for `Option`.

Built-in shaders are in `tinyrenderer/src/renderer/shader/`: `gouraud`, `phong`, `shadow`.
