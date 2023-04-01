[![Crates.io](https://img.shields.io/crates/v/bevy_fpc.svg)](https://crates.io/crates/bevy_fpc)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://codeberg.org/Eternahl/bevy_fpc#license)

First-person-controller plugin for the [Bevy](https://bevyengine.org) game-engine

# Usage

Plugins initialization:

```rust
// Require the `bevy_rapier3d` plugin
App::new()
	.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
	.add_plugin(FpcPlugin::default())
```

Spawn and embody an `fpc` entity:

```rust
commands.spawn(FpcBundle::default()).insert(Player);
```

Custom configuration:

```rust
app.insert_resource(FpcConfiguration{
  keyboard_linear_inputs: LINEAR_AZERTY_LAYOUT,
  ..Default::default()
})
```

Try out the example by cloning this repo and running the following command:

```rust
// In this example, you can press the `Tab` key to switch the "angular state"
cargo run --example basic
```
Example map model by [noyou](https://sketchfab.com/3d-models/game-pirate-adventure-map-696dfa212fda4240817615bdccb373d0), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

# Docs/Features

- `bevy_fpc` main crate ([Rust API Docs](https://docs.rs/bevy_fpc))
- `bevy_fpc_sprint` (default) add sprinting capability to the controller

# License

Licensed under either of

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.