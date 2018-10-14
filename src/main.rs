extern crate clap;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;


#[derive(Deserialize)]
struct DirectionResult {
    status: String,
    routes: Vec<DirectionRoute>,
}

#[derive(Deserialize)]
struct DirectionRoute { legs: Vec<DirectionLeg>, }

#[derive(Deserialize)]
struct DirectionLeg {
    distance: TextValue,
    duration: TextValue,
}

#[derive(Deserialize)]
struct TextValue { text: String, }

#[derive(Deserialize)]
struct ApiKeys { directions : String, }


/*
Todo features:
done for now: time walking|driving|transit "place" "place"
next: latlng "place"
definitely add: find "pizza", "italian food", etc.
definitely add: aliases set "wilk" "Erneston Wilkson center"
    - find out how to save file data
definitely add: aliases rm "wilk"
definitely add: aliases view
*/


fn main() {

    let argparse = App::new("GMAP")
        .version("0.1.0")
        .author("Aaron Chan <aaron.y.chan64@gmail.com>")
        .about("Google Maps CLI w additional functionality written in Rust")
        .subcommand(SubCommand::with_name("time")
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
        .subcommand(SubCommand::with_name("coordinates")
            .about("return the latitude and longitude of an address")
            .arg(Arg::with_name("place")
                .required(true)
                .takes_value(true)
                .help("address to retrieve latitude and longitude")))
        .get_matches();

    let keys_json : String = read_file("keys/api-keys.json");
    let api_keys: ApiKeys = serde_json::from_str(&keys_json).unwrap();
     
    if let Some(argparse) = argparse.subcommand_matches("time") {
        let travel_method = argparse.value_of("travel-method").unwrap();
        let origin = argparse.value_of("origin").unwrap();
        let destination = argparse.value_of("destination").unwrap();
        println!("{}, {}, {}", travel_method, origin, destination);

        let mut url_params : String = "https://maps.googleapis.com/maps/api/directions/json?".to_owned();
        url_params = url_params + "origin=" + origin + "&";
        url_params = url_params + "destination=" + destination + "&";
        match travel_method {
            "walking" => url_params += "mode=walking&",
            "driving" => url_params += "mode=driving&",
            "transit" => url_params += "mode=transit&",
            _ => {
                println!("Error: Unrecognized travel method (try 'walking', 'driving', 'bicycling', 'transit')");
                ::std::process::exit(1);
            }
        }
        url_params = url_params + "key=" + &api_keys.directions;

        println!("{}", url_params);

        let res = reqwest::Client::new().get(&url_params).send();
        let thing:DirectionResult = res.unwrap().json().unwrap();

        // TODO check status
        println!("---- QUERY ----");
        println!("GET {}", url_params);
        println!("");
        println!("---- RESULTS ----");
        println!("Status: {}", thing.status);
        println!("Distance: {}", thing.routes[0].legs[0].distance.text);
        println!("Time to Travel: {}", thing.routes[0].legs[0].duration.text);
        ::std::process::exit(1);
    }
    else if let Some(argparse) = argparse.subcommand_matches("coordinates") {
        let place = argparse.value_of("place").unwrap();
        println!("{}", place);

        // url https://maps.googleapis.com/maps/api/geocode/json?address=1600+Amphitheatre+Parkway,+Mountain+View,+CA&key=YOUR_API_KEY
        // response: see notes
    }
}

// Taken from https://github.com/serde-rs/serde/issues/1195
fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}
