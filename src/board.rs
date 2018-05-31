use std::collections::{HashMap, HashSet};
use color::Color;

#[derive(Clone)]
pub struct Board {
    colors : HashMap<(usize, usize), Option<Color>>,
    pub size : (usize, usize),
}

impl Board {
    pub fn new(size : (usize, usize)) -> Board {
        let mut board = Board{
            colors: HashMap::new(), 
            size : size,
        };

        let (width, height) = size;

        for x in 0..width {
            for y in 0..height {
                board.colors.insert((x, y), None);
            }
        }
        
        {
            let mut insert = |cdn, color|{
                board.colors.insert(cdn, Some(color));
            };

            let (x, y) = (width / 2, height / 2);
            
            use self::Color::{White, Black};

            insert((x - 1, y - 1), White);
            insert((x, y), White);
            insert((x - 1, y), Black);
            insert((x, y - 1), Black);
        }

        board
    }

    pub fn print(&self) {
        let (width, height) = self.size;
        let head = (0..width).fold(" ".to_string(), |s, x| format!("{} {}", s, x));

        let mut board : Vec<_> = self.colors.iter().collect();
        board.sort_by_key(|((w, _), _)| w);

        let table = (0..height).fold("".to_string(), |s, h| {
            format!("{}{}|{}\n", s, h, {
                board.iter()
                .filter(|((_, h_), _)| *h_ == h)
                .fold("".to_string(), |s, ((_, _), color)| {
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

    fn rev_cdns(
        &self, coodinate : (usize, usize), 
        player : &Color
        ) -> HashSet<(usize, usize)> {
    
        let mut rev_cdns_set : HashSet<(usize, usize)> = HashSet::new();

        // if the coodinate in the keys and empty
        if self.colors.get(&coodinate).is_some() {
            let (w, h) = coodinate;

            let find_same = |board : &Vec<(&(usize, usize), &Option<Color>)>| {
                let found = board.iter()
                    .rev()
                    .find(|(_, color)| {
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
                if board.clone().iter()
                    .all(|(_, color)|{
                        if let Some(color) = color {
                            color != player
                        } else { false }
                }) {
                    board.iter().for_each(|(cdn, _)| {
                        rev_cdns_set.insert(**cdn);
                    });
                }
            };

            let mut search = |
                f : &Fn((usize, usize)) -> bool, 
                g : &Fn((usize, usize)) -> usize,
                h : &Fn((usize, usize), (usize, usize)) -> bool,
            | {
                let mut board : Vec<_> = 
                    self.colors.iter()
                    .filter(|(item, _)| f(**item))
                    .collect();
                
                board.sort_by_key(|(item, _)| g(**item));

                if let Some(cdnf) = find_same(&board) {
                    let mut board = board.iter()
                        .filter(|(item, _)|{
                            h(cdnf, **item)
                        }).collect();
                    add_revs(&board);
                }
            };

            let (width, height) = self.size;

            // Upside
            search(
                &|(x, y)| x == w && y < h,
                &|(_, y)| y,
                &|(_, hf), (_, y)| hf < y,
            );

            // Downside
            search(
                &|(x, y)| x == w && y > h,
                &|(_, y)| height - y,
                &|(_, hf), (_, y)| hf > y,
            );

            // Leftside
            search(
                &|(x, y)| y == h && x < w,
                &|(x, _)| x,
                &|(wf, _), (x, _)| wf < x,
            );

            // Rightside
            search(
                &|(x, y)| y == h && x > w,
                &|(x, _)| width - x,
                &|(wf, _), (x, _)| wf > x,
            );

            // Leftup
            search(
                &|(x, y)| (w + y) == (x + h) && x < w,
                &|(x, _)| x,
                &|(wf, _), (x, _)| wf < x,
            );

            // Rightdown
            search(
                &|(x, y)| (w + y) == (x + h) && x > w,
                &|(x, _)| width - x,
                &|(wf, _), (x, _)| wf > x,
            );

            // Leftdown
            search(
                &|(x, y)| (w + h) == (x + y) && x < w,
                &|(x, _)| x,
                &|(wf, _), (x, _)| wf < x,
            );

            // Rightup
            search(
                &|(x, y)| (w + h) == (x + y) && x > w,
                &|(x, _)| width - x,
                &|(wf, _), (x, _)| wf > x,
            );
        }

        rev_cdns_set
    }

    fn exist_nextto(&self, (w, h) : (usize, usize), player : &Color) -> bool {
        let v = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        v.iter().any(|(x, y)| {
            let (w, h) = (w as i32 - x, h as i32 - y);
            let (w, h) = (w as usize, h as usize);

            if let Some(Some(color)) = self.colors.get(&(w, h)) {
                *color == player.rev()
            } else {false}
        })
    }

    pub fn putable(&self, player : &Color) -> bool {
        self.colors.iter()
            .any(|(cdn, color)| {
                color.is_none() &&
                self.exist_nextto(*cdn, player) && 
                self.rev_cdns(*cdn, &player).iter().count() > 0
            })
    }

    pub fn put(&mut self, coodinate : (usize, usize), player : &Color) -> bool {
        let rev_cdns = self.rev_cdns(coodinate, player);
        if rev_cdns.iter().count() > 0 {
            self.colors.insert(coodinate, Some(*player));
            for cdn in rev_cdns {
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

    pub fn finished(&self, player : &Color) -> bool {
        self.colors.iter()
            .filter(|(_, color)| color.is_none())
            .count() == 0 ||
            (!self.putable(player) && !self.putable(&player.rev()))
    }

    fn count_color(&self, col : Color) -> usize {
        self.colors.iter()
            .filter(|(_, color)| {
                if let Some(c) = color {col == *c} else {false}
            }).count()
    }

    pub fn winner(&self) -> Option<Color> {
        let white_num = self.count_color(Color::White);
        let black_num = self.count_color(Color::Black);
        
        if white_num > black_num {
            Some(Color::White)
        } else if white_num < black_num  {
            Some(Color::Black)
        } else {
            None
        }
    }
}