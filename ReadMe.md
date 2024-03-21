# Rust zeromq sandbox

Currently, the one binary is a localhost SUB client.

It defaults to port `9876` and topic "hotdogs" but port and topic can be added to the command line invocation.

```
./rust_zeromq_box 6789 nothotdog
```