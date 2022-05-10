use std::env;
use std::io::stdin;
use ansi_term::{Style, Color::Red, Color::Green};

#[derive(Debug)]
#[derive(PartialEq)]
enum Tile {
    NONE,
    X,
    O,
}
struct TicTacToe<'a> {
    player: Tile,
    board: &'a mut [[Tile; 3]; 3]
}
impl<'a> TicTacToe<'a> {
    fn choose_coords(&self) -> [usize; 2] {
        [self.choose_row(), self.choose_col()]
    }
    fn choose_row(&self) -> usize {
        let mut row;
        let mut row_num;
        println!("Enter the row:\t");
        loop {
            row = String::new();
            match stdin().read_line(&mut row) {
                Ok(_) => (),
                Err(e) => println!("Error while reading line: {}", e)
            }
            match row.trim().parse() {
                Ok(num) => row_num = num,
                Err(_) => {
                    println!("{}", Red.paint("Please enter a number between 1 and 3:"));
                    continue;
                }
            };
            if row_num < 1 || row_num > 3 {
                println!("{}", Red.paint("Please enter a number between 1 and 3:"));
            } else {
                break;
            }
        }
        row_num
    }
    fn choose_col(&self) -> usize {
        let mut col;
        let mut col_num;
        println!("Enter the column:\t");
        loop {
            col = String::new();
            match stdin().read_line(&mut col) {
                Ok(_) => (),
                Err(e) => println!("Error while reading line: {}", e)
            }
            match col.trim().parse() {
                Ok(num) => col_num = num,
                Err(_) => {
                    println!("{}", Red.paint("Please enter a number between 1 and 3:"));
                    continue;
                }
            };
            if col_num < 1 || col_num > 3 {
                println!("{}", Red.paint("Please enter a number between 1 and 3:"));
            } else {
                break;
            }
        }
        col_num
    }
    fn print(&self) {
        println!("{}", Style::new().underline().paint("       "));
        print!("|");
        match self.board[0][0] {
            Tile::NONE => print!("  "),
            _ => print!("{:?} ", self.board[0][0])
        } match self.board[0][1] {
            Tile::NONE => print!("  "),
            _ => print!("{:?} ", self.board[0][1])
        } match self.board[0][2] {
            Tile::NONE => print!(" |\n|"),
            _ => print!("{:?}|\n|", self.board[0][2])
        }
        match self.board[0][0] {
            Tile::NONE => print!("  "),
            _ => print!("{:?} ", self.board[0][0])
        } match self.board[0][1] {
            Tile::NONE => print!("  "),
            _ => print!("{:?} ", self.board[0][1])
        } match self.board[0][2] {
            Tile::NONE => print!(" |\n"),
            _ => print!("{:?}|\n", self.board[0][2])
        }
        match self.board[2][0] {
            Tile::X => print!("{}", Style::new().underline().paint("|X ")),
            Tile::O => print!("{}", Style::new().underline().paint("|O ")),
            Tile::NONE => print!("{}", Style::new().underline().paint("|  ")),
        }
        match self.board[2][1] {
            Tile::X => print!("{}", Style::new().underline().paint("X ")),
            Tile::O => print!("{}", Style::new().underline().paint("O ")),
            Tile::NONE => print!("{}", Style::new().underline().paint("  ")),
        }
        match self.board[2][2] {
            Tile::X => println!("{}", Style::new().underline().paint("X|")),
            Tile::O => println!("{}", Style::new().underline().paint("O|")),
            Tile::NONE => println!("{}", Style::new().underline().paint(" |")),
        }
    }
    fn play(&mut self) {
        loop {
            let coordinates = self.choose_coords();
            println!("{:?} plays at {:?}", self.player, coordinates);
            match self.board[coordinates[0]][coordinates[1]] {
                Tile::NONE => match self.player {
                    Tile::X => self.board[coordinates[0]][coordinates[1]] = Tile::X,
                    Tile::O => self.board[coordinates[0]][coordinates[1]] = Tile::O,
                    _ => ()
                },
                _ => println!("{}", Red.paint("This spot is taken, try again"))
            }
            self.print();
            break;
        }
        match self.check_winner() {
            Tile::NONE => println!("{}", Green.paint("You win!")),
            _ => println!("{}", Red.paint("Computer wins :(")),
        }
        
    }
    fn check_winner(&self) -> Tile {
        Tile::NONE
    }
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    let character;
    if cmd_args.len() > 1 {
        character = match cmd_args[1].to_uppercase().trim() {
            "X" => {
                println!("You are X.");
                Tile::X
            },
            "O" => {
                println!("You are O.");
                Tile::O
            },
            _ => choose_character()
        }
    } else {
        character = choose_character();
    }
    let mut game = TicTacToe {
        player: character,
        board: &mut [
            [Tile::NONE, Tile::NONE, Tile::NONE],
            [Tile::NONE, Tile::NONE, Tile::NONE],
            [Tile::NONE, Tile::NONE, Tile::NONE],
        ],
    };
    game.print();
    game.play();
    game.print();
}

fn choose_character() -> Tile {
    let mut char;
    println!("Choose your character, X or O:");
    loop {
        char = String::new();
        match stdin().read_line(&mut char) {
            Ok(_) => (),
            Err(e) => println!("Error while reading line: {}", e)
        }
        match char.trim().to_string().to_uppercase().as_str() {
            "X" => return Tile::X,
            "O" => return Tile::O,
            _ => {
                println!("{}", Red.paint("Please enter either X or O:"));
                continue;
            }
        }
    }
}