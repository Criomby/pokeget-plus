use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pokémon name or Pokédex ID (or "random" | 0)
    pub pokemon: Vec<String>,

    /// Hide the Pokémon's name
    #[arg(long, default_value_t = false)]
    pub hide_name: bool,

    /// Form of the Pokémon
    #[arg(short, long, default_value = "")]
    pub form: String,

    /// Display the Pokémon as it's mega form
    #[arg(short, long, default_value_t = false)]
    pub mega: bool,

    /// Display the Pokémon as it's mega X form
    #[arg(long, default_value_t = false)]
    pub mega_x: bool,

    /// Display the Pokémon as it's mega Y form
    #[arg(long, default_value_t = false)]
    pub mega_y: bool,

    /// Display the Pokémon as shiny
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,

    /// Display the alolan variant of the Pokémon
    #[arg(short, long, default_value_t = false)]
    pub alolan: bool,

    /// Display the gigantamax variant of the Pokémon
    #[arg(short, long, default_value_t = false)]
    pub gmax: bool,

    /// Display the hisui variant of the Pokémon
    #[arg(long, default_value_t = false)]
    pub hisui: bool,

    /// Display the noble variant of the Pokémon, this option often times only works in tandom with --hisui.
    #[arg(short, long, default_value_t = false)]
    pub noble: bool,

    /// Display the galarian variant of the Pokémon
    #[arg(long, default_value_t = false)]
    pub galar: bool,

    /// Display the female variant of the Pokémon if it exists. Doesn't apply to Nidoran for some reason.
    #[arg(long, default_value_t = false)]
    pub female: bool,

    #[cfg(feature = "gen7")]
    /// Display older gen7 sprite if available
    #[arg(long, default_value_t = false)]
    pub gen7: bool,
}
