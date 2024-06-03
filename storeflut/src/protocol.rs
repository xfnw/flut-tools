use std::fmt::{self, Write};
use std::str::FromStr;

/// a pixelflut protocol line
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Line {
    PX(PXSetLine),
    SIZE(SizeLine),
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

                Ok(PX(PXSetLine { x, y, color }))
            }
            Some("SIZE") => {
                let x = values.next().ok_or(BadX)?;
                let x: u32 = x.parse().or(Err(BadX))?;
                let y = values.next().ok_or(BadY)?;
                let y: u32 = y.parse().or(Err(BadY))?;

                Ok(SIZE(SizeLine { x, y }))
            }
            _ => Err(UnknownCommand),
        }
    }
}

/// a pixelflut PX line
#[derive(Debug, Clone, Copy)]
pub struct PXSetLine {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

impl fmt::Display for PXSetLine {
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

/// a pixelflut PX request line
#[derive(Debug, Clone, Copy)]
pub struct PXGetLine {
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for PXGetLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PX ")?;
        self.x.fmt(f)?;
        f.write_char(' ')?;
        self.y.fmt(f)?;
        f.write_char('\n')
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SizeLine {
    pub x: u32,
    pub y: u32,
}
