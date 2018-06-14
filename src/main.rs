extern crate reversi;

fn main() {
    if let Err(e) = reversi::game::start() {
        println!("{}", e);
    }
}