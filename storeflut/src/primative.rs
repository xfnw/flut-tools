/// abstraction for a slab of memory mapped out of order onto a pixelflut server
///
/// it stores 2 MiB of data. to simplify implementation, and since most pixelflut servers
/// are larger than this, it is hardcoded to use the top left roughly 1024x682 region
///
/// does not check bounds, all operations will wrap around after 2^21 bytes
#[derive(Debug)]
pub struct MemorySlab {}

impl MemorySlab {
    pub fn new() -> MemorySlab {
        MemorySlab {}
    }
}

pub fn scramble(location: u32) -> (u32, u32) {
    let location = location.reverse_bits() >> 11;
    let inner = location % 3;
    let location = location / 3;
    (location, inner)
}

pub fn num_to_coord(location: u32) -> (u32, u32) {
    (location % 1024, location / 1024)
}

pub fn coord_to_num(x: u32, y: u32) -> u32 {
    y * 1024 + x
}

mod tests {
    use crate::primative::*;

    #[test]
    fn max_scramble() {
    }

    #[test]
    fn coords_reversible() {
        assert_eq!(num_to_coord(coord_to_num(621, 926)), (621, 926))
    }
}
