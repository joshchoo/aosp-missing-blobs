AospMissingBlobs
----------------

AospMissingBlobs is a nifty tool to identify required blobs (.so) that are missing from AOSP ROM builds,
and to show which existing blobs rely on them. This will be particularly useful for ROM developers who 
want to ensure that they have not missed out any required proprietary OEM blobs.

### Concept
Blobs (.so) and compiled binaries in Android need other blobs to work. It is possible to identify the
dependencies for each blob by running the following command:

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

AospMissingBlobs uses this idea to identify all the dependencies of a given list of blobs, and to
check whether any of these dependencies are not present.

### Requirements
- readelf
- grep
- sed
- java compiler

### Building
In the source file directory, execute:
```
$ javac *.java
$ jar -cvfe MissingBlobs.jar MissingBlobs *.class
```

A runnable `MissingBlobs.jar` will be produced.

Alternatively, you can load this project in your favorite Java IDE and create an executable jar.

### Usage
This program takes as arguments a list of directories that contain compiled binaries and blobs (.so).

```
$ java -jar MissingBlobs.jar <blob directory1> <blob directory2> <blob directory3> <...>
```

#### Example
Assuming you have extracted the ROM's system.img and vendor.img to `~/system` and `~/vendor`, respectively, run the following:

```
$ java -jar MissingBlobs.jar ~/system/lib ~/vendor/lib
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

### Downloads
`MissingBlobs.jar` downloads can be found in the [Releases](https://github.com/joshuous/AospMissingBlobs/releases).
