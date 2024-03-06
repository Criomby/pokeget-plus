use rust_embed::RustEmbed;

// include datasets based on feature selection

#[cfg(not(feature = "gen7"))]
#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon-gen8"]
pub struct Data;

#[cfg(feature = "gen7")]
#[derive(RustEmbed)]
#[folder = "data/pokesprite/"]
#[include = "pokemon-*"]
pub struct Data;