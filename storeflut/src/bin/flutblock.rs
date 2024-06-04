use async_trait::async_trait;
use nbd_async::{serve_local_nbd, BlockDevice};
use std::env::args;
use std::io::Result;
use std::sync::Arc;
use storeflut::primative::MemorySlab;
use tokio::net::TcpStream;

struct FlutDev {
    slab: Arc<MemorySlab>,
}

#[async_trait(?Send)]
impl BlockDevice for FlutDev {
    async fn read(&mut self, offset: u64, buf: &mut [u8]) -> Result<()> {
        let offset = offset as usize;
        let mut myoffset = 0;
        let mut length = buf.len();
        while length > 1024 {
            let data = self
                .slab
                .get((offset + myoffset) as u32, 1024)
                .await
                .unwrap();
            buf[myoffset..myoffset + 1024].copy_from_slice(data.as_slice());
            myoffset += 1024;
            length -= 1024;
        }
        let data = self
            .slab
            .get((offset + myoffset) as u32, length as u32)
            .await
            .unwrap();
        buf[myoffset..length + myoffset].copy_from_slice(data.as_slice());
        Ok(())
    }
    async fn write(&mut self, offset: u64, buf: &[u8]) -> Result<()> {
        let offset = offset as usize;
        let mut myoffset = 0;
        let mut length = buf.len();
        while length > 1024 {
            self.slab
                .set((offset + myoffset) as u32, &buf[myoffset..myoffset + 1024])
                .await
                .unwrap();
            myoffset += 1024;
            length -= 1024;
        }
        self.slab
            .set(
                (offset + myoffset) as u32,
                &buf[myoffset..myoffset + length],
            )
            .await
            .unwrap();
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let argv: Vec<_> = args().skip(1).collect();
    if argv.len() != 2 {
        println!("usage: host nbdpath");
        return;
    }
    let host = argv[0].as_str();
    let path = argv[1].as_str();
    let stream = TcpStream::connect(host).await.unwrap();
    let slab = Arc::new(MemorySlab::new(stream));
    let newslab = slab.clone();
    tokio::spawn(async move {
        newslab.start().await;
    });
    let newslab = slab.clone();
    tokio::spawn(async move {
        newslab.keepalive().await;
    });
    let dev = FlutDev { slab };
    serve_local_nbd(path, 512, 4096, false, dev).await.unwrap();
}
