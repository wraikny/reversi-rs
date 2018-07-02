use std::collections::{HashMap, HashSet};
use color::Color;

extern crate rayon;
use board::rayon::prelude::*;

#[derive(Clone)]
pub(crate) struct Board {
    pub(crate) colors : HashMap<(usize, usize), Option<Color>>,
    pub(crate) size : (usize, usize),
}

impl Board {
    pub(crate) fn new(size : (usize, usize)) -> Board {
        let (width, height) = size;

        let colors = (0..width).into_par_iter()
            .flat_map(|x| {
                (0..height).into_par_iter()
                    .map(move |y| ((x, y), None))
            }).collect();

        let mut board = Board{colors, size};
        
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

    pub(crate) fn display(&self) {
        let (width, height) = self.size;

        let current = format!("{}:{}\n{}:{}\n", 
            Color::Black, self.count_color(Color::Black), 
            Color::White, self.count_color(Color::White));
        
        let head = (0..width).fold("\\".to_string(), |s, x| format!("{} {}", s, x));

        let mut board : Vec<_> = self.colors.iter().collect();
        board.sort_by_key(|((w, _), _)| w);

        let table = (0..height).fold("".to_string(), |s, h| {
            format!("{}{} {}\n", s, h, {
                board.iter()
                .filter(|((_, h_), _)| *h_ == h)
                .fold("".to_string(), |s, ((_, _), color)| {
                    s + "" + match color {
                        Some(c) => c.sym(),
                        None => Color::sym_empty(),
                    } + " "
                })
            })
        });

        println!("{}\n{}\n{}", current, head, table);
    }

    fn rev_cdns(
        &self, coordinate : (usize, usize), 
        player : &Color
        ) -> HashSet<(usize, usize)> {
    
        let mut rev_cdns_set : HashSet<(usize, usize)> = HashSet::new();

        // if the coordinate in the keys and empty
        if self.colors.get(&coordinate).is_some() {
            let (w, h) = coordinate;

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
                if board.clone().par_iter()
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

        v.par_iter().any(|(x, y)| {
            let (w, h) = (w as i32 - x, h as i32 - y);
            let (w, h) = (w as usize, h as usize);

            if let Some(Some(color)) = self.colors.get(&(w, h)) {
                *color == player.rev()
            } else {false}
        })
    }

    pub(crate) fn putable_cdns(&self, player : &Color) -> Vec<(usize, usize)> {
        self.colors.par_iter()
            .filter(|(cdn, color)| {
                color.is_none() &&
                self.exist_nextto(**cdn, player) && 
                self.rev_cdns(**cdn, &player).par_iter().count() > 0
            }).map(|(cdn, _)| cdn.clone()).collect()
    }

    pub(crate) fn putable(&self, player : &Color) -> bool {
        self.putable_cdns(player).len() > 0
    }

    pub(crate) fn put(&mut self, coordinate : (usize, usize), player : &Color) -> &Board {
        let rev_cdns = self.rev_cdns(coordinate, player);
        if rev_cdns.len() > 0 {
            self.colors.insert(coordinate, Some(*player));
            for cdn in rev_cdns {
                self.rev(cdn);
            }
        }
        self
    }

    fn rev(&mut self, coordinate : (usize, usize)) {
        if let Some(Some(color)) = self.colors.get(&coordinate) {
            self.colors.insert(coordinate, Some(color.rev()));
        }
    }

    pub(crate) fn finished(&self, player : &Color) -> bool {
        self.colors.par_iter()
            .filter(|(_, color)| color.is_none())
            .count() == 0 ||
            (!self.putable(player) && !self.putable(&player.rev()))
    }

    fn count_color(&self, col : Color) -> usize {
        self.colors.par_iter()
            .filter(|(_, color)| {
                if let Some(c) = color {col == *c} else {false}
            }).count()
    }

    pub(crate) fn winner(&self) -> Option<Color> {
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