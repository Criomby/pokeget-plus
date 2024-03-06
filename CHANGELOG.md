## 1.5.0

### What's new:

**Features:**

- optional feature to include **gen7 sprites**
    - feature not included in the prebuild binaries
    - to compile with feature enabled:<br>
    `cargo build --release --locked --features gen7`
    - then to use gen7 instead of gen8 sprites use e.g. `pokeget random --gen7`
    - increases binary size by ~1 MB

**Misc:**

- corrected spelling of "Pokémon" in output
- CLI output more concise
- some refactoring
- rust_embed 6.8.1 -> 8.3.0