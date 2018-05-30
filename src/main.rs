const WIDTH : usize = 8;
const HEIGHT : usize = 8;

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Color {
    fn rev(&mut self) {
        match self {
            Color::Black => {*self = Color::White},
            Color::White => {*self = Color::Black},
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

struct Table {
    Colors: [[Option<Color>; WIDTH]; HEIGHT],
}

impl Table {
    fn new() -> Table {
        let mut table = Table{Colors: [[None; 8]; 8]};
        table.Colors[3][3] = Some(Color::White);
        table.Colors[4][4] = Some(Color::White);
        table.Colors[3][4] = Some(Color::Black);
        table.Colors[4][3] = Some(Color::Black);
        table
    }

    fn print(&self) {
        let head = (0..WIDTH).fold(" ".to_string(), |s, x| format!("{} {}", s, x));
        let result = self.Colors.iter().enumerate().map(|(i, x)| {
            x.iter().fold(format!("{}|", i).to_string(), |s, y| {
                s + "" + match y {
                    Some(Color::Black) => "B",
                    Some(Color::White) => "W",
                    None => " ",
                } + "|"
            })
        }).fold("".to_string(), |s, x| s + &x + "\n");

        println!("{}\n{}", head, result);
    }

    fn put(&mut self, color: &Color, (w, h): (usize, usize)) -> bool {
        if w > WIDTH || h > HEIGHT {
            false
        } else {
            match self.Colors[h][w] {
                None => {
                    self.Colors[h][w] = Some(*color);
                    true
                },
                _ => false,
            }
        }
    }
}

fn main() {
    println!("Reversi");

    let mut table = Table::new();

    let mut player = Color::Black;

    let mut winner : Option<Color> = None;

    'main_loop: while let None = winner {
        table.print();
        let mut coodinate : Option<(usize, usize)> = None;
        
        while let None = coodinate {
            println!("Please input {} coodinate.(q: end)", &player);
            let mut read = String::new();
            std::io::stdin().read_line(&mut read)
                .expect("Failed to read line.");
            
            if read.trim() == "q".to_string() {
                break 'main_loop;
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
            println!("Input correctly!");
        };

        if let Some((w, h)) = coodinate {
            println!("({}, {})", w, h);

            table.put(&player, (w, h));
        }

        player.rev();
    }
}
