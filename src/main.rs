#![allow(unused_variables)]

#![feature(dir_builder)]
#![feature(iter_arith)]
#![feature(custom_attribute)]
#![feature(custom_derive, plugin)]

#![plugin(serde_macros)]

extern crate hyper;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate clap;

#[macro_use]
mod macros;

mod card;
mod player;
mod game_status;
mod deal;
mod strategy;
mod try_from;
mod error;

use player::Player;
use player::PlayerName;
use player::Password;
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

    // Settings.init();
    let player = Player::new(player_name, password, server, DefensiveCardStrategy, repeat);
    player.play();
}
