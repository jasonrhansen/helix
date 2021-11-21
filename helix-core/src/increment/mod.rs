pub mod date;
pub mod number;

use crate::{Range, Tendril};

pub trait Increment {
    fn increment(&self, amount: i64) -> (Range, Tendril);
}
