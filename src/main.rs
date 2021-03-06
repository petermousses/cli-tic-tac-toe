use std::env;
use std::fmt;
use rand::Rng;
use std::io::stdin;
use ansi_term::{Style, Color::*};

#[derive(PartialEq)]
enum Tile {
    NONE,
    X,
    O,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Tile::X => write!(f, "X"),
            &Tile::O => write!(f, "O"),
            &Tile::NONE => write!(f, " "),
        }
    }
}
struct TicTacToe<'a> {
    player: Tile,
    computer: Tile,
    board: &'a mut Vec<Vec<Tile>>
}
impl<'a> fmt::Display for TicTacToe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match write!(f, "{}\n", Style::new().underline().paint("       ")) {
            Ok(_) => (),
            Err(e) => println!("Error printing out gameboard {}", e)
        }
        match write!(f, "|{} {} {}|\n", self.board[0][0], self.board[0][1], self.board[0][2]) {
            Ok(_) => (),
            Err(e) => println!("Error printing out gameboard {}", e)
        }
        match write!(f, "|{} {} {}|\n", self.board[1][0], self.board[1][1], self.board[1][2]) {
            Ok(_) => (),
            Err(e) => println!("Error printing out gameboard {}", e)
        }
        match match self.board[2][0] {
            Tile::X => write!(f, "{}", Style::new().underline().paint("|X ")),
            Tile::O => write!(f, "{}", Style::new().underline().paint("|O ")),
            Tile::NONE => write!(f, "{}", Style::new().underline().paint("|  ")),
        } {
            Ok(_) => (),
            Err(e) => println!("Error printing out gameboard {}", e)
        }
        match match self.board[2][1] {
            Tile::X => write!(f, "{}", Style::new().underline().paint("X ")),
            Tile::O => write!(f, "{}", Style::new().underline().paint("O ")),
            Tile::NONE => write!(f, "{}", Style::new().underline().paint("  ")),
        } {
            Ok(_) => (),
            Err(e) => println!("Error printing out gameboard {}", e)
        }
        match self.board[2][2] {
            Tile::X => write!(f, "{}", Style::new().underline().paint("X|")),
            Tile::O => write!(f, "{}", Style::new().underline().paint("O|")),
            Tile::NONE => write!(f, "{}", Style::new().underline().paint(" |")),
        }
    }
}
impl<'a> TicTacToe<'a> {
    fn copy_tile(&self, to_copy: &Tile) -> Tile {
        match to_copy {
            &Tile::X => Tile::X,
            &Tile::O => Tile::O,
            &Tile::NONE => Tile::NONE
        }
    }
    fn choose_coords(&self) -> [usize; 2] {
        [self.choose_coord(true), self.choose_coord(false)]
    }
    fn choose_coord(&self, row_t_or_col_f: bool) -> usize {
        let mut coord;
        let mut coord_num;
        if row_t_or_col_f   { println!("Enter the row:\t"); }
        else                { println!("Enter the column:\t"); }
        loop {
            coord = String::new();
            match stdin().read_line(&mut coord) {
                Ok(_) => (),
                Err(e) => println!("Error while reading line: {}", e)
            }
            match coord.trim().parse() {
                Ok(num) => coord_num = num,
                Err(_) => {
                    println!("{}", Red.paint("Please enter a number between 1 and 3:"));
                    continue
                }
            };
            if coord_num >= 1 && coord_num <= 3 { break }
            println!("{}", Red.paint("Please enter a number between 1 and 3:"));
        }
        coord_num
    }
    fn play(&mut self) {
        let mut winner = Tile::NONE;
        while winner == Tile::NONE && !self.is_board_full() {
            println!("{}", self);
            let coordinates = self.choose_coords();
            println!("{} plays at {:?}", self.player, coordinates);
            match self.board[coordinates[0] - 1][coordinates[1] - 1] {
                Tile::NONE => self.board[coordinates[0] - 1][coordinates[1] - 1] = self.copy_tile(&self.player),
                _ => {
                    println!("{}", Red.paint("This spot is taken, try again"));
                    continue;
                }
            }
            self.computer_turn();
            winner = self.check_winner();
        }
        println!("{}", self);
        if winner == self.player {
            println!("{}", Green.paint("You win!"));
        } else if winner == self.computer {
            println!("{}", Red.paint("Computer wins :("));
        } else {
            println!("{}", Blue.paint("Cat's game, no winner."));
        }
    }
    fn computer_turn(&mut self) {
        if self.is_board_full() { return }
        loop {
            let x_coord = rand::thread_rng().gen_range(0, 3);
            let y_coord = rand::thread_rng().gen_range(0, 3);
            if self.board[x_coord][y_coord] == Tile::NONE {
                self.board[x_coord][y_coord] = self.copy_tile(&self.computer);
                return
            }
        }
    }
    fn check_winner(&self) -> Tile {
        // Vertical
        if self.board[0][0] == self.board[1][0] && self.board[1][0] == self.board[2][0] { return self.copy_tile(&self.board[0][0]) }
        if self.board[0][1] == self.board[1][1] && self.board[1][1] == self.board[2][1] { return self.copy_tile(&self.board[0][1]) }
        if self.board[0][2] == self.board[1][2] && self.board[1][2] == self.board[2][2] { return self.copy_tile(&self.board[0][2]) }
        // Horizontal
        if self.board[0][0] == self.board[0][1] && self.board[0][1] == self.board[0][2] { return self.copy_tile(&self.board[0][0]) }
        if self.board[1][0] == self.board[1][1] && self.board[1][1] == self.board[1][2] { return self.copy_tile(&self.board[1][0]) }
        if self.board[2][0] == self.board[2][1] && self.board[2][1] == self.board[2][2] { return self.copy_tile(&self.board[2][0]) }
        // Diagonal
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] { return self.copy_tile(&self.board[0][0]) }
        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] { return self.copy_tile(&self.board[0][2]) }
        // No winner yet
        Tile::NONE
    }
    fn is_board_full(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Tile::NONE {
                    return false
                }
            }
        }
        true
    }
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    let character;
    let computer;
    if cmd_args.len() > 1 {
        (character, computer) = match cmd_args[1].to_uppercase().trim() {
            "X" => {
                println!("You are X.");
                (Tile::X, Tile::O)
            },
            "O" => {
                println!("You are O.");
                (Tile::O, Tile::X)
            },
            _ => choose_character()
        }
    } else {
        (character, computer) = choose_character();
    }
    TicTacToe {
        player: character,
        computer: computer,
        board: &mut vec![
            vec![Tile::NONE, Tile::NONE, Tile::NONE],
            vec![Tile::NONE, Tile::NONE, Tile::NONE],
            vec![Tile::NONE, Tile::NONE, Tile::NONE],
        ],
    }.play();
}

fn choose_character() -> (Tile, Tile) {
    let mut char;
    println!("Choose your character, X or O:");
    loop {
        char = String::new();
        match stdin().read_line(&mut char) {
            Ok(_) => (),
            Err(e) => println!("Error while reading line: {}", e)
        }
        match char.trim().to_string().to_uppercase().as_str() {
            "X" => return (Tile::X, Tile::O),
            "O" => return (Tile::O, Tile::X),
            _ => {
                println!("{}", Red.paint("Please enter either X or O:"));
                continue;
            }
        }
    }
}