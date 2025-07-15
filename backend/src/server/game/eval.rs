use std::{fmt::Debug};
use serde::Deserialize;

/// enum that represents the calculation to perform for 2 values
#[derive(Debug, Clone, Deserialize)]
pub enum EvalOp { Add, Sub, Mul, PowB, PowA }

impl EvalOp {
    pub fn eval<T:
        std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        // needed for pow (ensure that a and b are u32 to be able to call pow)
        + Copy
        + Into<u32>
        + From<u32>
        >(&self, a: T, b: T) -> T {
        use EvalOp::*;

        match *self {
            Add => { a + b }
            Sub => { a - b }
            Mul => { a * b }
            PowB => {
                let a_u32: u32 = a.into();
                let b_u32: u32 = b.into();
                let result = a_u32.pow(b_u32);
                T::from(result)
            }
            PowA => {
                let a_u32: u32 = a.into();
                let b_u32: u32 = b.into();
                let result = b_u32.pow(a_u32);
                T::from(result)
            }
        }
    }
}