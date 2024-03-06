# pokeget+

An advanced version of the original [pokeget-rs](https://github.com/talwat/pokeget-rs).

Additional features over the original:
- Option to use the smaller/retro gen7 sprites with `--gen7`
- \+ various fixes and changes (see [changelog](CHANGELOG.md))
- Coming soon:
    - Pokémon items with `--item [NAME]`
    - List available Pokémon & items with `--list`, `-l`

<br>

## Installation

You have two options:

### 1. Download the latest binary from [releases](https://github.com/Criomby/pokeget-rs/releases)

- The **min** version contains *only the gen8 sprites* for reduced file size (~8.6 MB)
- The **plus** version contains the full feature set including the *gen7 sprites* and *items* (~X MB)

### 2. Build from source

Install via cargo:

```sh
cargo install --git https://github.com/Criomby/pokeget-rs --locked --all-features
```
and making sure `$HOME/.cargo/bin` is added to `$PATH`.

*or* 

clone the repository and compile manually:

```sh
git clone --recurse-submodules https://github.com/Criomby/pokeget-rs
cd pokeget-rs
cargo build --release --locked --all-features
```

If you wish to only have the gen8 sprites to get a smaller binary<br>
and you don't need gen7 sprites or items just remove the `--all-features` flag.

**Update** with either re-running `cargo install --git https://github.com/Criomby/pokeget-rs` or<br>
`git pull` on the repository and then recompile.

<br>

## Usage

`pokeget [POKEMON]` e.g. `pokeget pikachu`

for more info and to see all options, run `pokeget --help`

### Advanced Usage

TODO

<br>

You can also use multiple pokemon like:

`pokeget bulbasaur pikachu`

and Pokédex ID's work too:

`pokeget 1 2 3`

<br>

## Why?

I saw the demand for additional features and since the original author did not want to include the proposed features, I decided to fork and code them myself.
Still, the original author did some great work creating the foundation of this project.

If at any time the original author decides to want to include any added features in the original project, just open an issue and I'll be happy to draft a PR.

<br>

## Credits

The sprites are from [pokesprite](https://github.com/msikma/pokesprite).
They are embedded into the binary, so no additional files will have to be downloaded and `pokeget` runs fully offline.

Also see the [other projects](OTHER_PROJECTS.md).
