mod parser;
mod types;

pub use parser::{parse, parse_line};
pub use types::*;

#[cfg(test)]
mod tests {
    use crate::types::Mnemonic;

    use super::*;

    #[test]
    fn mnemonic_to_mcfunction_test() {
        // A = (B + C) * 3
        let a = Scoreboard::new("A", "mcasm");
        let b = Scoreboard::new("B", "mcasm");
        let c = Scoreboard::new("C", "mcasm");

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

    #[test]
    fn parse_test() {
        let source = "
            DEF mcasm::A mcasm::B
            ADD mcasm::A mcasm::C
            MUL mcasm::A 3

            REL mcasm::A

            NTS mcasm::D mcasm:some path.to.data[0]::<float> 1024
            STN mcasm:some path.to.data[1] mcasm::D 1
        ";
        println!(
            "{}",
            match parse(source) {
                Ok(o) => o
                    .iter()
                    .filter_map(|mnemonic| mnemonic.to_string().ok())
                    .collect::<Vec<String>>()
                    .join("\n"),
                Err(e) => e
                    .iter()
                    .map(|(index, error)| format!(
                        "An error occured at line {}: {:?}",
                        index, error
                    ))
                    .collect::<Vec<String>>()
                    .join("\n"),
            }
        );
    }
}
