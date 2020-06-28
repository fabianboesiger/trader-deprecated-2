use crate::{Asset, Filter};

pub struct Market<'a> {
    pub base: &'a Asset,
    pub quote: &'a Asset,
    filters: Vec<Box<dyn Filter>>
}