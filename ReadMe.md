# Rust zeromq sandbox

Currently, there are two executables in the project, `zmq_subscriber` and `monster_publisher`.  
The publisher loops and publishes the same message bytes on each iteration (for now). The subscriber
will deserialize the message bytes and write a summary stdout.

These executables use the flatbuffers *Monster* example schema.  You must use a new `flatc` compiler (`23.5.26+`). 
There is a breaking change for rust generated files when mixing old compiler output with new
_flatbuffers_ dependency libs. The build step (see **build.rs**) uses the `flatc` compiler. Specify the
absolute path in an environment variable:
 - `FLATC_DIR`

A clean solution (that will keep IDEs happy) is to create `.cargo/config.toml` with this:
```toml
[env]
FLATC_DIR = "<absolute path to a good flatc>"
```

## Monsters
I picked the flatbuffers monster example as a use case.  The schema is in the **fb_schema** folder. 

### terminal 1, publish

```console
> cargo run --bin monster_publisher 127.0.0.1 8756

Publishing monsters on 127.0.0.1:8756

```

### terminal 2, subscribe

```console
> cargo run --bin zmq_subscriber 127.0.0.1 8756 monster

Subscribing to "monster", on 127.0.0.1:8756

Message 0
topic: monster
name: Troll, mana: 199, hp: 150
color: Red
position: [21,22,13]
weapon: Club
weapon: Other Club
...
```

## Other examples
Further examples can be found in `src/bin` subfolders. 