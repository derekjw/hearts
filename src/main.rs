#![allow(dead_code)]
#![allow(unused_variables)]

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
    let player_name = PlayerName::new("FlyingBirds");
    let password = Password::new("mypassword");

    println!("Start Game");

    // Settings.init();
    let player = Player::new(player_name, password, "localhost", SimpleCardStrategy);
    player.play();
}

struct Settings {
    username: String,
    password: String,
    hostname: String,
}
