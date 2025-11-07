use super::Scoreboard;
use crate::types::*;

pub trait Assignable {
    fn assign(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Addable {
    fn add(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Subtractable {
    fn sub(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Multiplicatable {
    fn mul(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Dividable {
    fn div(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Surplusable {
    fn sur(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Releasable {
    fn rel(&self) -> String;
}

enum Opecode {
    /// Define
    Def((String, Box<dyn Assignable>)),
    /// Move
    Mov((Scoreboard, Box<dyn Assignable>)),
    /// Addition
    Add((Scoreboard, Box<dyn Addable>)),
    /// Subtraction
    Sub((Scoreboard, Box<dyn Subtractable>)),
    /// Multiplication
    Mul((Scoreboard, Box<dyn Multiplicatable>)),
    /// Division
    Div((Scoreboard, Box<dyn Dividable>)),
    /// Surplus
    Sur((Scoreboard, Box<dyn Surplusable>)),
    /// Release
    Rel(Box<dyn Releasable>),
}
