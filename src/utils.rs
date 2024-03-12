use std::process::exit;
use rand::Rng;
use inflector::Inflector;
use crate::cli::Args;

/// Returns a random pokemon.
pub fn random(list: &[&str]) -> String {
    let mut rand = rand::thread_rng();
    String::from(list[rand.gen_range(0..list.len())])
}

/// Check cl args compliance
pub fn check_args(args: &Args) {
    if args.names.is_empty() {
        print_err("Please specify the Pokémon or item to display");
        exit(1);
    }

    #[cfg(feature = "items")]
    {
        // check options
        let mut options: Vec<String> = Vec::new();

        if !(args.form == "regular") {
            options.push("--form <FORM>".to_string());
        }    
        if args.female {
            options.push("--female".to_string());
        }
        if args.shiny {
            options.push("--shiny ".to_string());
        }
        #[cfg(feature = "gen7")]
        if args.gen7 {
            options.push("--gen7".to_string());
        }

        if args.is_item {
            if options.len() > 0 {
                print_err(&format!("Items do not have any options.\nRemove: {:?}", options));
                exit(1);
            }
        }
    }
}

pub fn format_name(name: &String) -> String {
    if name.contains('/') {
        // format item names
        let (cat, var) = name.split_once('/').unwrap();
        format!("{} {}", var, cat).to_title_case()
    } else {
        name.to_title_case().replace('-', " ")
    }
}

// prints a formatted error message
pub fn print_err(text: &str) {
    eprintln!("\x1b[31m{text}\x1b[0m\n'pokeget --help' for more info.")
}