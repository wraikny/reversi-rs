extern crate reversi;
use reversi::game::*;
use reversi::cpu;

fn main() {
    let setting = Setting {
        black : PlayerType::Human,
        white : PlayerType::Computer(4),
        boardsize : (8, 8),
    };

    let cpu_setting = cpu::Setting::new(5, 50, 20, 1, 7, 50);
    
    start(&setting, &cpu_setting, true);
}