// contains functionality for saving / loading a game to / from a file!()

use std::fs::File;
use std::io::prelude::*;

use crate::{
    Board, Board::*,
    Notation,
    LoadSaveError, LoadSaveError::* };



pub fn load(path: String, game_tag: String) -> Result<Board, LoadSaveError> {
    let file;
    match File::open() {
        Ok(f) => file = f,
        Err() => return FileReadError,
    }


    let games = String::new();
}
