extern crate reversi;
use reversi::game::*;
use reversi::cpu;
// extern crate rayon;
// use rayon::prelude::*;

fn main() {
    let setting = Setting {
        black : PlayerType::Computer(3),
        white : PlayerType::Computer(3),
        boardsize : (8, 8),
    };

    let cpu_setting = cpu::Setting::new(5, 50, 20, 2, 10, 50);
    
    start(&setting, &cpu_setting, true);
}