# Simple Config

## why
When you write any application, usually you need to store its configuration somewhere.
It's probably doesn't matter if your app for a personal use on one platform only.

However for applications with wide use on different platforms, this crate will help:

```rust
let mut config_path = simcfg::get_config_root())).unwrap();
config_path.push("my app name");
// add config file name here
....
```
