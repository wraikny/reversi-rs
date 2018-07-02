use board::Board;
use color::Color;

extern crate rayon;
use cpu::rayon::prelude::*;

pub struct Setting {
    pub(crate) puttable : i32,
    pub(crate) corner : i32,
    pub(crate) aroundcorner : i32,
    pub(crate) wall : i32,
    pub(crate) all : i32,
    pub(crate) emptynum : usize,
}

impl Setting {
    pub fn new(puttable : i32, corner : i32, aroundcorner : i32, wall : i32, all : i32, emptynum : usize) -> Setting {
        Setting {
            puttable,
            corner,
            aroundcorner,
            wall,
            all,
            emptynum,
        }
    }
}

fn eval(board : &Board, player : &Color, cs : &Setting) -> i32 {
    let (mw, mh) = board.size;
    let (w, h) = (mw - 1, mh - 1);

    let count = |c| board.putable_cdns(&c).len() as i32;

    let corners = vec![(0, 0), (0, h), (w, 0), (w, h)];

    let nextcorners = vec![
        (0, 1), (1, 0), (1, 1),
        (1, h), (0, mh), (1, mh),
        (w, 1), (mw, 0), (mw, 1),
        (mw, h), (w, mh), (mw, mh),
    ];

    let count_corner = |c| {
        board.filter(|cdn| corners.contains(&cdn))
            .filter(|(_, color)|
                if let Some(color) = color {
                    *color == c
                } else {false}
            ).count() as i32
    };

    let next_corner = |c| {
        board.colors.par_iter()
            .filter(|(cdn, _)| nextcorners.contains(*cdn))
            .filter(|(_, color)|
                if let Some(color) = color {
                    *color == c
                } else {false}
            ).count() as i32
    };

    let on_wall = |c| {
        board.colors.par_iter()
            .filter(|(cdn, _)| !corners.contains(*cdn) && !nextcorners.contains(*cdn))
            .filter(|((x, y), _)| {
                (x - 0) * (x - w) * (y - 0) * (y - h) == 0
            })
            .filter(|(_, color)|
                if let Some(color) = color {
                    *color == c
                } else {false}
            ).count() as i32
    };

    // p : player, o : oposite
    let (pc, oc) = (*player, player.rev());

    let (p, o) = (count(pc), count(oc));
    let (cp, co) = (count_corner(pc), count_corner(oc));
    let (wp, wo) = (on_wall(pc), on_wall(oc));

    let en = board.colors.par_iter().filter(|(_, color)| color.is_none()).count();

    // Set good parameter
    (p - o) * cs.puttable + 
    (cp - co) * cs.corner + 
    (wp - wo) * cs.wall + 
    if en < cs.emptynum {
        let (pn, on) = (board.count_color(pc) as i32, board.count_color(oc) as i32);
        (pn - on) * cs.all
    } else {
        let (ncp, nco) = (next_corner(pc), next_corner(oc));
        (nco - ncp) * cs.aroundcorner
    }
}

fn alpha_beta(board : &Board, player : &Color, turn : Color, ev : (i32, i32), depth : usize, cs : &Setting) -> i32 {
    let new = |cdn, ev| {
        alpha_beta(board.clone().put(cdn, &turn), player, turn.rev(), ev, depth - 1, cs)
    };

    let putable = board.putable(&turn);

    if !putable && !board.putable(&turn.rev()) {
        if let Some(winner) = board.winner() {
            // 2^20
            1048576 * (if winner == *player { 1 } else { -1 })
        } else {
            0
        }
    } else if !putable || depth == 0 {
        eval(&board, player, cs)
    } else {
        let (mut a, mut b) = ev;
        if *player == turn {
            for cdn in board.putable_cdns(&turn) {
                a = a.max(new(cdn, (a, b)));
                if a >= b { break; }
            }
            a
        } else {
            for cdn in board.putable_cdns(&turn) {
                b = b.min(new(cdn, (a, b)));
                if a >= b { break; }
            }
            b
        }
    }
}

pub(crate) fn select(player : &Color, board : &Board, depth : usize, cs : &Setting) -> Option<(usize, usize)> {
    let cdns = board.putable_cdns(player);

    if cdns.len() == 0 {
        None
    } else {
        let inf = 2_i32.pow(20);

        let fc = cdns[0].clone();

        let (cdn, _) = cdns.into_par_iter().map(|cdn| {
            let v = alpha_beta(board.clone().put(cdn, player), player, *player, (-inf, inf), depth, cs);
            (cdn, v)
        }).reduce(|| (fc, -inf), |(cdn1, v1), (cdn2, v2)| {
            if v1 >= v2 {
                (cdn1, v1)
            } else {
                (cdn2, v2)
            }
        });

        Some(cdn)
    }
}