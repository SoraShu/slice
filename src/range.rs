use std::str::FromStr;

#[derive(Eq, Debug)]
pub struct Range {
    pub start: Index,
    pub end: Index,
    pub step: usize,
    reversed: bool,
}

impl Range {
    fn new(start: Index, end: Index, step: isize) -> Self {
        match step {
            0 => Self {
                start: Index::Head(0),
                end: Index::Head(0),
                step: 0,
                reversed: false,
            },
            i if i > 0 => Self {
                start,
                end,
                step: step as usize,
                reversed: false,
            },
            _ => Self {
                start,
                end,
                step: step.unsigned_abs(),
                reversed: true,
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
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Index::Head(0)),
            "-" => Ok(Index::Tail(0)),
            _ => s
                .parse()
                .map(Index::new)
                .map_err(|_| "Could not parse index"),
        }
    }
}

fn _parse(slice: &str) -> Result<Range, &'static str> {
    let mut range = slice.split(':');

    let start = range
        .next()
        .unwrap_or("+")
        .parse()
        .map_err(|_| "Could not parse start")?;
    let end = range
        .next()
        .unwrap_or("-")
        .parse()
        .map_err(|_| "Could not parse end")?;
    let step = range
        .next()
        .unwrap_or("1")
        .parse()
        .map_err(|_| "Could not parse step")?;

    Ok(Range::new(start, end, step))
}

pub fn parse(slices: Vec<String>) -> Vec<Range> {
    let mut ranges = Vec::new();

    for slice in slices {
        ranges.push(match _parse(&slice) {
            Ok(range) => range,
            Err(e) => {
                panic!("Could not parse slice: {}", e)
            }
        });
    }

    ranges
}
