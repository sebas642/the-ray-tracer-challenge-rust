# The Ray Tracer Challenge

![Build Status](https://github.com/sebas642/the-ray-tracer-challenge-rust/workflows/Rust/badge.svg)

A partial implementation of [The Ray Tracer Challenge](https://pragprog.com/titles/jbtracer/the-ray-tracer-challenge/) book using Rust.
This is a side project to learn and experiment with the Rust programming language. Performance and idiomatic Rust code are not the main goals.

Debug mode is extremely slow. Sample apps should be executed in release mode. The output images are in PPM format.

To execute the latest chapter and show the resulting image:

```bash
cargo run --release | display
```

To get help on the command line options, you can execute:

```bash
cargo run --release -- --help
```

## Chapters

- [x] Chapter 1 - Tuples, Points, and Vectors
- [x] Chapter 2 - Drawing on a Canvas
- [x] Chapter 3 - Matrices
- [x] Chapter 4 - Matrix Transformations
- [x] Chapter 5 - Ray-Sphere Intersections
- [x] Chapter 6 - Light and Shading
- [x] Chapter 7 - Making a Scene
- [x] Chapter 8 - Shadows
- [x] Chapter 9 - Planes
- [x] Chapter 10 - Patterns
- [x] Chapter 11 - Reflection and Refraction
- [ ] Chapter 12 - Cubes
- [ ] Chapter 13 - Cylinders
- [ ] Chapter 14 - Groups
- [ ] Chapter 15 - Triangles
- [ ] Chapter 16 - Constructive Solid Geometry (CSG)
- [ ] Chapter 17 - Next Steps
- [ ] Chapter 18 - Rendering the Cover Image

## References

- [The Ray Tracer Challenge Errata](https://pragprog.com/titles/jbtracer/errata)
- [The Ray Tracer Challenge Forum](https://forum.raytracerchallenge.com/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/) or execute `rustup doc --book`
- [The Rust Reference](https://doc.rust-lang.org/stable/reference/) or execute `rustup doc --reference`
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) or execute `rustup doc --rust-by-example`

## Sample

![Sample Image](https://i.imgur.com/8CIOZPI.png)
