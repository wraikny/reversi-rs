extern crate reversi;
use reversi::game::*;
use reversi::cpu;

fn main() {
    let cpu_setting = cpu::Setting::new(5, 5, 50, 20, 1, 7, 50);

    let setting = Setting {
        black : PlayerType::Human,
        white : PlayerType::Computer(cpu_setting),
        boardsize : (8, 8),
    };
    
    start(&setting, true);
}