use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// NAME(S) or "random"
    pub names: Vec<String>,

    #[cfg(feature = "items")]
    /// NAME(S) is item (e.g. 'ball/master')
    #[arg(short, long = "item", default_value_t = false)]
    pub is_item: bool,

    #[cfg(feature = "gen7")]
    /// Display retro gen7 sprite
    #[arg(long, default_value_t = false)]
    pub gen7: bool,

    /// Hide the Pokémon / item name
    #[arg(long, default_value_t = false)]
    pub hide_name: bool,

    /// Display shiny Pokémon
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,

    /// Display the female variant of the Pokémon. Doesn't apply to Nidoran for some reason.
    #[arg(short, long, default_value_t = false)]
    pub female: bool,

    /// Form of the Pokémon
    #[arg(long, default_value = "regular")]
    pub form: String,
}
