# Simple Config

## why
Using a stardard place for storing an app configuration helps to reduce a polution of 
random directories over PC hard drive.

Use the crate to get a configuration directory for your platform and then store the configuration there.

```rust
let mut config_path = simcfg::get_config_root())).unwrap();
config_path.push("my app name");
// push a config file name here
....
```

## build the crate

Unless you plan to use the crate in sources, you can build it as a binary lib.
Obtain [RustBee](https://github.com/vernisaz/rust_bee) unless  you already have it.

Run **rb** in the root of the crate. The crate will be stored in the `crate_dir` directory.