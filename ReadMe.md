# Rust zeromq sandbox

Currently, the one binary is a localhost SUB client.

Added _flatbuffers_ messaging.  You must use a new `flatc` compiler, `23.5.26+`. 
There is a breaking change for rust generated files when mixing old compiler output with new
_flatbuffers_ libs.

The build step (see **build.rs**) uses the `flatc` and `protoc` compilers. Specify their 
absolute paths in environment variables:
 - `FLATC_DIR`
 - `PROTOC_DIR`

## Monsters
I picked the flatbuffers monster example as use a case.  The schema is in the **fb_schema** folder. 

### terminal 1, publish

```console
> cargo run --bin monster_publisher 127.0.0.1 8756
```

### terminal 2, subscribe

```console
> cargo run --bin zmq_subscriber 127.0.0.1 8756 monster
```