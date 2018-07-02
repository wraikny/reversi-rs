extern crate reversi;
use reversi::game::*;
use reversi::cpu;
// extern crate rayon;
// use rayon::prelude::*;

fn main() {
    let setting = Setting {
        black : PlayerType::Computer(1),
        white : PlayerType::Computer(5),
        boardsize : (8, 8),
    };

    let cpu_setting = cpu::Setting::new(5, 100, 20, 2, 10, 10);
    
    for _ in 0..10 {
        start(&setting, &cpu_setting, false);
    }
}