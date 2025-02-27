![Build Status](https://travis-ci.org/andrewdavidmackenzie/libproc-rs.svg?branch=master "Mac OS X")
[![codecov](https://codecov.io/gh/andrewdavidmackenzie/libproc-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/andrewdavidmackenzie/libproc-rs)

# libproc-rs
This is a library for getting information about running processes for Mac OS X and Linux.

# Using it
```
extern crate libproc;
use libproc::libproc::proc_pid;

match proc_pid::pidpath(pid) {
    Ok(path) => println!("PID {}: has path {}", pid, path),
    Err(err) => writeln!(&mut std::io::stderr(), "Error: {}", err).unwrap()
}
```

# Documentation
[Online code docs](https://andrewdavidmackenzie.github.io/libproc-rs/libproc/)

# API
At the moment these methods have been implemented:

## Process / PID related
```
pub fn listpids(proc_types: ProcType) -> Result<Vec<u32>, String> (macos) (linux)
```
```
pub fn listpidspath(proc_types: ProcType, path: &str) -> Result<Vec<u32>, String> (macos) (linux)
```
```
pub fn pidinfo<T: PIDInfo>(pid : i32, arg: u64) -> Result<T, String> (macos)
```
```
pub fn regionfilename(pid: i32, address: u64) -> Result<String, String> (macos)
```
```
pub fn pidpath(pid : i32) -> Result<String, String> (macos) (linux)
```
```
pub fn libversion() -> Result<(i32, i32), String> (macos)
```
```
pub fn name(pid: i32) -> Result<String, String> (linux) (macos)
```
```
pub fn listpidinfo<T: ListPIDInfo>(pid : i32, max_len: usize) -> Result<Vec<T::Item>, String> (macos)
```
```
pub fn pidcwd(pid: pid_t) -> Result<PathBuf, String> (linux)
```
```
pub fn cwdself() -> Result<PathBuf, String> (linux)
```

## File and FileDescriptor related
```
pub fn pidfdinfo<T: PIDFDInfo>(pid : i32, fd: i32) -> Result<T, String> (macos)
```

## PID Resource Usage related
(Added in Mac OS X 10.9 - under "macosx_10_9" feature)
```
pub fn pidrusage<T: PIDRUsage>(pid : i32) -> Result<T, String> (macos)
```

## Kernel Message Buffer - kmsgbuf
```
pub fn kmsgbuf() -> Result<String, String>
```

# Binaries
`cargo build` builds the following binaries:
- `procinfo` that takes a PID as an optional argument (uses it's own pid if none supplied) and returns information about the process on stdout
- `dmesg` is a version of dmesg implemented in rust that uses libproc-rs.

# Rust Versions
The Github Actions CI matrix tests for rust `stable`, `beta` and `nightly` on all platforms

# Platforms
Mac OS X (10.5 and above) and Linux.

## Mac OS X Versions
Calls were aded to libproc in Mac OS X 10.7 and again in 10.9. 
This library can be compiled to not include those calls by using rust `features`
to enable/disable support for those versions.

The default build is for Mac OS 10.9 or later. See: 
```toml
[features]
default = ["macosx_10_9"]
macosx_10_7 = []
macosx_10_9 = ["macosx_10_7"]
```
in Cargo.toml

To build for versions prior to Mac OS 10.7 disable the default features by passing --no-default-features to cargo.

To build for Mac OS X 10.7 (or 10.8) you can enable that feature alone using
--no-default-features --features "macosx_10_7"

# Build and Test
`cargo test` should build and test as usual for rust projects.

However, as some functions need to be run as `root` to work, I run travis-CI tests as `root`. So, when developing in local
it's best if you use `sudo cargo test`. NOTE: This can get you into permissions problems when switching back and for
between using `cargo test` and `sudo cargo test`. To fix that run `sudo cargo clean` and then build or test as you prefer.

In order to have tests pass when run as `root` or not, some tests need to check if they are `root` at run-time 
(using our own `am_root()` function is handy) and avoid failing if *not* run as `root`. 

## Macos: clang detection and header file finding
Newer versions of `bindgen` have improved the detection of `clang` and hence macos header files. 
If you also have llvm/clang installed directly or via `brew` this may cause the build to fail saying it 
cannot find `libproc.h`. This can be fixed by setting `CLANG_PATH="/usr/bin/clang"` so that `bindgen` 
detects the Xcode version and hence can fidn the correct header files.

# CI Testing
Continuous Integration testing has been moved from travis-ci to GitHub Actions. For details see the 
[rust.yml](`.github/workflows/rust.yml`) GitHub Actions workflow

# Input Requested
* Suggestions for API, module re-org and cross-platform abstractions are welcome.
* How to do error reporting? Define own new Errors, or keep simple with Strings?
* Would like Path/PathBuf returned when it makes sense instead of String?

# TODO
See the [list of issues](https://github.com/andrewdavidmackenzie/libproc-rs/issues). 
I put the "help wanted" label where I need help from others.
 
- Look at what similar methods could be implemented as a starting point on Linux
- Complete the API on Mac OS X - figuring out all the Mac OS X / Darwin version mess....
- Add more documentation (including samples with documentation test)
- Add own custom error type and implement From::from to ease reporting of multiple error types in clients

# LICENSE
This code is licensed under MIT license (see LICENCE).

# CONTRIBUTING
You are welcome to fork this repo and make a pull request, or write an issue.
