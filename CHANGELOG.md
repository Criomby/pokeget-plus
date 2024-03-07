## 1.6.1

### What's new:

**Features:**

- `--item` flag can also be short `-i`

**Misc:**

- all features enabled by default when building
- refactoring cl args checks
- args usage error messages

<br>

## 1.6.0

### What's new:

**Features:**

- Get Pokémon items with `--item [NAME]`
    - can be conditionally compiled with `--features items`
    - increases binary size by ~600 KB
    - see documentation for more usage info and availabe items

**Misc:**

- prebuilt binaries will include all features from now on
- documentation
- some refactoring

<br>

## 1.5.0

### What's new:

**Features:**

- optional feature to include **retro gen7 sprites**
    - to compile with feature enabled:<br>
    `cargo build --release --locked --features gen7`
    - then to use gen7 instead of gen8 sprites use e.g. `pokeget random --gen7`
    - increases binary size by ~1 MB

**Misc:**

- corrected spelling of "Pokémon" in output
- CLI output more concise
- some refactoring
- rust_embed 6.8.1 -> 8.3.0