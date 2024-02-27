#[derive(Clone, Copy)]
pub enum Color {
    Empty,
    White,
//    Red,
//    Green,
//    Blue,
}

#[derive(Debug, Clone)]
pub enum Move {
    Step(usize),
    Wall([usize; 3]),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Player {
    Player1,
    Player2
}

pub enum IoState {
    Hello,
    Await,
    Help(Option<String>),
    NewBoard,
    NewBoard5x5,
    PlayMoves(String),
    PlayMovesNoCheck(String),
    ShowBoard,
    Set(Flag),
    SetNotation(Notation),
    Unset(Flag),
    Fill(String),
    Brute(usize),
    Brute5x5(usize),
    Undo(usize),
    Quit,
}

#[derive(Clone, Copy)]
pub enum Notation {
    Relative,
    Absolute,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Flag {
    Invert,
}

#[derive(Eq, Hash, PartialEq)]
pub enum ParseError {
    NoMovesGiven,
    NoFlagGiven,
    UnknownFlag,
    UnknownNotation,
    NoNotationGiven,
    NoFillChordsGiven,
    FillChordsIncorrect,
    UnknownCommand,
    InputAfterShow,
    BruteNoDepthGiven,
    InvalidMove,
    NotANumber,
    NotEnoughToUndo,
    InputAfterNew,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum MoveError {
    EdgeOfBoard,
    BlockedByWall,
    BlockedByOpponent,
    OpponentNotThere,
    SpaceBehindFree,
    NoWallsLeft,
    SpaceOccupied,
    P1NoPath,
    P2NoPath,
    GameIsOver,
}

#[derive(Eq, Hash, PartialEq)]
pub enum HelpMessage {
    General,
    New,
    Move,
    Show,
    Set,
    Unset,
    Brute,
    Fill,
    Undo,
    Notation,
    WrongInput,
}