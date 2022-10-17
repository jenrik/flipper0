# Bindings for Flipper Zero

Automatically generated bindings (or "externs") for [Flipper Zero Fw][] with some little hand-crafted wrappers and additions as upper abstraction layer.


Useful for:
- FAPs (default features, without `all-symbols`)
- Fw modules, services and built-in apps because there are building from source, so includes all symbols (use feature `all-symbols`)


__`---> `[Examples][]` <---`__


## State

![Maintenance Status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

Current state of the project is WiP. _Highlly & dirty work-in-progress._
Any contribution are appreciated, you know.

Supported (means "tested with") fw version: __0.69.0__ but should work normally with any 0.68.x-0.69.x without `prebuild` feature.


## Prerequisites

- Rust toolchain (currently only `nightly` supported)
- target `thumbv7em-none-eabihf`
- `libclang` for [bindgen][bingen+clang]
- clone of [Flipper Zero firmware][Flipper Zero Fw]
- ARM toolchain, run `fbt` to easily get it


Add this as dependency to your cargo manifest file:
```
[dependencies.flipper0]
version = "0.1"
default-features = false # disable prebuild
```

To build just add `FLIPPER_FW_SRC_PATH` to your ENV anyhow (config, build-script, shell-rc, etc..), for example run:
```
FLIPPER_FW_SRC_PATH=~/path/to/flipperzero-firmware/ cargo build
```
Or without SDK, that will be downloaded from the official repository:
```
FLIPPER_REPO_BRANCH=release cargo build
```


## Build Configuration

### Environment variables:
| Feature                   | Required | Description                                                                                                                                               | Use with feature                  |
| ------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `FLIPPER_FW_SRC_PATH`       | required | Needed to build from source in local working copy of [firmware repo][Flipper Zero Fw], points to root of the repo.                                        | `use-local-sdk`                   |
| `ARM_TOOLCHAIN`           | optional | If omitted build-script will search it in the working copy of the [firmware repo][Flipper Zero Fw]. Typically should points to "arm-none-eabi" directory. | `use-local-sdk`, `use-remote-sdk` |
| `FLIPPER_REPO_REV`        | optional | Revision or tag.                                                                                                                                          | `use-remote-sdk`                  |
| `FLIPPER_REPO_BRANCH`     | optional | Name of branch.                                                                                                                                           | `use-remote-sdk`                  |
| `FLIPPER_REPO_CLONE_PATH` | optional | Path points to directory where the SDK repository will be cloned. Default is `OUT_DIR/flipperzero-firmware`.                                              | `use-remote-sdk`                  |


### Features:

- `allocator`: default, include global allocator implementation
  - `oom`: default, out-of-mem handler. Disable it to use you custom handler or `#![feature(default_alloc_error_handler)]`.
- `panic`: default, include global panic & OoM handler

Build methods features:

| Feature          | Deafault | Description                                                            | Used ENV vars                                                                                             |
| ---------------- | -------- | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `prebuild`       | +        | use pre-generated bindings                                             |                                                                                                           |
| `use-local-sdk`  | +        | look at `FLIPPER_FW_SRC_PATH`, build from source                         | `FLIPPER_FW_SRC_PATH` (required), `ARM_TOOLCHAIN` (optional)                                                |
| `use-remote-sdk` | -        | clone remote git repo, initial setup with fbt, then build from source. | `FLIPPER_REPO_REV`, `FLIPPER_REPO_BRANCH`, `FLIPPER_REPO_CLONE_PATH`, `ARM_TOOLCHAIN` (all vars optional) |

_`prebuild` is default feature just for ability to build crate out-of-the-box._



[bingen+clang]: https://github.com/rust-lang/rust-bindgen/issues/918
[Flipper Zero Fw]: https://github.com/flipperdevices/flipperzero-firmware/
[examples]: https://github.com/boozook/flipper0/blob/master/examples/
