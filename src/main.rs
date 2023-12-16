#[allow(dead_code)]

use std::io;
use std::io::Write;

mod vis;
use vis::printing::print_board;

mod helpers;
use helpers::move_conversions::move_from_str;
use helpers::{help, get_input};

mod board;
use board::Board;

mod enums;
use enums::IoState::*;

mod graph;









fn main() {
    let mut state = Hello;
    let mut input = None;
    let mut current_board = Board::new();
    loop {
        match state {
            Hello => {
                println!("Hi! :D Please enter a command. Type help for help");
                state = Await;
                input = None;
            },
            Await => {
                print!(">> "); io::stdout().flush().expect("flush failed!");

                let new = get_input();
                
                match (new.get(0), new.get(1)) {
                    (Some(s), None) => match s.as_str() {
                        "help" | "h" => state = Help(None),
                        "new" | "n" => state = NewBoard,
                        "move" | "m" => println!("Please enter the moves after the move command."),
                        "show" | "s" => state = ShowBoard,
                        "exit" | "end" | "quit" => break,
                        _ => println!("Unknown command."),
                        },
                    (Some(s), Some(i)) => match s.as_str() {
                            "help" | "h" => state = Help(Some((*i.clone()).to_string())),
                            "new" | "n" => {state = NewBoard; input = Some(i.clone())},
                            "move" | "m" => {state = PlayMoves; input = Some(i.clone())},
                            "show" | "s" => println!("Please type nothing after show."),
                            _ => println!("Unknown command."),
                        },
                    _ => (),
                };
            },
            Help(s) => {help(&s); state = Await; input = None;},
            NewBoard => {
                match input {
                    None => {println!("New board created!");current_board = Board::new();},
                    Some(s) => match Board::from(s.as_str()) {
                        Ok(b) => {println!("New board created!");current_board = b;},
                        Err(_) => println!("Invalid input!"),
                    },
                }
                state = Await;
                input = None;
            },
            PlayMoves => {
                match current_board.extend(input.unwrap().as_str()) {
                    Ok(()) => { println!("Board updated! Player {} to move,", current_board.to_move+1); state = ShowBoard },
                    Err(e) => { println!("{e}"); state = Await },
                };
                input = None;
            },
            ShowBoard => {
                print_board(&current_board);
                state = Await;
                input = None
            },
        }
    }
}