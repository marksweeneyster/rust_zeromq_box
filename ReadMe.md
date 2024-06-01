# Rust zeromq sandbox

Currently, the one binary is a localhost SUB client.

Added _flatbuffers_ messaging.  You must use a new `flatc` compiler, `23.5.26+`. 
There is a breaking change for rust generated files when mixing old compiler output with new
_flatbuffers_ libs.

The build step (see **build.rs**) uses the `flatc` and `protoc` compilers. Specify their 
absolute paths in environment variables:
 - `FLATC_DIR`
 - `PROTOC_DIR`