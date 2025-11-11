use super::Scoreboard;
use crate::types::{storage::Storage, *};

pub trait ScoreAssignable {
    fn assign(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait ScoreAddable {
    fn add(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait ScoreSubtractable {
    fn sub(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait ScoreMultiplicatable {
    fn mul(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait ScoreDividable {
    fn div(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait ScoreSurplusable {
    fn sur(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError>;
}

pub trait Releasable {
    fn rel(&self) -> String;
}

pub trait ScoreCompareble {
    fn cmp(
        &self,
        unless: bool,
        comparison: &str,
        scoreboard: &Scoreboard,
    ) -> Result<String, MCAsmError>;
}

pub enum Mnemonic {
    /// Define Mnemonic
    ///
    /// DEF <Scoreboard> <Source>
    ///
    /// Completedly same as MOV btw
    Def((Scoreboard, Box<dyn ScoreAssignable>)),
    /// Move Mnemonic
    ///
    /// MOV <Scoreboard> <Source>
    Mov((Scoreboard, Box<dyn ScoreAssignable>)),
    /// Addition Mnemonic
    ///
    /// ADD <Scoreboard> <Source>
    Add((Scoreboard, Box<dyn ScoreAddable>)),
    /// Subtraction Mnemonic
    ///
    /// SUB <Scoreboard> <Source>
    Sub((Scoreboard, Box<dyn ScoreSubtractable>)),
    /// Multiplication Mnemonic
    ///
    /// MUL <Scoreboard> <Source>
    Mul((Scoreboard, Box<dyn ScoreMultiplicatable>)),
    /// Division Mnemonic
    ///
    /// DIV <Scoreboard> <Source>
    Div((Scoreboard, Box<dyn ScoreDividable>)),
    /// Surplus Mnemonic
    ///
    /// SUR <Scoreboard> <Source>
    Sur((Scoreboard, Box<dyn ScoreSurplusable>)),
    /// NBT to Score Mnemonic
    ///
    /// NTS <Scoreboard> <StorageName> <NBTPath & Datatype(Unused)> <Magnification>
    Nts((Scoreboard, Storage, Path, u32)),
    /// Score to NBT Mnemonic
    ///
    /// STN <StorageName> <NBTPath & Datatype> <Scoreboard> <Magnification>
    Stn((Storage, Path, Scoreboard, f32)),
    /// Release Mnemoric
    ///
    /// REL <Scoreboard>
    Rel(Box<dyn Releasable>),
    /// Jump if Equal
    ///
    /// JE <Source> <Source> <Mnemonic>
    Je((Scoreboard, Box<dyn ScoreCompareble>, Box<Mnemonic>)),
    /// Jump if Not Equal
    ///
    /// JNE <Source> <Source> <Mnemonic>
    Jne((Scoreboard, Box<dyn ScoreCompareble>, Box<Mnemonic>)),
    /// Jump if Less Than
    ///
    /// JL <Source> <Source> <Mnemonic>
    Jl((Scoreboard, Box<dyn ScoreCompareble>, Box<Mnemonic>)),
    /// Jump if Greater Than
    ///
    /// JG <Source> <Source> <Mnemonic>
    Jg((Scoreboard, Box<dyn ScoreCompareble>, Box<Mnemonic>)),
}

impl Mnemonic {
    pub fn to_string(&self) -> Result<String, MCAsmError> {
        match self {
            Self::Def((score, assignable)) => assignable.assign(score),
            Self::Mov((score, assignable)) => assignable.assign(score),

            Self::Add((score, source)) => source.add(score),
            Self::Sub((score, source)) => source.sub(score),
            Self::Mul((score, source)) => source.mul(score),
            Self::Div((score, source)) => source.div(score),
            Self::Sur((score, source)) => source.sur(score),

            Self::Nts((score, storage, path, magnif)) => {
                Ok(storage.store_to_score(score, &path.path, *magnif))
            }
            Self::Stn((storage, path, score, magnif)) => {
                Ok(score.storage_to_score(storage, &path.type_annotation, &path.path, *magnif))
            }

            Self::Rel(releasable) => Ok(releasable.rel()),

            Self::Je((lhs, rhs, mnemonic)) => Ok(format!(
                "{}\n{}",
                rhs.cmp(false, "=", lhs)?,
                mnemonic.to_string()?
            )),
            Self::Jne((lhs, rhs, mnemonic)) => Ok(format!(
                "{}\n{}",
                rhs.cmp(true, "=", lhs)?,
                mnemonic.to_string()?
            )),
            Self::Jl((lhs, rhs, mnemonic)) => Ok(format!(
                "{}\n{}",
                rhs.cmp(false, "<", lhs)?,
                mnemonic.to_string()?
            )),
            Self::Jg((lhs, rhs, mnemonic)) => Ok(format!(
                "{}\n{}",
                rhs.cmp(false, ">", lhs)?,
                mnemonic.to_string()?
            )),
        }
    }
}
