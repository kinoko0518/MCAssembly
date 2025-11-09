use once_cell::sync::Lazy;

use crate::{
    MCAsmError, Releasable, ScoreAddable, ScoreAssignable, ScoreDividable, ScoreMultiplicatable,
    ScoreSubtractable, ScoreSurplusable,
    types::storage::{Storage, StorageType},
};

/// Objective::Scoreholder in Assembly
#[derive(Clone)]
pub struct Scoreboard {
    pub scoreholder: String,
    pub objective: String,
}

/// A special scoreboard to be used to literal score conversion.
pub static LSC: Lazy<Scoreboard> = Lazy::new(|| Scoreboard {
    scoreholder: "LITERAL_SCORE_CONVERSION".into(),
    objective: "MC_ASM".into(),
});

impl Scoreboard {
    pub fn new(scoreholder: impl Into<String>, objective: impl Into<String>) -> Self {
        Self {
            scoreholder: scoreholder.into(),
            objective: objective.into(),
        }
    }
    pub fn try_from(from: &str) -> Result<Self, MCAsmError> {
        from.split_once("::")
            .map(|(objective, scoreholder)| Self {
                scoreholder: scoreholder.into(),
                objective: objective.into(),
            })
            .ok_or(MCAsmError::InvalidScoreboard)
    }
    /// Unsafe!
    pub fn set(&self, source: i32) -> String {
        format!(
            "scoreboard players set {} {} {}",
            self.scoreholder, self.objective, source
        )
    }
    /// Unsafe!
    pub fn add(&self, source: i32) -> String {
        format!(
            "scoreboard players add {} {} {}",
            self.scoreholder, self.objective, source
        )
    }
    /// Unsafe!
    pub fn remove(&self, source: i32) -> String {
        format!(
            "scoreboard players remove {} {} {}",
            self.scoreholder, self.objective, source
        )
    }
    /// Unsafe!
    pub fn free(&self) -> String {
        format!(
            "scoreboard players reset {} {}",
            self.scoreholder, self.objective
        )
    }
    /// Unsafe!
    pub fn operate(&self, operation: impl Into<String>, other: &Scoreboard) -> String {
        format!(
            "scoreboard players operation {} {} {} {} {}",
            self.scoreholder,
            self.objective,
            operation.into(),
            other.scoreholder,
            other.objective
        )
    }
    pub fn get(&self) -> String {
        format!(
            "scoreboard players get {} {}",
            self.scoreholder, self.objective
        )
    }
    pub fn storage_to_score(
        &self,
        storage: &Storage,
        ntb_type: &StorageType,
        path: &String,
        magnif: f32,
    ) -> String {
        format!(
            "execute store result storage {} {} {} {} run {}",
            storage.fullname(),
            path,
            ntb_type,
            magnif,
            self.get()
        )
    }
}

impl ScoreAssignable for Scoreboard {
    fn assign(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("=", self))
    }
}

impl ScoreAddable for Scoreboard {
    fn add(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("+=", self))
    }
}

impl ScoreSubtractable for Scoreboard {
    fn sub(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("-=", self))
    }
}

impl ScoreMultiplicatable for Scoreboard {
    fn mul(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("*=", self))
    }
}

impl ScoreDividable for Scoreboard {
    fn div(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("/=", self))
    }
}

impl ScoreSurplusable for Scoreboard {
    fn sur(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("%=", self))
    }
}

impl Releasable for Scoreboard {
    fn rel(&self) -> String {
        self.free()
    }
}
