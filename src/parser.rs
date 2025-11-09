use crate::types::*;

fn score_and_score_or_integer(
    splitten_at_1: Option<&&str>,
    splitten_at_2: Option<&&str>,
    score_score_closure: fn(Scoreboard, Scoreboard) -> Mnemonic,
    score_int_closure: fn(Scoreboard, i64) -> Mnemonic,
) -> Result<Mnemonic, MCAsmError> {
    if let (Some(operand1), Some(operand2)) = (splitten_at_1, splitten_at_2) {
        match (
            Scoreboard::try_from(operand1),
            Scoreboard::try_from(operand2),
            operand2.parse::<i64>(),
        ) {
            (Ok(score1), _, Ok(int_literal)) => Ok(score_int_closure(score1, int_literal)),
            (Ok(score1), Ok(score2), Err(_)) => Ok(score_score_closure(score1, score2)),
            _ => Err(MCAsmError::InvalidOperand),
        }
    } else {
        Err(MCAsmError::TooFewOperand)
    }
}

fn nbtstorage_to_score(
    operands: (Option<&&str>, Option<&&str>, Option<&&str>, Option<&&str>),
) -> Result<Mnemonic, MCAsmError> {
    if let (Some(s1), Some(s2), Some(s3), Some(s4)) = operands {
        let score = Scoreboard::try_from(s1)?;
        let storage = Storage::try_from(s2)?;
        let path = Path::try_from(s3)?;
        let magnif = s4
            .parse::<u32>()
            .or(Err(MCAsmError::CantImplicateAsUnsignedInteger))?;

        Ok(Mnemonic::Nts((score, storage, path, magnif)))
    } else {
        Err(MCAsmError::TooFewOperand)
    }
}

fn score_to_nbtstorage(
    operands: (Option<&&str>, Option<&&str>, Option<&&str>, Option<&&str>),
) -> Result<Mnemonic, MCAsmError> {
    if let (Some(s1), Some(s2), Some(s3), Some(s4)) = operands {
        let storage = Storage::try_from(s1)?;
        let path = Path::try_from(s2)?;
        let score = Scoreboard::try_from(s3)?;
        let magnif = s4
            .parse::<u32>()
            .or(Err(MCAsmError::CantImplicateAsUnsignedInteger))?;
        Ok(Mnemonic::Stn((storage, path, score, magnif)))
    } else {
        Err(MCAsmError::TooFewOperand)
    }
}

fn release(operand: Option<&&str>) -> Result<Mnemonic, MCAsmError> {
    if let Some(s1) = operand {
        Ok(Mnemonic::Rel(Box::new(Scoreboard::try_from(s1)?)))
    } else {
        Err(MCAsmError::TooFewOperand)
    }
}

pub fn parse_line(line: &str) -> Result<Mnemonic, MCAsmError> {
    let splitten: Vec<&str> = line.split_whitespace().collect();
    let mnemonic = match splitten.get(0) {
        Some(s) => s,
        None => return Err(MCAsmError::EmptyLineGiven),
    };
    macro_rules! score_source_mnemonic {
        ($x:path) => {
            score_and_score_or_integer(
                splitten.get(1),
                splitten.get(2),
                |a: Scoreboard, b: Scoreboard| -> Mnemonic { $x((a, Box::new(b))) },
                |a: Scoreboard, b: i64| -> Mnemonic { $x((a, Box::new(IntLiteral::from(b)))) },
            )
        };
    }
    match *mnemonic {
        "DEF" => score_source_mnemonic!(Mnemonic::Def),
        "MOV" => score_source_mnemonic!(Mnemonic::Mov),
        "ADD" => score_source_mnemonic!(Mnemonic::Add),
        "SUB" => score_source_mnemonic!(Mnemonic::Sub),
        "MUL" => score_source_mnemonic!(Mnemonic::Mul),
        "DIV" => score_source_mnemonic!(Mnemonic::Div),
        "SUR" => score_source_mnemonic!(Mnemonic::Sur),
        "NTS" => nbtstorage_to_score((
            splitten.get(1),
            splitten.get(2),
            splitten.get(3),
            splitten.get(4),
        )),
        "STN" => score_to_nbtstorage((
            splitten.get(1),
            splitten.get(2),
            splitten.get(3),
            splitten.get(4),
        )),
        "REL" => release(splitten.get(1)),
        _ => Err(MCAsmError::UnknownMnemonic),
    }
}

pub fn parse(mcassembly: &str) -> Result<Vec<Mnemonic>, Vec<(usize, MCAsmError)>> {
    let mut mnemonics = Vec::new();
    let mut errors = Vec::new();

    for (index, line) in mcassembly
        .lines()
        .map(|s| s.trim())
        .enumerate()
        .filter(|(_, s)| !s.is_empty())
        .filter(|(_, s)| !s.starts_with("//"))
    {
        match parse_line(line) {
            Ok(o) => mnemonics.push(o),
            Err(e) => errors.push((index, e)),
        }
    }

    if errors.is_empty() {
        Ok(mnemonics)
    } else {
        Err(errors)
    }
}
