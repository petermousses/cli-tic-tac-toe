use std::env;
use std::io::stdin;
use ansi_term::{Style, Color::*};
use rand::prelude::*;

#[derive(Debug)]
#[derive(PartialEq)]
enum Tile {
    NONE,
    X,
    O,
}
struct TicTacToe<'a> {
    player: Tile,
    board: &'a mut Vec<Vec<Tile>>
}
impl<'a> TicTacToe<'a> {
    fn copy_tile(&self, input: &Tile) -> Tile {
        match input {
            &Tile::X => Tile::X,
            &Tile::O => Tile::O,
            &Tile::NONE => Tile::NONE
        }
    }
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
        match self.board[1][0] {
            Tile::NONE => print!("  "),
            _ => print!("{:?} ", self.board[1][0])
        } match self.board[1][1] {
            Tile::NONE => print!("  "),
            _ => print!("{:?} ", self.board[1][1])
        } match self.board[1][2] {
            Tile::NONE => print!(" |\n"),
            _ => print!("{:?}|\n", self.board[1][2])
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
        let mut winner = Tile::NONE;
        while winner == Tile::NONE {
            if self.is_board_full() {
                return;
            }
            let coordinates = self.choose_coords();
            println!("{:?} plays at {:?}", self.player, coordinates);
            match self.board[coordinates[0] - 1][coordinates[1] - 1] {
                Tile::NONE => match self.player {
                    Tile::X => {
                        println!("at X");
                        self.board[coordinates[0] - 1][coordinates[1] - 1] = Tile::X;
                        println!("{:?}", self.board[coordinates[0] - 1][coordinates[1] - 1]);
                    },
                    Tile::O => self.board[coordinates[0] - 1][coordinates[1] - 1] = Tile::O,
                    _ => ()
                },
                _ => {
                    println!("{}", Red.paint("This spot is taken, try again"));
                    continue;
                }
            }
            self.other_turn();
            self.print();
            winner = self.check_winner();
        }
        if winner == self.player {
            println!("{}", Green.paint("You win!"));
        } else {
            println!("{}", Red.paint("Computer wins :("));
        }
    }
    fn other_turn(&mut self) {
        if  !self.board.iter().any(|x| x.iter().any(|y| y != &Tile::NONE)) 
            // !self.board.iter().any(|x| x.iter().any(|y| y == &Tile::NONE))
        { return; }
        println!("In here");
        loop {
            let mut shouldReturn = true;
            for row in 0..3 {
                for col in 0..3 {
                    if self.board[row][col] == Tile::NONE {
                        shouldReturn = false;
                    }
                }
            }
            if shouldReturn { return }
            let x_coord = rand::thread_rng().gen_range(0, 3);
            let y_coord = rand::thread_rng().gen_range(0, 3);
            if self.board[x_coord][y_coord] == Tile::NONE {
                self.board[x_coord][y_coord] = match self.player {
                    // Tile::X => Tile::O,
                    Tile::O => Tile::X,
                    _ => Tile::O
                };
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
        println!("{}", Blue.paint("Cat's game, no winner!"));
        true
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
        board: &mut vec![
            vec![Tile::NONE, Tile::NONE, Tile::NONE],
            vec![Tile::NONE, Tile::NONE, Tile::NONE],
            vec![Tile::NONE, Tile::NONE, Tile::NONE],
        ],
    };
    game.print();
    game.play();
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