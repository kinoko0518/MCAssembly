use once_cell::sync::Lazy;

use crate::{
    Addable, Assignable, Dividable, Multiplicatable, Releasable, Subtractable, Surplusable,
};

pub struct Scoreboard {
    pub scoreholder: String,
    pub objective: String,
}

/// A special scoreboard to be used to literal score conversion.
pub static LSC: Lazy<Scoreboard> = Lazy::new(|| Scoreboard {
    scoreholder: "MC_ASM".into(),
    objective: "LITERAL_SCORE_CONVERSION".into(),
});

/// A special scoreboard to be used as accumulator.
pub static ACM: Lazy<Scoreboard> = Lazy::new(|| Scoreboard {
    scoreholder: "MC_ASM".into(),
    objective: "ACCUMULATOR".into(),
});

impl Scoreboard {
    pub fn new(scoreholder: impl Into<String>, objective: impl Into<String>) -> Self {
        Self {
            scoreholder: scoreholder.into(),
            objective: objective.into(),
        }
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
}

impl Assignable for Scoreboard {
    fn assign(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("=", other))
    }
}

impl Addable for Scoreboard {
    fn add(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("+=", other))
    }
}

impl Subtractable for Scoreboard {
    fn sub(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("-=", other))
    }
}

impl Multiplicatable for Scoreboard {
    fn mul(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("*=", other))
    }
}

impl Dividable for Scoreboard {
    fn div(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("/=", other))
    }
}

impl Surplusable for Scoreboard {
    fn sur(&self, other: &Scoreboard) -> Result<String, super::MCAsmError> {
        Ok(other.operate("%=", other))
    }
}

impl Releasable for Scoreboard {
    fn rel(&self) -> String {
        self.free()
    }
}
