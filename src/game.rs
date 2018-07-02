use std;

use color::Color;
use board::Board;
use cpu;

enum Input {
    Coordinate((usize, usize)),
    Quit,
}

fn read_coordinate(size : (usize, usize)) -> Input {
    let (width, height) = size;
    
    loop {
        let mut read = String::new();
        std::io::stdin().read_line(&mut read)
            .expect("Failed to read line.");
        
        // Quit the game.
        if read.trim() == "q".to_string() {
            return Input::Quit;
        }

        let c : Vec<_> = read.split_whitespace()
            .map(|x| x.trim().parse::<usize>()).collect();
        
        if let (Some(Ok(w)), Some(Ok(h))) = (c.get(0), c.get(1)) {
            if w < &width && h < &height {
                return Input::Coordinate((*w, *h));
            } else {
                println!("-*- ({}, {}) is out of range. -*-", w, h)
            }
        } else {
            println!("-*- Input correctly. -*-")
        }
    }
}

fn result_game(color : Option<Color>) {
    if let Some(color) = color {
        println!("{} win!", color);
    } else {
        println!("Draw");
    }
}

pub enum PlayerType {
    Human,
    Computer(cpu::Setting),
}

pub struct Setting {
    pub black : PlayerType,
    pub white : PlayerType,
    pub boardsize : (usize, usize),
}

impl Setting {
    fn player_type(&self, c : &Color) -> &PlayerType {
        match *c {
            Color::Black => &self.black,
            Color::White => &self.white,
        }
    }
}

pub fn start(setting : &Setting, display : bool) {

    let size = setting.boardsize;
    if display {
        println!("Reversi!");
        println!("The Board size is {:?}.\n", size);
    }

    let is_cpu = |c : &Color| match setting.player_type(c) {
        PlayerType::Human => false,
        PlayerType::Computer(_) => true
    };

    let mut board = Board::new(size);

    let mut player = Color::Black;

    let mut count = 0;

    'main_loop: loop {
        if display {
            board.display();
        } else {
            print!("{}, ", count);
            count = count + 1;
        }

        if board.finished(&player) {
            if !display {
                println!();
            }
            let winner = board.winner();
            result_game(winner);
            break 'main_loop;
        }

        if board.putable(&player) {
            if !is_cpu(&player) {
                println!("Input coordinate of {} as 'w h'. (q: quit)", &player);
            }
            'input: loop {
                let coordinate = match setting.player_type(&player) {
                    PlayerType::Human => read_coordinate(size),
                    PlayerType::Computer(cs) => Input::Coordinate(cpu::select(&player, &board, &cs).unwrap()),
                };

                match coordinate {
                    Input::Coordinate(cdn) => if board.putable_cdns(&player).contains(&cdn) {
                        board.put(cdn, &player);
                        break 'input;
                    } else {
                        println!("-*-Couldn't put there.-*-");
                    },
                    Input::Quit => break 'main_loop,
                }
            }
        } else {
            if display {
                println!("-*-Skiped the player {}-*-", &player);
            }
        }

        player = player.rev();
    }
}