use std::marker::Unpin;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader, Lines, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::{
        broadcast::{self, Sender},
        Mutex,
    },
};

use crate::protocol::Line;

/// abstraction for a slab of memory mapped out of order onto a pixelflut server
///
/// it stores 2 MiB of data. to simplify implementation, and since most pixelflut servers
/// are larger than this, it is hardcoded to use the top left roughly 1024x682 region
///
/// does not check bounds, all operations will wrap around after 2^21 bytes
#[derive(Debug)]
pub struct MemorySlab {
    read: Lines<BufReader<ReadHalf<TcpStream>>>,
    write: WriteHalf<TcpStream>,
    broadcast: Sender<(u32, u32)>,
}

impl MemorySlab {
    pub async fn new(stream: TcpStream) -> MemorySlab {
        let (read, write) = io::split(stream);
        let read = BufReader::new(read).lines();
        let (broadcast, _) = broadcast::channel(1024);
        MemorySlab {
            read,
            write,
            broadcast,
        }
    }
    pub async fn start(&mut self) {
        while let Some(line) = self.read.next_line().await.unwrap() {
            match line.parse() {
                Ok(Line::PX(line)) => {
                    println!("i got a {:?}", line);
		    let loc = coord_to_num(line.x, line.y);
		    self.broadcast.send((loc, line.color)).unwrap();
                }
                Ok(_) => (),
                Err(e) => println!("oh no {:?}", e),
            }
        }
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
    #[test]
    fn coords_reversible() {
        use crate::primative::*;
        assert_eq!(num_to_coord(coord_to_num(621, 926)), (621, 926))
    }
}
