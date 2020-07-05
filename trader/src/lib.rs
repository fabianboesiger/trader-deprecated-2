mod indicator;
mod investor;
mod trader;

pub use indicator::*;
pub use investor::*;
pub use trader::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
