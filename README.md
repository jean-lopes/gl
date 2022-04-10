# gl

## Your application setup

Instead of the default [gl](https://crates.io/crates/gl) crate, add the following to your project `Cargo.toml`
```toml
[dependencies.gl]
git = "https://github.com/jean-lopes/gl"
features = ["gl33"] # generate bindings for OpenGL 3.3 API

[features] 
gl_debug = ["gl/debug"] # creates a feature on your application to enable OpenGL debug
```

## Features

### OpenGL API Version
Create bindings for specific OpenGL API version.

| Feature | OpenGL version |
|:-------:|:--------------:|
| `gl10`  | 1.0            |
| `gl11`  | 1.1            |
| `gl12`  | 1.2            |
| `gl13`  | 1.3            |
| `gl14`  | 1.4            |
| `gl15`  | 1.5            |
| `gl20`  | 2.0            |
| `gl21`  | 2.1            |
| `gl30`  | 3.0            |
| `gl31`  | 3.1            |
| `gl32`  | 3.2            |
| `gl33`  | 3.3            |
| `gl40`  | 4.0            |
| `gl41`  | 4.1            |
| `gl42`  | 4.2            |
| `gl43`  | 4.3            |
| `gl44`  | 4.4            |
| `gl45`  | 4.5            |
| `gl46`  | 4.6            |

_default: `gl46`_

### OpenGL Profile

Enable Compatibility profile. More information about profiles on [khronos.org](https://www.khronos.org/opengl/wiki/OpenGL_Context#OpenGL_3.2_and_Profiles).

feature: `compatibility`

_default profile: Core_

### Function fallbacks

Enable fallback functions. Fallbacks are listed on [gl-rs](https://docs.rs/gl/latest/gl/index.html#functions).

feature: `fallbacks`

_default: disabled_

### OpenGL debug
To run your application tracing all OpenGL functions, and errors, build/run with:
```bash
cargo run --features gl_debug
```

Example `Cargo.toml` with multiple features enabled
```toml
[dependencies.gl]
git = "https://github.com/jean-lopes/gl"
features = ["gl11", "compatibility", "fallbacks"] # generate bindings for OpenGL 1.1 API, Compatibility profile and functions fallback

[features] 
gl_debug = ["gl/debug"] # creates a feature on your application to enable OpenGL debug
```

Inspired on https://nercury.github.io/rust/opengl/tutorial/2018/02/12/opengl-in-rust-from-scratch-06-gl-generator.html

