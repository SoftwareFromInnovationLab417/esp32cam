# Esp32Cam development board

This project started at: 2023/4/12

This project using `rust` as programming language, and using a component of camera from `esp32Cam` written by `c`.

Firstly, you can git clone the `esp32Cam` repository to the folder `component`, and then create a `bindings.h` in `/src`,
after that write this to the `Cargo.toml`:

```toml
[package.metadata.esp-idf-sys]
extra_components = [
  { component_dirs = [
    "component",
  ], bindings_header = "src/bindings.h" },
]
```

Then, run the command `cargo build --release`, it would meet the error, some fields name was conflicted.

we can use some searching tools such as `everythings` to search for a file named `bindings.rs`, which it may create by the building stage.

and then, you should replace the conflict field name `resolution` into `resolution_`, and then run command `cargo build --release` again,
you will find that the component is available(using these through the `esp_idf_sys`,
you should reference the `binding.rs` api as well), after that no errors any more.

How to use camera? [camera relevant project code](https://github.com/ChenHuaYou/esp-cam-rs/blob/main/src/main.rs#L129)

How to use wifi? [wifi relevant project code](https://github.com/ivmarkov/rust-esp32-std-demo/blob/main/src/main.rs#L1113)
