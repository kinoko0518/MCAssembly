mod float;
mod integer;

pub use float::FltLiteral;
pub use integer::IntLiteral;

struct StringLiteral {
    data: String,
}
