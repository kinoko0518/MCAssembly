mod literal;
mod opecode;
mod scoreboard;
mod storage;

pub use literal::{FltLiteral, IntLiteral};
pub use opecode::{
    Command, Condition, IntoSingleString, Mnemonic, Qualified, Releasable, ScoreAddable,
    ScoreAssignable, ScoreDividable, ScoreMultiplicatable, ScoreSubtractable, ScoreSurplusable,
};
pub use scoreboard::Scoreboard;
pub use storage::{Path, Storage, StorageType};

#[derive(Debug)]
pub enum MCAsmError {
    NarrowingConversion,
    InvalidAssignment,
    EmptyLineGiven,
    TooFewOperand,
    InvalidOperand,
    UnknownMnemonic,
    UnknownType,
    InvalidScoreboard,
    InvalidStorage,
    CantImplicateAsUnsignedInteger,
}
