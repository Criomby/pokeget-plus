//! Display pokemon sprites in your terminal.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use clap::Parser;
use pokeget_plus::cli::Args;
use pokeget_plus::sprites::{
    combine_sprites,
    get_sprites
};
use pokeget_plus::utils::{
    check_args,
    format_name
};
use std::collections::HashMap;
use serde_json;

fn main() {
    let pokedex_list: Box<[&'static str]> = include_str!("../data/pokedex-list.txt").split('\n').collect();
    let items_list: Box<[&'static str]> = include_str!("../data/items-list.txt").split('\n').collect();
    let items_index: HashMap<String, String> = serde_json::from_str(include_str!("../data/items.json")).expect("Invalid items.json");

    let args = Args::parse();

    check_args(&args);

    let mut names = args.names.clone();

    let (width, height, sprites) =
        get_sprites(
            &mut names,
            &args,
            #[cfg(not(feature = "gen7"))]
            None,
            #[cfg(feature = "gen7")]
            Some(args.gen7),
            #[cfg(not(feature = "items"))]
            None,
            #[cfg(feature = "items")]
            Some(args.is_item),
            &pokedex_list,
            &items_list,
            &items_index
        );
    
    let combined = combine_sprites(width, height, &sprites);

    if !args.hide_name {
        eprintln!(
            "{}\n",
            names
                .iter()
                .enumerate()
                .map(|(i, x)| format_name(x) + if i != names.len() - 1 { ", " } else { "" })
                .collect::<String>()
        );
    }

    println!("{}", showie::to_ascii(&combined));
}
