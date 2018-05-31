use std::collections::HashMap;

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

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::Black => "Black",
            Color::White => "White",
        })
    }
}

struct Board {
    colors: HashMap<(usize, usize), Option<Color>>,
}

impl Board {
    fn new() -> Board {
        let mut board = Board{
            colors: HashMap::new()
        };
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                board.colors.insert((x, y), None);
            }
        }
        board.colors.insert((3, 3), Some(Color::White));
        board.colors.insert((4, 4), Some(Color::White));
        board.colors.insert((3, 4), Some(Color::Black));
        board.colors.insert((4, 3), Some(Color::Black));
        board
    }

    fn print(&self) {
        let mut table : Vec<_> = self.colors.iter().collect();
        table.sort_by_key(|((w, _), _)| w);
        let table = table;

        let head = (0..WIDTH).fold(" ".to_string(), |s, x| format!("{} {}", s, x));

        let table = (0..HEIGHT).fold("".to_string(), |s, h| {
            format!("{}{}|{}\n", s, h, {
                table.iter().filter(|((_, h_), _)| *h_ == h)
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

    fn put(&mut self, cdn : (usize, usize), player : &Color) {
        let (w, h) = cdn;
        if w < WIDTH && h < HEIGHT {
            if let None = self.colors[&cdn] {
                self.colors.insert(cdn, Some(*player));
            }
        }
    }
}

fn read_cdn(player : &Color) -> Option<(usize, usize)> {
    let mut coodinate : Option<(usize, usize)> = None;
        
    while let None = coodinate {
        println!("Input coodinate of {} as 'w h'.(q: end)", player);
        let mut read = String::new();
        std::io::stdin().read_line(&mut read)
            .expect("Failed to read line.");
        
        // Quit the gam.
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
                break;
            }
        }
        println!("Input correct coodinate!");
    };

    coodinate
}

fn main() {
    println!("Reversi");

    let mut board = Board::new();

    let mut player = Color::Black;

    let mut winner : Option<Color> = None;

    'main_loop: while let None = winner {
        board.print();
        let mut coodinate : Option<(usize, usize)> = read_cdn(&player);

        if let Some((w, h)) = coodinate {
            board.put((w, h), &player);

        } else {
            break 'main_loop;
        }

        player = player.rev();
    }
}
