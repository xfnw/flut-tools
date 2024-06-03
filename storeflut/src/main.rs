use primative::MemorySlab;
use std::sync::Arc;
use tokio::net::TcpStream;

pub mod primative;
pub mod protocol;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("feesh:1234").await.unwrap();
    let slab = Arc::new(MemorySlab::new(stream));
    let newslab = slab.clone();
    tokio::spawn(async move {
        newslab.start().await;
    });
    println!("{:?}", slab.get_pixel(1).await);
    for i in 100000..100255 {
	slab.set_byte(i, i as u8).await.unwrap();
    }
}
