extern crate clap;
extern crate reqwest;

use clap::{Arg, App, SubCommand};
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Ip {
    origin: String,
}

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
        let travel_method = argparse.value_of("travel-method").unwrap();
        let origin = argparse.value_of("origin").unwrap();
        let destination = argparse.value_of("destination").unwrap();
        println!("{}, {}, {}", travel_method, origin, destination);

        if travel_method != "walk" && travel_method != "drive" && travel_method != "public" {
            println!("Unrecognized travel method (try 'walk', 'drive', 'public')");
        }

        // DO HTTP Request
        // Solve the key problem
        

        let res = reqwest::Client::new()
                        .get("http://httpbin.org/ip").send();
                        //.get("https://byu-courses.herokuapp.com/courses/codes").send();
        let thing:Ip = res.unwrap().json().unwrap();
        println!("{}", thing.origin);
    }
}

/*
timeto walk|drive|bus "place" "place"
latlng "place"
find "pizza", "italian food", etc.
aliases set "wilk" "Erneston Wilkson center"
aliases rm "wilk"
aliases view
*/
