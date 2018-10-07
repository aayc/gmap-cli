extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let argparse = App::new("GMAP")
        .version("0.1.0")
        .author("Aaron Chan <aaron.y.chan64@gmail.com>")
        .about("Google Maps CLI w additional functionality written in Rust")
        .subcommand(SubCommand::with_name("timeto")
            .about("return how many minutes to travel from place to place")
            .arg(Arg::with_name("travel-method")
                .required(true)
                .takes_value(true)
                .help("walk|drive|public \"START\" \"DESTINATION\""))
            .arg(Arg::with_name("origin")
                .required(true)
                .takes_value(true)
                .help("place where you start (alias or \"ADDRESS\")"))
            .arg(Arg::with_name("destination")
                .required(true)
                .takes_value(true)
                .help("place where you end (alias or \"ADDRESS\")")))
        .get_matches();
    
    if let Some(argparse) = argparse.subcommand_matches("timeto") {
        let travelMethod = argparse.value_of("travel-method").unwrap();
        let origin = argparse.value_of("origin").unwrap();
        let destination = argparse.value_of("destination").unwrap();
        println!("{}, {}, {}", travelMethod, origin, destination);

        if travelMethod != "walk" && travelMethod != "drive" && travelMethod != "public" {
            println!("Unrecognized travel method (try 'walk', 'drive', 'public')");
        }

        // DO HTTP Request
    }
    //println!("{}", action);
}

/*
timeto walk|drive|bus "place" "place"
latlng "place"
find "pizza", "italian food", etc.
aliases set "wilk" "Erneston Wilkson center"
aliases rm "wilk"
aliases view
*/
