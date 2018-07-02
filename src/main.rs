extern crate reversi;
use reversi::game::*;

fn main() {
    let setting = Setting {
        black : PlayerType::Human,
        white : PlayerType::Computer
    };

    if let Err(e) = start(setting) {
        println!("{}", e);
    }
}