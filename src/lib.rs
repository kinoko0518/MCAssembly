mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use crate::types::{Multiplicatable, Scoreboard};

    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            IntLiteral::from(64)
                .mul(&Scoreboard::new("test", "test"))
                .unwrap()
        );
    }
}
