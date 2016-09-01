#![feature(custom_attribute)]
#![feature(custom_derive, plugin)]

#![plugin(serde_macros)]

#![allow(unused_attributes)]

extern crate hyper;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate try_from;

#[macro_use]
mod macros;

mod card;
mod hearts_client;
mod game_status;
mod deal;
mod strategy;
mod error;

use hearts_client::HeartsClient;
use hearts_client::Password;
use game_status::PlayerName;
use strategy::DefensiveCardStrategy;

use clap::App;

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();

    let cli_options = App::new("hearts")
        .version("0.0.1")
        .author("Derek Williams <derek@nebvin.ca>")
        .about("Plays hearts for RBS Code Comp Nov 2015")
        .args_from_usage(
            "-u --user=<USER> 'Sets the player name'
             -p --password=<PASSWORD> 'Sets the player password'
             -s --server=<SERVER> 'Sets the hearts server'
             -r --repeat 'After game ends, attempt to join again'")
        .get_matches();

    let player_name = PlayerName::new(cli_options.value_of("USER").unwrap());
    let password = Password::new(cli_options.value_of("PASSWORD").unwrap());
    let server = cli_options.value_of("SERVER").unwrap();
    let repeat = cli_options.is_present("repeat");

    info!("Start Game");

    let strategy = DefensiveCardStrategy::new(player_name);

    let client = HeartsClient::new(password, server, strategy, repeat);
    client.play();
}
