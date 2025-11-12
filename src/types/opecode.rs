use super::Scoreboard;
use crate::types::{storage::Storage, *};

#[derive(Clone, Debug)]
pub struct Command {
    pub command: String,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        Self { command: value }
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.command
    }
}

#[derive(Clone, Debug)]
pub struct Condition {
    condition: String,
}

impl From<String> for Condition {
    fn from(value: String) -> Self {
        Self { condition: value }
    }
}

#[derive(Clone, Debug)]
pub struct Qualified {
    command: Command,
    conditions: Option<Vec<Condition>>,
}

impl Qualified {
    pub fn serialise(self) -> String {
        let condition = self.conditions.map(|c| {
            c.into_iter()
                .map(|c| c.condition)
                .collect::<Vec<String>>()
                .join(" ")
        });
        format!(
            "{}{}",
            condition
                .map(|c| format!("execute {} run ", c))
                .unwrap_or(String::from("")),
            self.command.command
        )
    }
    pub fn push_condition(&mut self, condition: Condition) {
        match &mut self.conditions {
            Some(c) => c.push(condition),
            None => self.conditions = Some(vec![condition]),
        }
    }
}

impl From<String> for Qualified {
    fn from(value: String) -> Self {
        Self {
            command: Command::from(value),
            conditions: None,
        }
    }
}

impl From<Command> for Qualified {
    fn from(value: Command) -> Self {
        Self {
            command: value,
            conditions: None,
        }
    }
}

impl From<(Command, Vec<Condition>)> for Qualified {
    fn from(value: (Command, Vec<Condition>)) -> Self {
        Self {
            command: value.0,
            conditions: Some(value.1),
        }
    }
}

pub trait ScoreAssignable {
    fn assign(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError>;
}

pub trait ScoreAddable {
    fn add(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError>;
}

pub trait ScoreSubtractable {
    fn sub(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError>;
}

pub trait ScoreMultiplicatable {
    fn mul(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError>;
}

pub trait ScoreDividable {
    fn div(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError>;
}

pub trait ScoreSurplusable {
    fn sur(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError>;
}

pub trait Releasable {
    fn rel(&self) -> Vec<Qualified>;
}

pub trait ScoreCompareble {
    fn cmp(
        &self,
        unless: bool,
        comparison: &str,
        scoreboard: &Scoreboard,
    ) -> Result<(Vec<Qualified>, Condition), MCAsmError>;
}

pub trait IntoSingleString {
    fn into_single_string(self) -> String;
}

impl<T> IntoSingleString for T
where
    T: Iterator<Item = Qualified>,
{
    fn into_single_string(self) -> String {
        self.map(|q| q.serialise())
            .collect::<Vec<String>>()
            .join("\n")
    }
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
    pub fn to_qualified(&self) -> Result<Vec<Qualified>, MCAsmError> {
        let cmp = |unless: bool,
                   comparison: &str,
                   lhs: &Scoreboard,
                   rhs: &Box<dyn ScoreCompareble + 'static>,
                   mnemonic: &Box<Mnemonic>|
         -> Result<Vec<Qualified>, MCAsmError> {
            let cmp_result = rhs.cmp(unless, comparison, lhs)?;
            let mut res = cmp_result.0;
            res.extend(
                mnemonic
                    .to_qualified()?
                    .into_iter()
                    .map(|mut q| {
                        q.push_condition(cmp_result.1.clone());
                        println!("{:?}", q);
                        q
                    })
                    .collect::<Vec<Qualified>>(),
            );
            println!("{:?}", res);
            Ok(res)
        };

        match self {
            Self::Def((score, assignable)) => assignable.assign(score),
            Self::Mov((score, assignable)) => assignable.assign(score),

            Self::Add((score, source)) => source.add(score),
            Self::Sub((score, source)) => source.sub(score),
            Self::Mul((score, source)) => source.mul(score),
            Self::Div((score, source)) => source.div(score),
            Self::Sur((score, source)) => source.sur(score),

            Self::Nts((score, storage, path, magnif)) => Ok(vec![Qualified::from(
                storage.store_to_score(score, &path.path, *magnif),
            )]),
            Self::Stn((storage, path, score, magnif)) => Ok(vec![Qualified::from(
                score.storage_to_score(storage, &path.type_annotation, &path.path, *magnif),
            )]),

            Self::Rel(releasable) => Ok(releasable.rel()),

            Self::Je((lhs, rhs, mnemonic)) => cmp(false, "=", lhs, rhs, mnemonic),
            Self::Jne((lhs, rhs, mnemonic)) => cmp(true, "=", lhs, rhs, mnemonic),
            Self::Jl((lhs, rhs, mnemonic)) => cmp(false, "<", lhs, rhs, mnemonic),
            Self::Jg((lhs, rhs, mnemonic)) => cmp(false, ">", lhs, rhs, mnemonic),
        }
    }
}
