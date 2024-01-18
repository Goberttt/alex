use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs;

mod vis;
use crate::vis::print_board;

mod helpers;
use crate::helpers::move_from_str;
use crate::helpers::{ help, get_and_parse_input };

mod board;
use board::Board;

mod enums;
use crate::enums::{
    IoState, IoState::*,
    Notation, Notation::*,
    ParseError, ParseError::*,
    MoveError, MoveError::*,
    Flag, Flag::*,
    HelpMessage };

mod game;
use crate::game::GameState;
use crate::game::brute_force;

pub struct InteractiveInstance {
    pub board: Board,
    pub state: IoState,
    pub input: Option<String>,
    pub flags: HashMap<Flag, bool>,
    pub notation: Notation,
    pub parse_errors: HashMap<ParseError, String>,
    pub move_errors: HashMap<MoveError, String>,
    pub help_messages: HashMap<HelpMessage, String>,
}

fn main() {

    let mut instance = init();

    loop {
        match instance.state {
            Hello => {
                println!("\n\n\n    Hi! :D Please enter a command. Type 'help' for help");
                instance.state = Await;
            },
            Await => {
                print!(">> "); io::stdout().flush().expect("flush failed!");
                match get_and_parse_input() {
                    Ok(state) => instance.state = state,
                    Err(err) => println!("    {}", instance.parse_errors.get(&err).unwrap()),
                }
            },
            Help(s) => { help(&s, &instance.help_messages); instance.state = Await; },
            NewBoard(input) => {
                match input {
                    None => {println!("    New board created!"); instance.board = Board::new();},
                    Some(s) => match Board::from(s.as_str(), &instance.move_errors, instance.notation.clone()) {
                        Ok(b) => {println!("    New board created!"); instance.board = b;},
                        Err(_) => println!("    Invalid input!"),
                    },
                }
                instance.state = Await;
                instance.input = None;
            },
            PlayMoves(input) => {
                match instance.board.extend(input.as_str(), &instance.move_errors, instance.notation.clone()) {
                    Ok(()) => { println!("    Board updated! Player {} to move.", instance.board.to_move+1); instance.state = ShowBoard },
                    Err(e) => { println!("    {e}"); instance.state = Await },
                };
            },
            PlayMovesNoCheck(input) => { instance.board.extend_no_check(input.as_str(), instance.notation.clone()); 
                println!("    Board updated! Player {} to move.", instance.board.to_move+1); instance.state = ShowBoard },
            ShowBoard => {
                print_board(&instance);
                instance.state = Await;
            },
            Set(flag) => { *instance.flags.get_mut(&flag).unwrap() = true; instance.state = Await; },
            SetNotation(not) => { instance.notation = not; instance.state = Await; },
            Unset(flag) => { *instance.flags.get_mut(&flag).unwrap() = false; instance.state = Await; },
            Brute(depth) => { 
                println!("    Score is {}", brute_force(GameState {
                    board: instance.board.clone(),
                    children: vec![],
                    parent: None,
                    mv_from_parent: None,
                    score: None}, depth, instance.notation.clone() )); instance.state = Await;
            },
            Fill(input) => match instance.board.try_fill_from_str(input.clone()) {
                Ok(()) => { println!("    Fill successful"); instance.state = ShowBoard },
                Err(err) => { println!("    {}", instance.parse_errors.get(&err).unwrap()); instance.state = Await },
            },
            ForgetMoves => { instance.board.move_sequence.clear(); instance.state = Await; },
            Quit => break,
        };
    }
}

fn init() -> InteractiveInstance {

//create parse_errors map
    let mut parse_errors = HashMap::new();
    let parse_errors_from_file = fs::read_to_string("Messages/ParseErrors")
        .expect("ParseErrors file missing or corrupted.");
    for error_pair_joined in parse_errors_from_file.split("\n") {
        let error_pair: Vec<String> = error_pair_joined.split(": ").map(|x| x.to_string()).collect();
        match error_pair[0].as_str() {
            "NoMovesGiven" => { parse_errors.insert(NoMovesGiven, error_pair[1].clone()); },
            "NoFlagGiven" => { parse_errors.insert(NoFlagGiven, error_pair[1].clone()); },
            "UnknownFlag" => { parse_errors.insert(UnknownFlag, error_pair[1].clone()); },
            "UnknownNotation" => { parse_errors.insert(UnknownNotation, error_pair[1].clone()); },
            "NoNotationGiven" => { parse_errors.insert(NoNotationGiven, error_pair[1].clone()); },
            "NoFillChordsGiven" => { parse_errors.insert(NoFillChordsGiven, error_pair[1].clone()); },
            "FillChordsIncorrect" => { parse_errors.insert(FillChordsIncorrect, error_pair[1].clone()); },
            "UnknownCommand" => { parse_errors.insert(UnknownCommand, error_pair[1].clone()); },
            "InputAfterShow" => { parse_errors.insert(InputAfterShow, error_pair[1].clone()); },
            "InputAfterForget" => { parse_errors.insert(InputAfterForget, error_pair[1].clone()); },
            "BruteNoDepthGiven" => { parse_errors.insert(BruteNoDepthGiven, error_pair[1].clone()); },
            "InvalidMove" => { parse_errors.insert(InvalidMove, error_pair[1].clone()); },
            "NotANumber" => { parse_errors.insert(NotANumber, error_pair[1].clone()); },
            _ => (),
        }
    }

//create move_errors map
    let mut move_errors = HashMap::new();
    let move_errors_from_file = fs::read_to_string("Messages/MoveErrors")
        .expect("MoveErrors file missing or corrupted.");
    for error_pair_joined in move_errors_from_file.split("\n") {
        let error_pair: Vec<String> = error_pair_joined.split(": ").map(|x| x.to_string()).collect();
        match error_pair[0].as_str() {
            "EdgeOfBoard" => { move_errors.insert(EdgeOfBoard, error_pair[1].clone()); },
            "BlockedByWall" => { move_errors.insert(BlockedByWall, error_pair[1].clone()); },
            "BlockedByOpponent" => { move_errors.insert(BlockedByOpponent, error_pair[1].clone()); },
            "OpponentNotThere" => { move_errors.insert(OpponentNotThere, error_pair[1].clone()); },
            "SpaceBehindFree" => { move_errors.insert(SpaceBehindFree, error_pair[1].clone()); },
            "NoWallsLeft" => { move_errors.insert(NoWallsLeft, error_pair[1].clone()); },
            "SpaceOccupied" => { move_errors.insert(SpaceOccupied, error_pair[1].clone()); },
            "P1NoPath" => { move_errors.insert(P1NoPath, error_pair[1].clone()); },
            "P2NoPath" => { move_errors.insert(P2NoPath, error_pair[1].clone()); },
            _ => (),
        }
    }

//create help_messages map
    let mut help_messages = HashMap::new();
    let help_messages_from_file = fs::read_to_string("Messages/Help")
        .expect("Help file missing or corrupted.");
    for message_pair_joined in help_messages_from_file.split("\n\n") {
        let message_pair: Vec<String> = message_pair_joined.split(": ").map(|x| x.to_string()).collect();
        match message_pair[0].as_str() {
            "General" => { help_messages.insert(HelpMessage::General, message_pair[1].clone()); },
            "New" => { help_messages.insert(HelpMessage::New, message_pair[1].clone()); },
            "Move" => { help_messages.insert(HelpMessage::Move, message_pair[1].clone()); },
            "Show" => { help_messages.insert(HelpMessage::Show, message_pair[1].clone()); },
            "Set" => { help_messages.insert(HelpMessage::Set, message_pair[1].clone()); },
            "Unset" => { help_messages.insert(HelpMessage::Unset, message_pair[1].clone()); },
            "Brute" => { help_messages.insert(HelpMessage::Brute, message_pair[1].clone()); },
            "Encoding" => { help_messages.insert(HelpMessage::Notation, message_pair[1].clone()); },
            "WrongInput" => { help_messages.insert(HelpMessage::WrongInput, message_pair[1].clone()); }
            _ => (),
        }
    }

//create flags map
    let mut flags = HashMap::new();
    flags.insert(Invert, false);

    InteractiveInstance {
        board: Board::new(),
        state: Hello,
        input: None,
        flags: flags,
        notation: Relative,
        parse_errors: parse_errors,
        move_errors: move_errors,
        help_messages: help_messages,
    }
}

//m N E N E N S N S N S N S E S E S Ha3 Hc3 He3 Hc4 Hc6 Ha6 Hh2 Hf2 Hh3 Hg4 Hg7 Vc2 Vd5 Vc7 Ve6 Ve4 Vh4 He5