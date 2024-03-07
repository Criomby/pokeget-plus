# pokeget-plus

Display Pokémon, Pokéballs, berries, potions & more items in your terminal.<br>
For example you can get a random Pokémon each time you open a new terminal session.

![Pikachu](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/icons/pokemon/regular/pikachu.png)
![Sableye](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/icons/pokemon/regular/sableye.png)
![Arcanine](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/icons/pokemon/regular/arcanine.png)
![Metang](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/icons/pokemon/regular/metang.png)

![GreatBall](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/items/ball/great.png)
![BlukBerry](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/items/berry/bluk.png)
![Elixir](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/items/medicine/elixir.png)
![Honey](https://github.com/msikma/pokesprite/blob/c5aaa610ff2acdf7fd8e2dccd181bca8be9fcb3e/items/other-item/honey.png)

... and many more!

An advanced version of the original [pokeget-rs](https://github.com/talwat/pokeget-rs).

Key features:
- Display Pokémon or items in your terminal with many options
- Option to use the **retro gen7 sprites** with `--gen7`
- Get **Pokémon items** with `--item [NAME]`
  e.g. get a random Pokéball each time you open a new terminal session
- Prebuilt binaries for the most common targets
- \+ various changes and fixes (see [changelog](CHANGELOG.md))

<br>

## Installation

You have two options:

### 1. Download the latest binary from [releases](https://github.com/Criomby/pokeget-plus/releases)

### 2. Build from source

Install via cargo:

```sh
cargo install --git https://github.com/Criomby/pokeget-plus.git --locked
```
and making sure `$HOME/.cargo/bin` is added to `$PATH`.

*or* 

clone the repository and compile manually:

```sh
git clone --recurse-submodules https://github.com/Criomby/pokeget-plus.git
cd pokeget-plus
cargo build --release --locked
```

<br>

**Update** with either re-running `cargo install ...` as above or<br>
`git pull` on the repository and then recompile.

<br>

Tip:

> If you don't need the retro gen7 sprites or items, just add the `--no-default-features` flag and you'll get a smaller file size since those assets won't be embedded into the binary then.<br>
This will save you 1.6 MB (8.7 MB vs. 10.3 MB).

<br>

## Usage

### **Pokémon:**

`pokeget [NAME]`

e.g. `pokeget pikachu`<br>
Use `pokeget random` for a random Pokémon.

### **Item:** 

`pokeget --item [category/variation]`

e.g. `pokeget --item ball/fast` or `pokeget --item berry/pecha`

Use `pokeget --item random` for a random item or<br>
get a random item from a *category* with `pokeget --item ball/random` for a random Pokéball or `... berry/random` for a random berry, etc.

With the short item flag: `pokeget -i ball/random`, `pokeget -i berry/golden-razz`

-> List of all available items and categories: [items-list](data/items-list.txt) <-

<br>

### Advanced Usage

*For more info and to see all options, run* `pokeget --help`

You can also use multiple Pokémon / items like:

`pokeget bulbasaur pikachu` or `pokeget --item ball/ultra ball/random`

and Pokédex / item ID's work too, e.g.:

`pokeget 25` (Pikachu)

`pokeget --item 4` or `pokeget -i 4` (regular Pokéball)

> ID '0' is equal to 'random'.<br>
So you can also use `pokeget 0` and `pokeget --i 0` to get random results.



<br>

## Why?

I saw the demand for additional features like displaying a random Pokéball or berry in your terminal when you open a new session and since the original author did not want to include the proposed features, I decided to fork and code myself.<br>
Still, the original author did some great work creating the foundation of this project and I'm grateful for that.

If at any time the original author decides to want to include any added features in the original project, just open an issue and I'll be happy to draft a PR.

<br>

## Credits

The sprites are from [pokesprite](https://github.com/msikma/pokesprite).
They are embedded into the binary, so no additional files will have to be downloaded and `pokeget` runs fully offline.

Also see the [other projects](OTHER_PROJECTS.md).
