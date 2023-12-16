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
    ShowBoard,
}