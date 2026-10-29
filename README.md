# Particle Toggle

A native [Horizon](https://playhorizon.dev) mod that shows or hides each
particle type, one by one: flames, smoke, drips, crits, explosions, totems,
all 121 that Horizon draws.

- **Options → Mods → Particle Toggle**: a button per particle type
  (`Flame: Shown` / `Flame: Hidden`), plus *Show all particles* and *Hide all
  particles*.
- **Chat command** `/particles`:

  | Command | Does |
  |---|---|
  | `/particles hide flame smoke` | hide types (`off` works too) |
  | `/particles show flame` | show them again (`on`) |
  | `/particles toggle crit` | flip them |
  | `/particles hide dripping_*` | `*` matches any part of a name; `all` matches every type |
  | `/particles list` | the hidden types |
  | `/particles types [text]` | every type name, or those containing `text` |
  | `/particles reset` | show everything |

  Names are the vanilla registry names (`minecraft:` is optional).

Your choices are saved in the instance's
`horizon-mods/.horizon-mod-data/particle-toggle/hidden` (one type per line)
and come back the next time you play. Turning the mod off in Options → Mods
shows every particle again without losing the list.

Hidden particles are removed from Horizon's particle engine each frame,
after the game's ticks and before drawing, so they never appear and cost
nothing to draw. This is client-side: other players still see their own
particles.

## Install

Native mods change Horizon's own Rust source and are compiled into a game
build of their own (see Horizon's
[native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md)).
They run with your user's rights and are **not sandboxed**, so read the
source first: it is the 9 small files in
[`mixins/particle-toggle/`](mixins/particle-toggle).

This version is pinned to Horizon **Nightly 2026-10-28 · 6f8c7a0**
(`horizon_revision` in `horizon-mixin.toml`), and the launcher builds it
on that commit.

```sh
git clone https://github.com/voduff/horizon-particle-toggle
horizon-launcher native-build horizon-particle-toggle/mixins/particle-toggle --trust-native-code
```

The first build downloads the launcher's own Rust toolchain and Horizon's
crates and compiles the whole game (around 10 minutes). The build goes to the
**Native mods** instance: pick it in the launcher and play.

## Develop

`runtime.rs` is both a standalone library (the filter, the settings page and
the command, with unit tests) and the module horizon-mixins injects into the
game as `crate::game::particle_toggle`. The other files are small hooks:

| File | Hook |
|---|---|
| `frame.rs` | `native_frame`: drops hidden types from the engine's live particles and queue, through two `GET_MUT` bridges on `ParticleEngine` |
| `tick.rs` | `native_tick`: follows the mod's switch in Options → Mods, loads the saved list |
| `list.rs`, `settings.rs`, `click.rs` | `native_mods`, `native_settings`, `native_settings_click`: the Options → Mods page |
| `command.rs` | `native_command`: `/particles` |

```sh
cargo test                     # runtime unit tests
cargo clippy --all-targets -- -D warnings
```

To check the hooks against the real game, clone
[horizon](https://github.com/horizon-rust/horizon) (with the pinned commit)
and [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) at the
revision in [compatibility.json](compatibility.json) beside this repository,
then:

```sh
python3 tools/horizon.py stage    # export the pinned source and apply the mixins
cargo fetch --locked --manifest-path target/mixins/modded/Cargo.toml
python3 tools/horizon.py check    # type-check the modded game
python3 tools/horizon.py test     # run the runtime tests inside it
```

Pass `--horizon PATH` and set `HORIZON_MIXINS=PATH` when they live elsewhere.
Built from
[horizon-native-mod-template](https://github.com/horizon-rust/horizon-native-mod-template).

Not an official Minecraft or Horizon product. MIT licensed.
