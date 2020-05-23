use fixed::types::{U1F7, U1F15};
use fixed::traits::{LossyFrom};
use half::prelude::*;

/** 
 * This is an implementation of https://xkcd.com/927/ for 
 * converting number-like values to/from f32.
 * 
 * Many different types behave like floating point numbers,
 * but they don't support all the operators that f32 has,
 * and their conversion methods are different. Due to 
 * ophanage rules, we can't just implement one of the 
 * exisitng conversion traits for all those types - we
 * have to invent our own.
*/
pub trait ConvertF32 {
    fn make_into_f32(self) -> f32;
    fn make_from_f32(value: f32) -> Self;
}

/** Trivial "conversion" from f32 to f32. */
impl ConvertF32 for f32 {
    fn make_into_f32(self) -> f32 {
        return self;
    }

    fn make_from_f32(value: f32) -> Self {
        return value;
    }
}

impl ConvertF32 for U1F7 {
    fn make_into_f32(self) -> f32 {
        return f32::lossy_from(self);
    }

    fn make_from_f32(value: f32) -> Self {
        return U1F7::from_num(value);
    }
}

impl ConvertF32 for U1F15 {
    fn make_into_f32(self) -> f32 {
        return f32::lossy_from(self);
    }

    fn make_from_f32(value: f32) -> Self {
        return U1F15::from_num(value);
    }
}

impl ConvertF32 for f16 {
    fn make_into_f32(self) -> f32 {
        return self.to_f32();
    }

    fn make_from_f32(value: f32) -> Self {
        return f16::from_f32(value);
    }
}

impl ConvertF32 for i8 {
    fn make_into_f32(self) -> f32 {
        return self.into();
    }

    fn make_from_f32(value: f32) -> Self {
        return value as i8;
    }
}