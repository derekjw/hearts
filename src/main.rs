#![allow(dead_code)]
#![allow(unused_variables)]

extern crate hyper;
#[macro_use]
extern crate log;
extern crate env_logger;

mod card;
mod player;
mod game_status;
mod deal;
mod card_strategy;

use player::Player;
use player::PlayerName;
use player::Password;
use card_strategy::SimpleCardStrategy;

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();

    let player_name = PlayerName::new("FlyingBirds");
    let password = Password::new("mypassword");

    info!("Start Game");

    // Settings.init();
    let player = Player::new(player_name, password, "localhost", SimpleCardStrategy);
    player.play();
}

struct Settings {
    username: String,
    password: String,
    hostname: String,
}
