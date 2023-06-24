[![Crates.io](https://img.shields.io/crates/v/bevy_fpc.svg)](https://crates.io/crates/bevy_fpc)
[![MIT/Apache 2.0](https://img.shields.io/crates/l/bevy_fpc)](https://codeberg.org/Eternahl/bevy_fpc#license)
[![docs.rs](https://docs.rs/bevy_fpc/badge.svg)](https://docs.rs/bevy_fpc)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
![Pipeline status](https://ci.codeberg.org/api/badges/Eternahl/bevy_fpc/status.svg)

First-person-controller plugin for the [Bevy](https://bevyengine.org) game-engine.

The controller benefits from the features offered by the [`rapier character controller`](https://rapier.rs/docs/user_guides/bevy_plugin/character_controller).

# Usage

Plugins initialization:

```rust
// Require the `bevy_rapier3d` crate
# use bevy_fpc::FpcPlugin;
# use bevy::prelude::*;
# use bevy_rapier3d::prelude::*;
App::new()
	.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
	.add_plugin(FpcPlugin);
```

Spawn and embody an `fpc` entity:

```rust
# use bevy_fpc::{FpcBundle, Player};
# use bevy::prelude::{Commands};
# fn init(mut commands: Commands) {
	commands.spawn(FpcBundle::default()).insert(Player);
# }
```

Custom configuration:

```rust
# use bevy_fpc::{FpcConfiguration, LINEAR_AZERTY_LAYOUT};
# use bevy::prelude::{App, Commands};
# let mut app = App::new();
app.insert_resource(FpcConfiguration{
  keyboard_linear_inputs: LINEAR_AZERTY_LAYOUT,
  ..Default::default()
});
```

Try out the example by cloning this repo and running the following command:

```sh
# In this example, you can press the `Tab` key to switch the "angular state"
cargo run --example basic
```
Example map model [Temple ruins](https://sketchfab.com/3d-models/temple-ruins-6b3eb4e27e03485a886ce5304e95f897) by [Deyama](https://sketchfab.com/deyama), licensed under [Creative Commons Attribution](http://creativecommons.org/licenses/by/4.0/)

# Docs/Features

- [`bevy_fpc`](https://docs.rs/bevy_fpc) main crate / API
- [`bevy_fpc_core`](https://docs.rs/bevy_fpc_core) add minimal features to the controller (walking and looking)
- [`bevy_fpc_sprint`](https://docs.rs/bevy_fpc_sprint) (default) add sprinting capability to the controller

# Version compatibility

| bevy  | bevy_fpc |
|-------|----------|
| 0.10  | 0.1      |

# License

Licensed under either of

- MIT License ([LICENSE-MIT](https://codeberg.org/Eternahl/bevy_fpc/src/branch/trunk/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](https://codeberg.org/Eternahl/bevy_fpc/src/branch/trunk/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.