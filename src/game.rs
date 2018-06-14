use std;

use color::Color;
use board::Board;

enum Input {
    Coordinate((usize, usize)),
    Quit,
}

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

fn read_coordinate(player : &Color, size : (usize, usize)) -> Input {
    let (width, height) = size;
    
    loop {
        println!("Input coordinate of {} as 'w h'. (q: quit)", player);
        let mut read = String::new();
        std::io::stdin().read_line(&mut read)
            .expect("Failed to read line.");
        
        print!("\n");
        
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
                println!("-*- ({}, {}) is out of range. -*-\n", w, h)
            }
        } else {
            println!("-*- Input correctly. -*-\n")
        }
    }
}

pub fn start() -> Result<(), String> {
    let size = get_board_size("config.csv")?;

    println!("Reversi!!!!!\n");
    println!("The Board size is {:?}.\n", size);

    let mut board = Board::new(size);
    let mut player = Color::Black;

    let result = |color : Option<Color>| {
        if let Some(color) = color {
            println!("{} win!", color);
        } else {
            println!("Draw");
        }
    };

    'main_loop: loop {
        board.display();

        'input: loop {
            if board.putable(&player) {
                let coordinate = read_coordinate(&player, size);

                match coordinate {
                    Input::Coordinate(cdn) => if board.put(cdn, &player) {
                        break 'input;
                    },
                    Input::Quit => break 'main_loop,
                }
            } else {
                println!("-*-Skiped the player {}-*-\n", &player);
                player = player.rev();
                continue 'main_loop;
            }
        }

        if board.finished(&player) {
            result(board.winner());
            board.display();
            break 'main_loop;
        }

        player = player.rev();
    }
    Ok(())
}