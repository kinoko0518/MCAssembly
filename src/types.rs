mod literal;
mod opecode;
mod scoreboard;
mod storage;

pub use literal::{FltLiteral, IntLiteral};
pub use opecode::{
    Mnemonic, Releasable, ScoreAddable, ScoreAssignable, ScoreDividable, ScoreMultiplicatable,
    ScoreSubtractable, ScoreSurplusable,
};
pub use scoreboard::{ACM, Scoreboard};

#[derive(Debug)]
pub enum MCAsmError {
    NarrowingConversion,
    InvalidAssignment,
    EmptyLineGiven,
}
