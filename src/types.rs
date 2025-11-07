mod literal;
mod opecode;
mod scoreboard;
mod storage;

pub use literal::{FltLiteral, IntLiteral};
pub use opecode::{
    Addable, Assignable, Dividable, Multiplicatable, Releasable, Subtractable, Surplusable,
};
pub use scoreboard::{ACM, Scoreboard};

#[derive(Debug)]
pub enum MCAsmError {
    NarrowingConversionError,
}
