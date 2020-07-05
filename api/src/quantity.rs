use crate::{Asset, Monetary, Price};
use std::cmp::Ordering;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Quantity {
    pub quantity: Monetary,
    pub asset: &'static Asset,
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.asset, other.asset);

        Self {
            quantity: self.quantity + other.quantity,
            asset: self.asset,
        }
    }
}

impl Mul<Price> for Quantity {
    type Output = Self;

    fn mul(self, other: Price) -> Quantity {
        assert_eq!(self.asset, other.market.base);

        Self {
            quantity: self.quantity * other.price,
            asset: other.market.quote,
        }
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        self.quantity.eq(&other.quantity)
    }
}
