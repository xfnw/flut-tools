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
    pub fn scramble(location: u32) -> (u32, u32) {
        let location = location.reverse_bits() >> 11;
        let inner = location % 3;
        let location = location / 3;
        (location, inner)
    }
}
