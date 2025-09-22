mod game;
mod board;
mod piece;

use std::io;
use std::process::exit;
use crossterm::{execute, style::{self, Stylize}, cursor};
use crossterm::terminal::{Clear, ClearType};

fn main() {
    loop {
        menu();
    }
}

fn menu() {
    let mut stdout = io::stdout();

    // Menu display
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        style::PrintStyledContent( "Tetris-rust : Tet-rust".white()),
        cursor::MoveToNextLine(1),
        style::PrintStyledContent( "1. start new mod".white()),
        cursor::MoveToNextLine(1),
        style::PrintStyledContent( "2. show high scores".white()),
        cursor::MoveToNextLine(1),
        style::PrintStyledContent( "3. quit".white()),
        cursor::MoveToNextLine(1),
    ).unwrap();

    // Read choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().unwrap_or_else(|_| 0);

    // Handle choice
    match choice {
        1 => game::start_game(),
        2 => println!("high score"),
        3 => {
            exit(0);
        },
        _ => println!("unknown choice"),
    }
}

