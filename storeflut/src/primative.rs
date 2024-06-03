use std::fmt::{self, Write};
use std::str::FromStr;

/// abstraction for a slab of memory mapped out of order onto a pixelflut server
///
/// it stores 2 MiB of data. to simplify implementation, and since most pixelflut servers
/// are larger than this, it is hardcoded to use the top left roughly 1024x683 region
///
/// does not check bounds, all operations will wrap around after 2^21 bytes
#[derive(Debug)]
pub struct MemorySlab {}

impl MemorySlab {
    pub fn new() -> MemorySlab {
        MemorySlab {}
    }
    pub fn scramble(location: u32) -> (u32, u32) {
        let location = location.reverse_bits() >> 11;
        let inner = location % 3;
        let location = location / 3;
        (location, inner)
    }
}

/// a pixelflut protocol line
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Line {
    PX(PXLine),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum LineError {
    UnknownCommand,
    BadX,
    BadY,
    BadColor,
}

impl FromStr for Line {
    type Err = LineError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use Line::*;
        use LineError::*;

        let mut values = value.split(&[' ', '\n']);
        match values.next() {
            Some("PX") => {
                let x = values.next().ok_or(BadX)?;
                let x: u32 = x.parse().or(Err(BadX))?;
                let y = values.next().ok_or(BadY)?;
                let y: u32 = y.parse().or(Err(BadY))?;
                let color = values.next().ok_or(BadColor)?;
                let color = u32::from_str_radix(color, 16).or(Err(BadColor))?;

                Ok(PX(PXLine { x, y, color }))
            }
            _ => Err(UnknownCommand),
        }
    }
}

/// a pixelflut protocol PX line
#[derive(Debug, Clone, Copy)]
pub struct PXLine {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

impl fmt::Display for PXLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PX ")?;
        self.x.fmt(f)?;
        f.write_char(' ')?;
        self.y.fmt(f)?;
        f.write_char(' ')?;
        write!(f, "{:06x}", self.color)?;
        f.write_char('\n')
    }
}
