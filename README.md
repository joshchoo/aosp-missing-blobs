## aosp-missing-blobs

aosp-missing-blobs is a nifty tool to identify required blobs (.so) that are missing from AOSP ROM builds,
and to show which existing blobs rely on them. This will be particularly useful for ROM developers who
want to ensure that they have not missed out any required proprietary OEM blobs.

### Concept

Blobs (.so) and compiled binaries in Android need other blobs to work. It is possible to identify the
dependencies for each blob by inspecting their ELF information:

```
$ readelf -d <name_of_blob.so> | grep "\(NEEDED\)" | sed -r "s/.*\[(.*)\]/\1/"
```

Example (finding the dependencies of `libqti_performance.so`):

```
$ readelf -d libqti_performance.so | grep "\(NEEDED\)" | sed -r "s/.*\[(.*)\]/\1/"
libnativehelper.so
liblog.so
libcutils.so
libutils.so
libbase.so
vendor.qti.hardware.iop@2.0.so
libhidlbase.so
libhidltransport.so
libqti-perfd-client_system.so
libhwui.so
libc++.so
libc.so
libm.so
libdl.so
```

From the example, we can see that `libqti_performance.so` depends on `libqti-perfd-client_system.so`
and `vendor.qti.hardware.iop@2.0.so`.

aosp-missing-blobs uses this idea to identify all the dependencies of a given list of blobs, and to
check whether any of these dependencies are not present.

### Installation

#### Cargo

You can install with `cargo` after [setting up Rust](https://www.rust-lang.org/tools/install):

```
cargo install aosp-missing-blobs
```

### Building

aosp-missing-blobs is written in Rust, hence you'll need [Rust to be installed](https://www.rust-lang.org) to build the project.

In the root directory of the repo, execute:

```
$ git clone https://github.com/joshuous/aosp-missing-blobs
$ cd aosp-missing-blobs
$ cargo build --release

$ ./target/release/aosp-missing-blobs
```

A runnable `aosp-missing-blobs` binary will be produced.

### Usage

This program takes as arguments a **list of directories** that contain compiled binaries and blobs (.so).

```
$ aosp-missing-blobs <blob directory1> <blob directory2> <blob directory3> <blob directoryN>
```

Search blob directories recursively:

```
$ aosp-missing-blobs -r <blob root directory>
```

#### Example

Assuming you have extracted the ROM's system.img and vendor.img to `~/system` and `~/vendor`, respectively, run the following:

```
$ aosp-missing-blobs ~/system/lib ~/vendor/lib
```

The program does the following:

1. Search `~/system/lib` and `~/vendor/lib` directories for blobs.
2. Identify dependencies of each blob found.
3. Checks if required dependencies exist in `~/system/lib` and `~/vendor/lib`.
4. Output a list of missing dependencies and existing blobs that need them.

Depending on how wide your search scope is, the program may flag out missing dependencies 'incorrectly'.
Some dependencies may exist in other directories besides what the examples have shown.
Other directories with blobs include the following:

- /system/bin
- /system/lib[64]
- /system/lib[64]/vndk-28
- /system/lib[64]/vndk-sp-28
- /vendor/bin
- /vendor/lib[64]
- /vendor/lib[64]/vndk
- Etc.

Take note that the more directories you specify as arguments, the longer the program will run for!
