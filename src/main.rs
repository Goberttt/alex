#[allow(dead_code)]

use std::io;
use std::io::Write;

mod vis;
use vis::printing;

mod helpers;
use helpers::move_conversions::move_from_str;

mod board;
use board::Board;

#[derive(Clone, Copy)]
pub enum Color {
    Empty,
    White,
    Red,
    Green,
    Blue,
}

pub enum Move {
    Step(usize),
    Wall([usize; 3]),
}

pub enum IoState{
    Hello,
    Await,
    Help(Option<String>),
    NewBoard,
    PlayMoves,
//  ApplyMove(Move),
    ShowBoard,
//  ShowCustom(Vec<[usize; 2]>, [[[Color; 8]; 8]; 2], [[Color; 9]; 9]),
}
use crate::IoState::*;







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
                    Ok(()) => println!("Board updated!"),
                    Err(e) => println!("{e}"),
                } 
                state = Await; 
                input = None;
            },
            ShowBoard => {
                printing::print_board(&current_board);
                state = Await;
                input = None
            },
        }
    }
}

fn help(s: &Option<String>) {
    match s {
        None => println!{"The following commands are supported:
help, new, move, show and exit. Type help <command> to receive help about a command.
Type help encoding to see help about the move encoding."},
        Some(c) => match c.as_str() {
            "help" => println!("Type help <command> to receive help about a command.
Example: >> help new.
Type help encoding to see help about the move encoding."),
            "new" => println!("Resets the board.
This command can be followed by a valid move sequence like this: >> new S N S N."),
            "move" => println!("Must be followed by a valid move sequence.
Applies this move sequence to the current board.
Example: >> move S N Hd3."),
            "show" => println!("Prints the current board."),
            "exit" => println!("Exits the application."),
            "encoding" => println!("The moves are encoded as follows:
Moving the pawn is encoded by cardinal directions, i.e. N, E, S, W.
A jump is encoded by two consequtive such directions, like NN or NW for a sideways jump.
Walls are encoded by H and V standing for horizontal and vertical.
This is followed by the coordinates of the center of the wall, like this: Hd3 He7 Va1 Vb5."),
            _ => {println!("I'm sorry, I don't understand the input."); help(&None);},
        }
    }
}

fn get_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input. Please only enter valid UTF-8.");
    input.trim().splitn(2, " ").map(|x| x.to_string()).collect()
}
