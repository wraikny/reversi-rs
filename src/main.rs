use std::collections::{HashMap, HashSet};

const WIDTH : usize = 8;
const HEIGHT : usize = 8;

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Color {
    fn rev(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        if let Color::White = self {
            if let Color::White = other {
                true
            } else {false}
        } else {
            if let Color::Black = other {
                true
            } else {false}
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::Black => "Black",
            Color::White => "White",
        })
    }
}

#[derive(Clone)]
struct Board {
    colors: HashMap<(usize, usize), Option<Color>>,
}

impl Board {
    fn new() -> Board {
        let mut board = Board{colors: HashMap::new()};

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                board.colors.insert((x, y), None);
            }
        }
        
        {
            let mut insert = |cdn, color|{
                board.colors.insert(cdn, Some(color));
            };

            let (x, y) = (WIDTH / 2, HEIGHT / 2);
            
            use Color::{White, Black};

            insert((x - 1, y - 1), White);
            insert((x, y), White);
            insert((x - 1, y), Black);
            insert((x, y - 1), Black);
        }

        board
    }

    fn print(&self) {
        let head = (0..WIDTH).fold(" ".to_string(), |s, x| format!("{} {}", s, x));

        let mut board : Vec<_> = self.colors.iter().collect();
        board.sort_by_key(|((w, _), _)| w);

        let table = (0..HEIGHT).fold("".to_string(), |s, h| {
            format!("{}{}|{}\n", s, h, {
                board.iter().filter(|((_, h_), _)| *h_ == h)
                    .fold("".to_string(), |s, ((_, _), color)|{
                        s + "" + match color {
                            Some(Color::Black) => "B",
                            Some(Color::White) => "W",
                            None => " ",
                        } + "|"
                    })
            })
        });

        println!("{}\n{}", head, table);
    }

    fn rev_coodinates(
        &self, coodinate : (usize, usize), 
        player : &Color
    ) -> HashSet<(usize, usize)> {
        let mut rev_cdns : HashSet<(usize, usize)> = HashSet::new();

        // if the coodinate in the keys and empty
        if self.colors.get(&coodinate).is_some() {
            let (w, h) = coodinate;

            let find_same = |board : &Vec<(&(usize, usize), &Option<Color>)>| {
                let found = board.iter().rev().find(|(_, color)| {
                    if let Some(color) = *color {
                        color == player
                    } else { false }
                });

                if let Some((cdn, _)) = found {
                    Some(**cdn)
                } else {
                    None
                }
            };

            let mut add_revs = |board : &Vec<&(&(usize, usize), &Option<Color>)>| {
                if board.clone().iter().all(|(_, color)|{
                        if let Some(color) = color {
                            color != player
                        } else { false }
                }) {
                    board.iter().for_each(|(cdn, _)| {
                        rev_cdns.insert(**cdn);
                    });
                }
            };

            let mut search = |
                f : &Fn((usize, usize)) -> bool, 
                g : &Fn((usize, usize)) -> usize,
                h : &Fn((usize, usize), (usize, usize)) -> bool,
            | {
                let mut board : Vec<_> = self.colors.iter()
                    .filter(|(item, _)| f(**item)).collect();
                
                board.sort_by_key(|(item, _)| g(**item));

                if let Some(cdnf) = find_same(&board) {
                    let mut board = board.iter()
                        .filter(|(item, _)|{
                            h(cdnf, **item)
                        }).collect();
                    add_revs(&board);
                }
            };

            // upside
            search(
                &|(x, y)| x == w && y < h,
                &|(_, y)| y,
                &|(_, hf), (_, y)| hf < y,
            );
            // downside
            search(
                &|(x, y)| x == w && y > h,
                &|(_, y)| HEIGHT - y,
                &|(_, hf), (_, y)| hf > y,
            );
            // leftside
            search(
                &|(x, y)| y == h && x < w,
                &|(x, _)| x,
                &|(wf, _), (x, _)| wf < x,
            );
            // rightside
            search(
                &|(x, y)| y == h && x > w,
                &|(x, _)| WIDTH - x,
                &|(wf, _), (x, _)| wf > x,
            );
            // leftup
            search(
                &|(x, y)| (w + y) == (x + h) && x < w,
                &|(x, _)| x,
                &|(wf, _), (x, _)| wf < x,
            );
            // rightdown
            search(
                &|(x, y)| (w + y) == (x + h) && x > w,
                &|(x, _)| WIDTH - x,
                &|(wf, _), (x, _)| wf > x,
            );
            // leftdown
            search(
                &|(x, y)| (w + h) == (x + y) && x < w,
                &|(x, _)| x,
                &|(wf, _), (x, _)| wf < x,
            );
            // rightup
            search(
                &|(x, y)| (w + h) == (x + y) && x > w,
                &|(x, _)| WIDTH - x,
                &|(wf, _), (x, _)| wf > x,
            );
        }

        rev_cdns
    }

    fn exist_nextto(&self, (w, h) : (usize, usize), player : &Color) -> bool {
        let v = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];
        v.iter().any(|k|{
            let (x, y) = k;
            let (w, h) = ((w as i32 - x) as usize, (h as i32 - y) as usize);
            if let Some(Some(color)) = self.colors.get(&(w, h)) {
                *color == player.rev()
            } else {false}
        })
    }

    fn putable(&self, player : &Color) -> bool {
        self.colors.iter()
        .any(|(cdn, color)|{
            color.is_none() &&
            self.exist_nextto(*cdn, player) && 
            self.rev_coodinates(*cdn, &player).iter().count() > 0
        })
    }

    fn put(&mut self, coodinate : (usize, usize), player : &Color) -> bool {
        let rev_coodinates = self.rev_coodinates(coodinate, player);
        if rev_coodinates.iter().count() > 0 {
            self.colors.insert(coodinate, Some(*player));
            for cdn in rev_coodinates {
                self.rev(cdn);
            }
            true
        } else {
            println!("-*-You can't put there.-*-");
            false
        }
    }

    fn rev(&mut self, coodinate : (usize, usize)) {
        if let Some(Some(color)) = self.colors.get(&coodinate) {
            self.colors.insert(coodinate, Some(color.rev()));
        }
    }
}

fn read_coodinate(player : &Color) -> Option<(usize, usize)> {
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
            if w < WIDTH && h < HEIGHT {
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

fn main() {
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
                let coodinate = read_coodinate(&player);

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

        if board.colors.iter()
            .filter(|(_, color)| color.is_none())
            .count() == 0 ||
            (!board.putable(&player) && !board.putable(&player.rev()))
        {
            let count_color = |col : Color| board.colors.iter()
                .filter(|(_, color)| {
                    if let Some(c) = color {col == *c} else {false}
                }).count();
            
            let white_num = count_color(Color::White);
            
            let black_num = count_color(Color::Black);
            
            let winner =
                if white_num > black_num {
                    Some(Color::White)
                } else if white_num < black_num  {
                    Some(Color::Black)
                } else {
                    None
                };

            result(winner);
            board.print();
            break 'main_loop;
        }
        player = player.rev();
    }
}