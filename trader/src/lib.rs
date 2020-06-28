mod trader;
mod indicator;
mod investor;

pub use trader::*;
pub use indicator::*;
pub use investor::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
