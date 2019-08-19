//! This module contains test helper functions.

use std::borrow::Borrow;

use crate::expression::BooleanExpression;
use crate::field::Field;
use crate::wire_values::WireValues;
use num::BigUint;

pub fn assert_eq_true<F, T>(x: T, values: &WireValues<F>)
    where F: Field, T: Borrow<BooleanExpression<F>> {
    assert_eq!(true, x.borrow().evaluate(values));
}

pub fn assert_eq_false<F, T>(x: T, values: &WireValues<F>)
    where F: Field, T: Borrow<BooleanExpression<F>> {
    assert_eq!(false, x.borrow().evaluate(values));
}

#[derive(Debug)]
pub struct F7 {}

impl Field for F7 {
    fn order() -> BigUint {
        BigUint::from(7u8)
    }
}

#[derive(Debug)]
pub struct F11 {}

impl Field for F11 {
    fn order() -> BigUint {
        BigUint::from(11u8)
    }
}