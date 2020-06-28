use crate::{Asset, Monetary, Price};
use std::ops::{Add, Mul};

pub struct Quantity<'a> {
    quantity: Monetary,
    asset: &'a Asset,
}

impl<'a> Add for Quantity<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.asset, other.asset);

        Self {
            quantity: self.quantity + other.quantity,
            asset: self.asset,
        }
    }
}

impl<'a> Mul<Price<'a>> for Quantity<'a> {
    type Output = Self;

    fn mul(self, other: Price<'a>) -> Quantity<'a> {
        assert_eq!(self.asset, other.market.base);
        
        Self {
            quantity: self.quantity * other.price,
            asset: other.market.quote,
        }
    }
}

