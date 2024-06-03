use std::{
    env::args,
    io::{stdin, stdout, Read, Write},
    sync::Arc,
};
use tokio::net::TcpStream;

use crate::primative::MemorySlab;

pub mod primative;
pub mod protocol;

#[tokio::main]
async fn main() {
    let argv: Vec<_> = args().skip(1).collect();
    if !(2..=3).contains(&argv.len()) {
        println!("usage: host offset [length]");
        return;
    }

    let stream = TcpStream::connect(argv[0].as_str()).await.unwrap();
    let slab = Arc::new(MemorySlab::new(stream));
    let newslab = slab.clone();
    tokio::spawn(async move {
        newslab.start().await;
    });

    if argv.len() == 3 {
        let mut offset = argv[1].parse().unwrap();
        let mut length = argv[2].parse().unwrap();
        let mut out = stdout();

        while length > 1024 {
            out.write_all(slab.get(offset, 1024).await.unwrap().as_ref())
                .unwrap();
            length -= 1024;
            offset += 1024;
        }

        out.write_all(slab.get(offset, length).await.unwrap().as_ref())
            .unwrap();
        return;
    }

    let mut buf = [0_u8; 1024];
    let mut offset = argv[1].parse().unwrap();
    let mut inp = stdin();

    while let Ok(len) = inp.read(&mut buf) {
        if len == 0 {
            break;
        }

        slab.set(offset, &buf[..len]).await.unwrap();
        offset += len as u32;
    }
}
