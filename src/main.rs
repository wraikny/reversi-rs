extern crate reversi;
use reversi::game::*;

fn main() {
    let setting = Setting {
        black : PlayerType::Human,
        white : PlayerType::Computer(5),
        boardsize : (8, 8),
    };

    if let Err(e) = start(setting) {
        println!("{}", e);
    }
}