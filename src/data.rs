use rust_embed::RustEmbed;

// include datasets based on feature selection


#[cfg(all(not(feature = "gen7"), not(feature = "items")))]
#[derive(RustEmbed)]
#[folder = "data/pokesprite/"]
#[include = "pokemon-gen8/*"]
pub struct Data;

#[cfg(all(feature = "gen7", not(feature = "items")))]
#[derive(RustEmbed)]
#[folder = "data/pokesprite/"]
#[include = "pokemon-*"]
pub struct Data;

#[cfg(all(not(feature = "gen7"), feature = "items"))]
#[derive(RustEmbed)]
#[folder = "data/pokesprite/"]
#[include = "pokemon-gen8/*"]
#[include = "items/*"]
pub struct Data;

#[cfg(all(feature = "gen7", feature = "items"))]
#[derive(RustEmbed)]
#[folder = "data/pokesprite/"]
#[include = "pokemon-*"]
#[include = "items/*"]
pub struct Data;