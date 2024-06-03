use futures::stream::{FuturesOrdered, FuturesUnordered};
use futures::StreamExt;
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::{
        broadcast::{self, error::RecvError, Sender},
        Mutex,
    },
};

use crate::protocol::{Line, PXGetLine, PXSetLine};

#[non_exhaustive]
#[derive(Debug)]
pub enum MemorySlabError {
    RecvError(RecvError),
    IoError(std::io::Error),
}

/// abstraction for a slab of memory mapped out of order onto a pixelflut server
///
/// it stores 2 MiB of data. to simplify implementation, and since most pixelflut servers
/// are larger than this, it is hardcoded to use the top left roughly 1009x692 region
///
/// does not check bounds, all operations will wrap around after 2^21 bytes
#[derive(Debug)]
pub struct MemorySlab {
    read: Mutex<Lines<BufReader<ReadHalf<TcpStream>>>>,
    write: Mutex<WriteHalf<TcpStream>>,
    broadcast: Sender<(u32, u32)>,
}

impl MemorySlab {
    pub fn new(stream: TcpStream) -> MemorySlab {
        let (read, write) = io::split(stream);
        let read = Mutex::new(BufReader::new(read).lines());
        let write = Mutex::new(write);
        let (broadcast, _) = broadcast::channel(1024);
        MemorySlab {
            read,
            write,
            broadcast,
        }
    }
    pub async fn start(&self) {
        while let Some(line) = self.read.lock().await.next_line().await.unwrap() {
            match line.parse() {
                Ok(Line::PX(line)) => {
                    let loc = coord_to_num(line.x, line.y);
                    self.broadcast.send((loc, line.color)).unwrap();
                }
                Ok(_) => (),
                Err(e) => println!("oh no {:?}", e),
            }
        }
    }
    pub async fn wait_for(&self, offset: u32) -> Result<u32, MemorySlabError> {
        let mut rx = self.broadcast.subscribe();
        loop {
            let (rloc, color) = rx.recv().await.map_err(MemorySlabError::RecvError)?;
            if rloc == offset {
                return Ok(color);
            }
        }
    }
    pub async fn get_pixel(&self, offset: u32) -> Result<u32, MemorySlabError> {
        let (x, y) = num_to_coord(offset);
        let line = PXGetLine { x, y }.to_string();

        self.write
            .lock()
            .await
            .write_all(line.as_bytes())
            .await
            .map_err(MemorySlabError::IoError)?;

        self.wait_for(offset).await
    }
    pub async fn set_pixel(&self, offset: u32, color: u32) -> Result<(), MemorySlabError> {
        let (x, y) = num_to_coord(offset);
        let line = PXSetLine { x, y, color }.to_string();

        self.write
            .lock()
            .await
            .write_all(line.as_bytes())
            .await
            .map_err(MemorySlabError::IoError)
    }
    pub async fn get_byte(&self, location: u32) -> Result<u8, MemorySlabError> {
        let (offset, inner) = scramble(location);
        let pixel = self.get_pixel(offset).await?;
        let shifted = pixel >> (inner * 8);

        Ok(shifted as u8)
    }
    pub async fn set_byte(&self, location: u32, value: u8) -> Result<(), MemorySlabError> {
        let (offset, inner) = scramble(location);
        let oldpixel = self.get_pixel(offset).await?;
        let mask: u32 = !(((1 << 8) - 1) << (inner * 8));
        let shifted = (value as u32) << (inner * 8);
        let newcolor = oldpixel & mask | shifted;

        self.set_pixel(offset, newcolor).await
    }
    pub async fn get(&self, offset: u32, length: u32) -> Result<Vec<u8>, MemorySlabError> {
        let stream: FuturesOrdered<_> = (offset..offset + length)
            .map(|o| self.get_byte(o))
            .collect();

        stream
            .collect::<Vec<Result<u8, MemorySlabError>>>()
            .await
            .into_iter()
            .collect()
    }
    pub async fn set(&self, offset: u32, data: &[u8]) -> Result<(), MemorySlabError> {
        let mut stream: FuturesUnordered<_> = data
            .iter()
            .enumerate()
            .map(|(i, v)| self.set_byte(offset + i as u32, *v))
            .collect();

        while let Some(res) = stream.next().await {
            res?;
        }
        Ok(())
    }
}

pub fn scramble(location: u32) -> (u32, u32) {
    let location = location.reverse_bits() >> 11;
    let inner = location % 3;
    let location = location / 3;
    (location, inner)
}

pub fn num_to_coord(location: u32) -> (u32, u32) {
    (location % 1009, location / 1009)
}

pub fn coord_to_num(x: u32, y: u32) -> u32 {
    y * 1009 + x
}

mod tests {
    #[test]
    fn coords_reversible() {
        use crate::primative::*;
        assert_eq!(num_to_coord(coord_to_num(621, 926)), (621, 926))
    }
}
