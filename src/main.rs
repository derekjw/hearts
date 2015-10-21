#![allow(dead_code)]

mod card;
mod player;
mod game_status;
mod deal;

use player::Player;
use player::PlayerName;
use player::Password;

#[allow(dead_code)]
fn main() {
    let player_name = PlayerName::new("FlyingBirds");
    let password = Password::new("mypassword");

    println!("Start Game");

    // Settings.init();
    let player = Player::new(player_name, password, "localhost");
    player.play();
}

struct Settings {
    username: String,
    password: String,
    hostname: String,
}
