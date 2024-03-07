use std::collections::HashMap;
use std::process::exit;
use image::{DynamicImage, GenericImage};
use crate::utils::random;
use crate::data::Data;

/// Fetches a sprite and returns a vector of bytes.
/// This will also format the names properly.
pub fn get_sprite(
    name: &mut String,
    shiny: bool,
    female: bool,
    form: &str,
    gen7: Option<bool>,
    is_item: Option<bool>,
    pokedex_list: &[&str],
    items_list: &[&str],
    items_index: &HashMap<String, String>
) -> Vec<u8> {
    // get corresponding Pokemon/item name if number entered
    if let Ok(id) = name.parse::<usize>() {
        if id == 0 {
            *name = String::from("random");
        } else {
            if is_item.is_some_and(|is_item| is_item == true) {
                *name = items_index.get(&id.to_string()).unwrap_or_else(|| {
                    eprintln!("Item with ID '{}' not found", id);
                    exit(1);
                }).to_string();
            } else {
                if id <= pokedex_list.len() {
                    *name = String::from(pokedex_list[id - 1]);
                } else {
                    eprintln!("Pokédex ID out of range");
                    exit(1);
                }
            }
        }
    }

    // filepath of sprite
    let path: String;

    // fn flows for item/pokemon
    if is_item.is_some_and(|is_item| is_item == true) {
        // name is item

        if name == "random" {
            *name = random(items_list);
        } else if name.ends_with("random") {
            let (category, _) = name
                .split_once('/')
                .unwrap_or_else(|| {
                    eprintln!("Invalid item name.\nExpected structure: 'category/variation'\nExample: 'ball/master'");
                    exit(1);
                });
            // get all item variations in category
            let mut variations: Vec<&str> = vec![];
            for item in items_list {
                if item.starts_with(category) {
                    let (_, variation) = item.split_once('/').unwrap();
                    variations.push(variation);
                } else {
                    // since list of items is ordered alphabetically
                    // stop searching when first non-category item comes up
                    // if items in category have been found
                    if !variations.is_empty() {
                        break;
                    }
                }
            }
            if variations.is_empty() {
                eprintln!("Item category '{}' not found", category);
                exit(1);
            }
            *name = format!(
                "{}/{}",
                category,
                random(&variations)
            );
        }

        path = format!("items/{}.png", name);

    } else {
        // name is Pokemon

        let is_random = name == "random";

        if is_random {
            *name = random(pokedex_list);
        }

        let mut filename = name.to_owned();

        // The form shouldn't be applied to random pokemon.
        if !form.is_empty() && !is_random {
            filename.push('-');
            filename.push_str(form);
        }
    
        // I hate Mr. Mime and Farfetch'd.
        filename = filename
            .replace([' ', '_'], "-")
            .replace(['.', '\'', ':'], "")
            .to_lowercase();
    
        let prefix = match gen7 {
            Some(true) => "pokemon-gen7x/",
            Some(false) => "pokemon-gen8/",
            None => "pokemon-gen8/"
        };
    
        path = format!(
            "{}{}/{}{}.png",
            prefix,
            if shiny { "shiny" } else { "regular" },
            if female && !is_random { "female/" } else { "" }, // Random pokemon also shouldn't follow the female rule.
            filename.trim()
        );
    }
    
    Data::get(&path)
        .unwrap_or_else(|| {
            if is_item.is_some_and(|is_item| is_item == true) {
                eprintln!("Item '{name}' not found.\nExample: 'ball/master'\nTry 'ball/random' for a random Pokéball");
            } else {
                eprintln!("Pokémon '{name}' not found");
            }
            exit(1);
        })
        .data
        .into_owned()
}

/// Combines several sprites into one by stitching them horizontally.
pub fn combine_sprites(
    combined_width: u32,
    combined_height: u32,
    sprites: &[DynamicImage],
) -> DynamicImage {
    let mut combined = DynamicImage::new_rgba8(combined_width - 1, combined_height);
    let mut shift = 0;

    for sprite in sprites {
        combined
            .copy_from(sprite, shift, combined_height - sprite.height())
            .unwrap();
        shift += sprite.width() + 1;
    }

    combined
}

/// Loops through all the pokemon specified in the args and returns a vector of images.
/// This will also format the names properly.
///
/// Mutable access to `pokemons` is required to edit the names of random pokemon so they can be displayed.
pub fn get_sprites(
    names: &mut [String],
    shiny: bool,
    female: bool,
    form: &str,
    gen7: Option<bool>,
    is_item: Option<bool>,
    pokedex_list: &[&str],
    items_list: &[&str],
    items_index: &HashMap<String, String>
) -> (u32, u32, Vec<DynamicImage>) {
    let mut sprites = Vec::new();
    let mut combined_width: u32 = 0;
    let mut combined_height: u32 = 0;

    for name in names.iter_mut() {
        let bytes = get_sprite(
            name,
            shiny,
            female,
            form,
            gen7,
            is_item,
            pokedex_list,
            items_list,
            items_index,
        );

        let img = image::load_from_memory(&bytes).unwrap();
        let trimmed = showie::trim(&img);

        combined_width += trimmed.width() + 1;

        if trimmed.height() > combined_height {
            combined_height = trimmed.height();
        }

        sprites.push(trimmed);
    }

    (combined_width, combined_height, sprites)
}
