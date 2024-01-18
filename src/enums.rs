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

pub enum IoState {
    Hello,
    Await,
    Help(Option<String>),
    NewBoard(Option<String>),
    PlayMoves(String),
    PlayMovesNoCheck(String),
    ShowBoard,
    Set(Flag),
    SetNotation(Notation),
    Unset(Flag),
    Fill(String),
    Brute(usize),
    ForgetMoves,
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
    InputAfterForget,
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
    Notation,
    WrongInput,
}