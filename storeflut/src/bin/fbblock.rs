use async_trait::async_trait;
use memmap::MmapMut;
use nbd_async::{serve_local_nbd, BlockDevice};
use std::env::args;
use std::io::Result;

struct FbDev {
    data: MmapMut,
}

#[async_trait(?Send)]
impl BlockDevice for FbDev {
    async fn read(&mut self, offset: u64, buf: &mut [u8]) -> Result<()> {
	let offset = offset as usize;
	buf.copy_from_slice(&self.data[offset..offset + buf.len()]);
        Ok(())
    }
    async fn write(&mut self, offset: u64, buf: &[u8]) -> Result<()> {
        let offset = offset as usize;
	self.data[offset..offset + buf.len()].copy_from_slice(buf);
	Ok(())
    }
}

#[tokio::main]
async fn main() {
    let argv: Vec<_> = args().skip(1).collect();
    if argv.len() != 2 {
        println!("usage: fbpath nbdpath");
        return;
    }

    let fbpath = argv[0].as_str();
    let nbdpath = argv[1].as_str();

    let fb = linuxfb::Framebuffer::new(fbpath).unwrap();
    let data = fb.map().unwrap();
    let size = data.len() as u64;

    // turn off cursor blinking in tty
    print!("\033[?17;127c");
    
    let dev = FbDev { data };
    serve_local_nbd(nbdpath, 512, size / 512, false, dev)
        .await
        .unwrap();
}
