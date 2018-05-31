use std;

use color::Color;
use board::Board;

const WIDTH : usize = 8;
const HEIGHT : usize = 8;

fn get_board_size(filename : &str) -> (usize, usize) {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(why) => {
            println!("Couldn't open {}: {}", filename, Error::description(&why));
            return (WIDTH, HEIGHT);
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("Couldn't read {}: {}", filename, Error::description(&why));
            return (WIDTH, HEIGHT);
        },
        Ok(_) => (),
    }

    let mut s = s.split(",")
        .map(|x| {
            match x.trim().parse::<usize>() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        });

    if let (Some(Some(w)), Some(Some(h))) = (s.next(), s.next()) {
        (w, h)
    } else {
        (WIDTH, HEIGHT)
    }
}

fn read_coordinate(player : &Color, size : (usize, usize)) -> Option<(usize, usize)> {
    let (width, height) = size;
    let mut coordinate : Option<(usize, usize)> = None;
    
    while coordinate.is_none() {
        println!("Input coordinate of {} as 'w h'. (q: quit)", player);
        let mut read = String::new();
        std::io::stdin().read_line(&mut read)
            .expect("Failed to read line.");
        
        print!("\n");
        
        // Quit the game.
        if read.trim() == "q".to_string() {
            return None;
        }

        let mut c = read.split_whitespace().map(|x| {
                match x.trim().parse::<usize>() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                }
            });
        
        if let (Some(Some(w)), Some(Some(h))) = (c.next(), c.next()) {
            if w < width && h < height {
                coordinate = Some((w, h));
            } else {
                println!("-*- ({}, {}) is out of range. -*-\n", w, h)
            }
        } else {
            println!("-*- Input correctly. -*-\n")
        }
    };
    coordinate
}

pub fn start() {
    println!("Reversi!!!!!\n");

    let size = get_board_size("config.csv");

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

                if let Some(cdn) = coordinate {
                    if board.put(cdn, &player) {
                        break 'input;
                    }
                } else {
                    break 'main_loop;
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
}