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
        let expected_result:String = "
            scoreboard players operation #A mcasm = #B mcasm
            scoreboard players operation #A mcasm += #C mcasm
            scoreboard players set LITERAL_SCORE_CONVERSION MC_ASM 3
            scoreboard players operation #A mcasm *= LITERAL_SCORE_CONVERSION MC_ASM
            scoreboard players reset #A mcasm
            execute store result score #D mcasm run data get mcasm:some path.to.data[0] 1024
            execute store result storage mcasm:some path.to.data[1] int 1 run scoreboard players get #D mcasm
        ".lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().join("\n");
        let source = "
            DEF mcasm::#A mcasm::#B
            ADD mcasm::#A mcasm::#C
            MUL mcasm::#A 3

            REL mcasm::#A

            NTS mcasm::#D mcasm:some path.to.data[0]::<float> 1024
            STN mcasm:some path.to.data[1] mcasm::#D 1
        ";
        let stringfied = parse(source)
            .unwrap()
            .iter()
            .map(|mnemonic| mnemonic.to_string().unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        assert_eq!(stringfied, expected_result);
    }
}
