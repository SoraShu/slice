use crate::error::ParseRangeError;
use std::str::FromStr;

#[derive(Eq, Debug)]
pub struct Range {
    pub start: Index,
    pub end: Index,
    pub step: usize,
    reversed: bool,
}

impl Range {
    fn new(start: Index, end: Index, step: isize) -> Option<Self> {
        match step {
            0 => None,
            i if i > 0 => Some(Range {
                start,
                end,
                step: i as usize,
                reversed: false,
            }),
            i => Some(Range {
                start,
                end,
                step: i.unsigned_abs(),
                reversed: true,
            }),
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
    Empty,
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
    type Err = ParseRangeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "" => Ok(Index::Empty),
            _ => s.parse().map(Index::new).map_err(|_| ParseRangeError),
        }
    }
}

fn _parse(slice: &str) -> Result<Option<Range>, ParseRangeError> {
    let mut range = slice.split(':').map(|s| s.trim());

    let start = range
        .next()
        .unwrap_or("")
        .parse()
        .map_err(|_| ParseRangeError)?;
    let end = range
        .next()
        .unwrap_or("")
        .parse()
        .map_err(|_| ParseRangeError)?;
    let step = match range.next() {
        None => 1,
        Some("") => 1,
        Some(s) => s.parse().map_err(|_| ParseRangeError)?,
    };

    Ok(Range::new(start, end, step))
}

pub fn parse(slices: Vec<String>) -> Vec<Range> {
    let mut ranges = Vec::new();

    for slice in slices {
        match _parse(&slice) {
            Ok(Some(range)) => ranges.push(range),
            Ok(None) => {}
            Err(_) => {
                panic!("Could not parse slice: {}", &slice)
            }
        };
    }

    ranges
}
