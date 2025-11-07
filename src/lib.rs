mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use crate::types::Mnemonic;

    use super::*;

    #[test]
    fn it_works() {
        // A = (B + C) * 3
        let a = Scoreboard::new("A", "test");
        let b = Scoreboard::new("B", "test");
        let c = Scoreboard::new("C", "test");

        const IDEAL_RESULT: &str = "scoreboard players operation A test = A test\nscoreboard players operation A test += A test\nscoreboard players set MC_ASM LITERAL_SCORE_CONVERSION 3\nscoreboard players operation A test *= MC_ASM LITERAL_SCORE_CONVERSION";

        assert_eq!(
            [
                Mnemonic::Mov((a.clone(), Box::new(b))),
                Mnemonic::Add((a.clone(), Box::new(c))),
                Mnemonic::Mul((a, Box::new(IntLiteral::from(3))))
            ]
            .iter()
            .map(|opecode| opecode.to_string().unwrap())
            .collect::<Vec<String>>()
            .join("\n"),
            IDEAL_RESULT.to_string()
        );
    }
}
