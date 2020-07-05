use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Interval {
    I1m,
    I5m,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Interval::I1m => "1m",
                Interval::I5m => "5m",
            }
        )
    }
}
