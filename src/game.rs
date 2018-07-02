use std;

use color::Color;
use board::Board;
use cpu;

fn get_board_size(filename : &str) -> Result<(usize, usize), String> {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(why) => {
            return Err(format!("Couldn't open {}: {}", filename, Error::description(&why)));
        },
        Ok(file) => file,
    };
    

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            return Err(format!("Couldn't read {}: {}", filename, Error::description(&why)));
        },
        Ok(_) => (),
    }

    let mut s = s.split(",")
        .map(|x| x.trim().parse::<usize>());

    if let (Some(Ok(w)), Some(Ok(h))) = (s.next(), s.next()) {
        Ok((w, h))
    } else {
        Err("The format of the file of the board size is not correct.\nExample: 8, 8".to_owned())
    }
}

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
    Computer,
}

pub struct Setting {
    pub black : PlayerType,
    pub white : PlayerType,
}

pub fn start(setting : Setting) -> Result<(), String> {
    let size = get_board_size("config.csv")?;

    println!("Reversi!");
    println!("The Board size is {:?}.\n", size);

    let is_cpu = |t : &PlayerType| match *t {
        PlayerType::Human => false,
        PlayerType::Computer => true
    };

    let color_is_cpu = |color : &Color| {
        use color::Color::*;
        match *color {
            Black => is_cpu(&setting.black),
            White => is_cpu(&setting.white),
        }
    };

    let mut board = Board::new(size);

    let mut player = Color::Black;

    'main_loop: loop {
        board.display();

        if board.finished(&player) {
            result_game(board.winner());
            break 'main_loop;
        }

        if board.putable(&player) {
            if !color_is_cpu(&player) {
                println!("Input coordinate of {} as 'w h'. (q: quit)", &player);
            }
            'input: loop {
                let coordinate = if !color_is_cpu(&player) {
                    read_coordinate(size)
                } else {
                    Input::Coordinate(cpu::select(&player, &board).unwrap())
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
            println!("-*-Skiped the player {}-*-", &player);
        }

        player = player.rev();
    }

    Ok(())
}