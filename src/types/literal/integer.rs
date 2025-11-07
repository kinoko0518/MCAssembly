use crate::types::{scoreboard::LSC, *};

pub struct IntLiteral {
    data: i64,
}

impl From<i64> for IntLiteral {
    fn from(value: i64) -> Self {
        Self { data: value }
    }
}

impl Assignable for IntLiteral {
    fn assign(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError> {
        if self.data < (i32::MAX as i64) {
            Ok(scoreboard.set(self.data as i32))
        } else {
            Err(MCAsmError::NarrowingConversionError)
        }
    }
}

impl Addable for IntLiteral {
    fn add(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError> {
        Ok(scoreboard.add(self.data as i32))
    }
}

impl Subtractable for IntLiteral {
    fn sub(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError> {
        Ok(scoreboard.remove(self.data as i32))
    }
}

impl Multiplicatable for IntLiteral {
    fn mul(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError> {
        Ok(format!(
            "{}\n{}",
            LSC.set(self.data as i32),
            scoreboard.operate("*=", &LSC)
        ))
    }
}

impl Dividable for IntLiteral {
    fn div(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError> {
        Ok(format!(
            "{}\n{}",
            LSC.set(self.data as i32),
            scoreboard.operate("/=", &LSC)
        ))
    }
}

impl Surplusable for IntLiteral {
    fn sur(&self, scoreboard: &Scoreboard) -> Result<String, MCAsmError> {
        Ok(format!(
            "{}\n{}",
            LSC.set(self.data as i32),
            scoreboard.operate("%=", &LSC)
        ))
    }
}
