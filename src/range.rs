use crate::error::{Error, ParseRangeError, Result};
use std::str::FromStr;

#[derive(Eq, Debug)]
pub struct Range {
    pub start: Index,
    pub end: Index,
    pub step: usize,
    reversed: bool,
}

impl Range {
    /// Create a new range from start, end and step.
    /// Filter out empty ranges.
    fn new(start: Index, end: Index, step: isize) -> Option<Self> {
        match (&start, &end) {
            (Index::Head(i), Index::Head(j)) if i >= j => Option::None,
            (Index::Tail(i), Index::Tail(j)) if i <= j => Option::None,
            _ => match step {
                0 => Option::None,
                i if i > 0 => Some(Self {
                    start,
                    end,
                    step: step as usize,
                    reversed: false,
                }),
                _ => Some(Self {
                    start,
                    end,
                    step: step.unsigned_abs(),
                    reversed: true,
                }),
            },
        }
    }

    pub fn is_reversed(&self) -> bool {
        self.reversed
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.step == 0 && other.step == 0
            || (self.start == other.start
                && self.end == other.end
                && self.step == other.step
                && self.reversed == other.reversed)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Index {
    Head(usize),
    Tail(usize),
}

impl Index {
    pub fn new(index: isize) -> Self {
        match index {
            i if i >= 0 => Index::Head(i as usize),
            i => Index::Tail(i.unsigned_abs()),
        }
    }
}

impl FromStr for Index {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Index::Head(0)),
            "-" => Ok(Index::Tail(0)),
            // _ => s.parse().map(Index::new).map_err(|e| {
            //     log::error!("{}", e);
            //     Error::ParseRangeError(ParseRangeError)
            // }),
            _ => s
                .parse()
                .map(Index::new)
                .map_err(|e| Error::ParseIntError(e)),
        }
    }
}

fn _parse(slice: &str) -> Result<Option<Range>> {
    let mut range = slice.split(':');

    let start = range.next().unwrap_or("+").parse().map_err(|_| {
        log::error!("Invalid range at parsing start of {}", slice);
        Error::ParseRangeError(ParseRangeError)
    })?;
    let end = range.next().unwrap_or("-").parse().map_err(|_| {
        log::error!("Invalid range at parsing end of {}", slice);
        Error::ParseRangeError(ParseRangeError)
    })?;
    let step = range.next().unwrap_or("1").parse().map_err(|_| {
        log::error!("Invalid range at parsing step of {}", slice);
        Error::ParseRangeError(ParseRangeError)
    })?;

    Ok(Range::new(start, end, step))
}

pub fn parse(slices: Vec<String>) -> Result<Vec<Range>> {
    let mut ranges = Vec::new();

    for slice in slices {
        match _parse(&slice) {
            Ok(None) => {}
            Ok(Some(range)) => ranges.push(range),
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(ranges)
}
