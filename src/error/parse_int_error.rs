use thiserror::Error;
use std::num;

#[derive(Debug, Error, PartialEq)]
#[error("`num::ParseIntError`: {}", 0)]
pub struct ParseIntError(#[from] num::ParseIntError);

// pub struct ParseIntError(num::ParseIntError);

// impl PartialEq for ParseIntError {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.0.kind() == rhs.0.kind()
//     }
// }
