use crate::types::{opecode::ScoreCompareble, scoreboard::LSC, *};

#[derive(Clone)]
pub struct IntLiteral {
    data: i64,
}

impl From<i64> for IntLiteral {
    fn from(value: i64) -> Self {
        Self { data: value }
    }
}

impl ScoreAssignable for IntLiteral {
    fn assign(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError> {
        if self.data < (i32::MAX as i64) {
            Ok(vec![Qualified::from(scoreboard.set(self.data as i32))])
        } else {
            Err(MCAsmError::NarrowingConversion)
        }
    }
}

impl ScoreAddable for IntLiteral {
    fn add(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError> {
        Ok(vec![Qualified::from(scoreboard.add(self.data as i32))])
    }
}

impl ScoreSubtractable for IntLiteral {
    fn sub(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError> {
        Ok(vec![Qualified::from(scoreboard.remove(self.data as i32))])
    }
}

impl ScoreMultiplicatable for IntLiteral {
    fn mul(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError> {
        Ok(vec![
            Qualified::from(LSC.set(self.data as i32)),
            Qualified::from(scoreboard.operate("*=", &LSC)),
        ])
    }
}

impl ScoreDividable for IntLiteral {
    fn div(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError> {
        Ok(vec![
            Qualified::from(LSC.set(self.data as i32)),
            Qualified::from(scoreboard.operate("/=", &LSC)),
        ])
    }
}

impl ScoreSurplusable for IntLiteral {
    fn sur(&self, scoreboard: &Scoreboard) -> Result<Vec<Qualified>, MCAsmError> {
        Ok(vec![
            Qualified::from(LSC.set(self.data as i32)),
            Qualified::from(scoreboard.operate("%=", &LSC)),
        ])
    }
}

impl ScoreCompareble for IntLiteral {
    fn cmp(
        &self,
        unless: bool,
        comparison: &str,
        scoreboard: &Scoreboard,
    ) -> Result<(Vec<Qualified>, Condition), MCAsmError> {
        Ok((
            vec![Qualified::from(LSC.set(self.data as i32))],
            scoreboard.compare(unless, comparison, &*LSC),
        ))
    }
}
