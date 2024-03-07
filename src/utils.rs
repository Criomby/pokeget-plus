use std::process::exit;

use rand::Rng;

use crate::cli::Args;

/// Returns a random pokemon.
pub fn random(list: &[&str]) -> String {
    let mut rand = rand::thread_rng();
    String::from(list[rand.gen_range(0..list.len())])
}

/// Uses the arguments like gmax, mega, etc... to get a form which is appended to the pokemon filename.
pub fn get_form(args: &Args) -> String {
    let mut form;

    if args.mega {
        form = String::from("mega");
    } else if args.mega_x {
        form = String::from("mega-x");
    } else if args.mega_y {
        form = String::from("mega-y");
    } else if args.alolan {
        form = String::from("alola");
    } else if args.gmax {
        form = String::from("gmax");
    } else if args.hisui {
        form = String::from("hisui");
    } else if args.galar {
        form = String::from("galar");
    } else {
        form = args.form.clone();
    }

    if args.noble {
        form.push_str("-noble");
    }

    form
}

/// check cl args compliance
pub fn check_args(args: &Args) {
    if args.names.is_empty() {
        eprintln!("Please specify the Pokémon or item to display.\n'pokeget --help' for more info.");
        exit(1);
    }
    #[cfg(feature = "items")]
    {
        if args.is_item {
            let mut options = String::new();

            if !args.form.is_empty() {
                options.push_str("--form <FORM> ");
            } else if args.mega {
                options.push_str("--mega ");
            } else if args.mega_x {
                options.push_str("--mega-x ");
            } else if args.mega_y {
                options.push_str("--mega-y ");
            } else if args.shiny {
                options.push_str("--shiny ");
            } else if args.alolan {
                options.push_str("--alolan ");
            } else if args.gmax {
                options.push_str("--gmax ");
            } else if args.hisui {
                options.push_str("--hisui");
            } else if args.noble {
                options.push_str("--noble");
            } else if args.galar {
                options.push_str("--galar");
            } else if args.female {
                options.push_str("--female");
            }

            if !options.is_empty() {
                eprintln!("Items do not have any options.\nRemove: {}", options);
                exit(1);
            }
        }
    }
}