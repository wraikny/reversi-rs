use std;

use reversi::color::Color;
use reversi::board::Board;

fn read_coodinate(player : &Color, board : &Board) -> Option<(usize, usize)> {
    let mut coodinate : Option<(usize, usize)> = None;
    
    while coodinate.is_none() {
        println!("Input coodinate of {} as 'w h'.(q: quit)", player);
        let mut read = String::new();
        std::io::stdin().read_line(&mut read)
            .expect("Failed to read line.");
        
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
            let (width, height) = board.size;
            if w < width && h < height {
                coodinate = Some((w, h));
            } else {
                println!("-*- ({}, {}) is out of range. -*-", w, h)
            }
        } else {
            println!("-*- Input correctly. -*-")
        }
    };

    coodinate
}

pub fn start() {
    println!("-*-Reversi!-*-");

    let mut board = Board::new();

    let mut player = Color::Black;

    let result = |color : Option<Color>| {
        if let Some(color) = color {
            println!("{} win!", color);
        } else {
            println!("Draw");
        }
    };

    'main_loop: loop {
        board.print();
        'input: loop {
            if board.putable(&player) {
                let coodinate = read_coodinate(&player, &board);

                if let Some(cdn) = coodinate {
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
            board.print();
            break 'main_loop;
        }
        player = player.rev();
    }
}